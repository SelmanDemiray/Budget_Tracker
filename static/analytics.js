let dailyChart = null;
let popularChart = null;

// Load analytics data when page loads
document.addEventListener('DOMContentLoaded', () => {
    loadAnalytics();
    
    // Auto-refresh every 30 seconds
    setInterval(loadAnalytics, 30000);
});

async function loadAnalytics() {
    try {
        const response = await fetch('/api/analytics/dashboard', {
            credentials: 'include'
        });
        
        if (!response.ok) {
            throw new Error('Failed to load analytics');
        }
        
        const data = await response.json();
        updateDashboard(data);
    } catch (error) {
        console.error('Error loading analytics:', error);
    }
}

function updateDashboard(data) {
    // Update key metrics
    document.getElementById('totalUsers').textContent = data.total_users.toLocaleString();
    document.getElementById('activeSessions').textContent = data.active_sessions.toLocaleString();
    document.getElementById('pageViewsToday').textContent = data.page_views_today.toLocaleString();
    
    // Calculate average session time
    const avgTime = data.user_activity.length > 0 
        ? Math.round(data.user_activity.reduce((sum, user) => sum + user.total_time_minutes, 0) / data.user_activity.length)
        : 0;
    document.getElementById('avgSessionTime').textContent = `${avgTime}m`;
    
    // Update charts
    updateDailyViewsChart(data.daily_stats);
    updatePopularPagesChart(data.popular_pages);
    
    // Update tables
    updateUserActivityTable(data.user_activity);
    updateRecentEventsTable(data.recent_events);
}

function updateDailyViewsChart(dailyStats) {
    const ctx = document.getElementById('dailyViewsChart').getContext('2d');
    
    if (dailyChart) {
        dailyChart.destroy();
    }
    
    const labels = dailyStats.map(stat => new Date(stat.date).toLocaleDateString());
    const pageViews = dailyStats.map(stat => stat.page_views);
    const uniqueUsers = dailyStats.map(stat => stat.unique_users);
    
    dailyChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels.reverse(),
            datasets: [
                {
                    label: 'Page Views',
                    data: pageViews.reverse(),
                    borderColor: '#667eea',
                    backgroundColor: 'rgba(102, 126, 234, 0.1)',
                    tension: 0.4
                },
                {
                    label: 'Unique Users',
                    data: uniqueUsers.reverse(),
                    borderColor: '#28a745',
                    backgroundColor: 'rgba(40, 167, 69, 0.1)',
                    tension: 0.4
                }
            ]
        },
        options: {
            responsive: true,
            plugins: {
                legend: {
                    position: 'top',
                }
            },
            scales: {
                y: {
                    beginAtZero: true
                }
            }
        }
    });
}

function updatePopularPagesChart(popularPages) {
    const ctx = document.getElementById('popularPagesChart').getContext('2d');
    
    if (popularChart) {
        popularChart.destroy();
    }
    
    const labels = popularPages.slice(0, 8).map(page => page.path);
    const views = popularPages.slice(0, 8).map(page => page.view_count);
    
    popularChart = new Chart(ctx, {
        type: 'bar',
        data: {
            labels: labels,
            datasets: [{
                label: 'Page Views',
                data: views,
                backgroundColor: 'rgba(102, 126, 234, 0.8)',
                borderColor: '#667eea',
                borderWidth: 1
            }]
        },
        options: {
            responsive: true,
            plugins: {
                legend: {
                    display: false
                }
            },
            scales: {
                y: {
                    beginAtZero: true
                },
                x: {
                    ticks: {
                        maxRotation: 45,
                        minRotation: 45
                    }
                }
            }
        }
    });
}

function updateUserActivityTable(userActivity) {
    const tableContent = document.getElementById('userActivityTable');
    const headerRow = tableContent.firstElementChild;
    
    // Clear existing rows except header
    tableContent.innerHTML = '';
    tableContent.appendChild(headerRow);
    
    userActivity.forEach(user => {
        const row = document.createElement('div');
        row.className = 'table-row';
        
        const userName = user.user_name || 'Anonymous';
        const lastActivity = new Date(user.last_activity).toLocaleString();
        const isRecent = new Date() - new Date(user.last_activity) < 30 * 60 * 1000; // 30 minutes
        
        row.innerHTML = `
            <div>
                <span class="status-indicator ${isRecent ? 'status-active' : 'status-inactive'}"></span>
                ${userName}
            </div>
            <div>${lastActivity}</div>
            <div>${user.session_count}</div>
            <div>${user.page_views}</div>
            <div>${user.total_time_minutes}</div>
        `;
        
        tableContent.appendChild(row);
    });
}

function updateRecentEventsTable(recentEvents) {
    const tableContent = document.getElementById('recentEventsTable');
    const headerRow = tableContent.firstElementChild;
    
    // Clear existing rows except header
    tableContent.innerHTML = '';
    tableContent.appendChild(headerRow);
    
    recentEvents.slice(0, 20).forEach(event => {
        const row = document.createElement('div');
        row.className = 'table-row';
        
        const eventTime = new Date(event.created_at).toLocaleString();
        const eventData = event.event_data ? JSON.stringify(event.event_data, null, 2) : 'N/A';
        
        row.innerHTML = `
            <div><strong>${event.event_type}</strong></div>
            <div>${event.user_id ? 'User' : 'Anonymous'}</div>
            <div>${eventTime}</div>
            <div><pre style="font-size: 0.8rem; max-width: 200px; overflow: hidden;">${eventData}</pre></div>
        `;
        
        tableContent.appendChild(row);
    });
}

// Utility function to format time
function formatDuration(seconds) {
    if (seconds < 60) return `${seconds}s`;
    const minutes = Math.floor(seconds / 60);
    if (minutes < 60) return `${minutes}m`;
    const hours = Math.floor(minutes / 60);
    return `${hours}h ${minutes % 60}m`;
}
