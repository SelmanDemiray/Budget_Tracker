let currentUser = null;
let budgetData = [];
let categories = [];

// Category management variables
let allCategories = [];
let filteredCategories = [];
let collapsedCategories = new Set();

// Initialize app
document.addEventListener('DOMContentLoaded', async () => {
    await loadCategories();
    await checkAuth();
    
    // Set current year as default
    const yearSelect = document.getElementById('yearSelect');
    yearSelect.value = new Date().getFullYear().toString();
});

// Authentication functions
async function checkAuth() {
    try {
        const response = await fetch('/api/user', {
            credentials: 'include'
        });
        
        if (response.ok) {
            currentUser = await response.json();
            showBudgetApp();
        } else {
            showAuthModal();
        }
    } catch (error) {
        console.error('Auth check failed:', error);
        showAuthModal();
    }
}

function showAuthModal() {
    document.getElementById('authModal').style.display = 'block';
    document.getElementById('budgetApp').style.display = 'none';
    document.getElementById('userInfo').style.display = 'none';
}

function showBudgetApp() {
    document.getElementById('authModal').style.display = 'none';
    document.getElementById('settingsModal').style.display = 'none';
    document.getElementById('budgetApp').style.display = 'block';
    document.getElementById('userInfo').style.display = 'flex';
    document.getElementById('userName').textContent = currentUser.full_name;
    loadBudget();
}

async function login(event) {
    event.preventDefault();
    
    const email = document.getElementById('loginEmail').value;
    const password = document.getElementById('loginPassword').value;
    
    try {
        const response = await fetch('/api/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include',
            body: JSON.stringify({ email, password })
        });
        
        if (response.ok) {
            currentUser = await response.json();
            showBudgetApp();
        } else {
            alert('Login failed. Please check your credentials.');
        }
    } catch (error) {
        console.error('Login error:', error);
        alert('Login failed. Please try again.');
    }
}

async function register(event) {
    event.preventDefault();
    
    const full_name = document.getElementById('registerName').value;
    const email = document.getElementById('registerEmail').value;
    const password = document.getElementById('registerPassword').value;
    
    try {
        const response = await fetch('/api/register', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include',
            body: JSON.stringify({ full_name, email, password })
        });
        
        if (response.ok) {
            currentUser = await response.json();
            showBudgetApp();
        } else if (response.status === 409) {
            alert('Email already exists. Please use a different email.');
        } else {
            alert('Registration failed. Please try again.');
        }
    } catch (error) {
        console.error('Registration error:', error);
        alert('Registration failed. Please try again.');
    }
}

async function logout() {
    try {
        await fetch('/api/logout', {
            method: 'POST',
            credentials: 'include'
        });
        currentUser = null;
        showAuthModal();
    } catch (error) {
        console.error('Logout error:', error);
    }
}

// UI functions
function showLogin() {
    document.getElementById('loginForm').style.display = 'block';
    document.getElementById('registerForm').style.display = 'none';
    document.querySelector('.tab-btn').classList.add('active');
    document.querySelectorAll('.tab-btn')[1].classList.remove('active');
}

function showRegister() {
    document.getElementById('loginForm').style.display = 'none';
    document.getElementById('registerForm').style.display = 'block';
    document.querySelector('.tab-btn').classList.remove('active');
    document.querySelectorAll('.tab-btn')[1].classList.add('active');
}

function showSettings() {
    document.getElementById('settingsModal').style.display = 'block';
}

function closeSettings() {
    document.getElementById('settingsModal').style.display = 'none';
}

// Budget functions
async function loadCategories() {
    try {
        const response = await fetch('/api/budget/categories');
        categories = await response.json();
    } catch (error) {
        console.error('Failed to load categories:', error);
    }
}

async function loadBudget() {
    const year = document.getElementById('yearSelect').value;
    
    try {
        const response = await fetch(`/api/budget?year=${year}`, {
            credentials: 'include'
        });
        
        if (response.ok) {
            budgetData = await response.json();
            renderBudgetGrid();
            updateSummary();
        }
    } catch (error) {
        console.error('Failed to load budget:', error);
    }
}

function renderBudgetGrid() {
    const grid = document.getElementById('budgetGrid');
    grid.innerHTML = '';
    
    // Add category controls
    const controlsDiv = document.createElement('div');
    controlsDiv.className = 'category-controls';
    controlsDiv.innerHTML = `
        <input type="text" class="search-box" placeholder="Search categories and subcategories..." id="categorySearch">
        <div class="filter-buttons">
            <button class="filter-btn active" data-filter="all">All</button>
            <button class="filter-btn" data-filter="income">Income</button>
            <button class="filter-btn" data-filter="expense">Expenses</button>
        </div>
        <button class="collapse-all-btn" id="collapseAllBtn">Collapse All</button>
    `;
    grid.appendChild(controlsDiv);
    
    // Add event listeners for controls
    document.getElementById('categorySearch').addEventListener('input', handleSearch);
    document.querySelectorAll('.filter-btn').forEach(btn => {
        btn.addEventListener('click', handleFilter);
    });
    document.getElementById('collapseAllBtn').addEventListener('click', toggleCollapseAll);
    
    filteredCategories = [...categories];
    renderCategories();
}

function renderCategories() {
    const grid = document.getElementById('budgetGrid');
    const existingControls = grid.querySelector('.category-controls');
    grid.innerHTML = '';
    if (existingControls) {
        grid.appendChild(existingControls);
    }
    
    filteredCategories.forEach(category => {
        const section = document.createElement('div');
        section.className = `category-section ${category.is_income ? 'income' : 'expense'}`;
        section.setAttribute('data-category', category.name.toLowerCase());
        
        const header = document.createElement('div');
        header.className = 'category-header';
        
        const toggleBtn = document.createElement('button');
        toggleBtn.className = `category-toggle ${collapsedCategories.has(category.name) ? 'collapsed' : ''}`;
        toggleBtn.innerHTML = `${category.name} <span class="subcategory-count">(${category.subcategories.length} items)</span>`;
        toggleBtn.addEventListener('click', () => toggleCategory(category.name));
        
        header.appendChild(toggleBtn);
        section.appendChild(header);
        
        const subcategoryGrid = document.createElement('div');
        subcategoryGrid.className = `subcategory-grid ${collapsedCategories.has(category.name) ? 'collapsed' : ''}`;
        
        // Header row
        const headerRow = document.createElement('div');
        headerRow.className = 'subcategory-row';
        headerRow.innerHTML = `
            <div class="subcategory-name"><strong>Subcategory</strong></div>
            ${months.map(month => `<div class="month-header">${month}</div>`).join('')}
            <div class="month-header">TOTAL</div>
        `;
        subcategoryGrid.appendChild(headerRow);
        
        // Subcategory rows
        category.subcategories.forEach(subcategory => {
            const row = document.createElement('div');
            row.className = 'subcategory-row';
            row.setAttribute('data-subcategory', subcategory.toLowerCase());
            
            const nameCell = document.createElement('div');
            nameCell.className = 'subcategory-name';
            nameCell.textContent = subcategory;
            nameCell.title = subcategory; // Tooltip for long names
            row.appendChild(nameCell);
            
            let yearTotal = 0;
            
            // Month cells
            for (let month = 1; month <= 12; month++) {
                const cell = document.createElement('div');
                cell.className = 'amount-cell';
                
                const input = document.createElement('input');
                input.type = 'number';
                input.step = '0.01';
                input.className = 'amount-input';
                input.placeholder = '0.00';
                
                const existingEntry = budgetData.find(entry => 
                    entry.category === category.name && 
                    entry.subcategory === subcategory && 
                    entry.month === month
                );
                
                if (existingEntry) {
                    input.value = parseFloat(existingEntry.amount);
                    yearTotal += parseFloat(existingEntry.amount);
                }
                
                input.addEventListener('blur', () => saveBudgetEntry(category.name, subcategory, month, input.value));
                input.addEventListener('keypress', (e) => {
                    if (e.key === 'Enter') {
                        input.blur();
                    }
                });
                
                cell.appendChild(input);
                row.appendChild(cell);
            }
            
            // Year total cell
            const totalCell = document.createElement('div');
            totalCell.className = 'year-total';
            totalCell.textContent = formatCurrency(yearTotal);
            row.appendChild(totalCell);
            
            subcategoryGrid.appendChild(row);
        });
        
        section.appendChild(subcategoryGrid);
        grid.appendChild(section);
    });
}

function handleSearch(event) {
    const searchTerm = event.target.value.toLowerCase();
    
    if (searchTerm === '') {
        filteredCategories = [...categories];
    } else {
        filteredCategories = categories.filter(category => {
            const categoryMatch = category.name.toLowerCase().includes(searchTerm);
            const subcategoryMatch = category.subcategories.some(sub => 
                sub.toLowerCase().includes(searchTerm)
            );
            return categoryMatch || subcategoryMatch;
        }).map(category => {
            if (category.name.toLowerCase().includes(searchTerm)) {
                return category;
            } else {
                // Filter subcategories that match the search
                return {
                    ...category,
                    subcategories: category.subcategories.filter(sub => 
                        sub.toLowerCase().includes(searchTerm)
                    )
                };
            }
        });
    }
    
    renderCategories();
    highlightSearchResults(searchTerm);
}

function highlightSearchResults(searchTerm) {
    if (searchTerm === '') return;
    
    document.querySelectorAll('.subcategory-name').forEach(element => {
        const text = element.textContent;
        const regex = new RegExp(`(${searchTerm})`, 'gi');
        element.innerHTML = text.replace(regex, '<mark>$1</mark>');
    });
}

function handleFilter(event) {
    document.querySelectorAll('.filter-btn').forEach(btn => btn.classList.remove('active'));
    event.target.classList.add('active');
    
    const filter = event.target.getAttribute('data-filter');
    
    switch (filter) {
        case 'income':
            filteredCategories = categories.filter(cat => cat.is_income);
            break;
        case 'expense':
            filteredCategories = categories.filter(cat => !cat.is_income);
            break;
        default:
            filteredCategories = [...categories];
    }
    
    renderCategories();
}

function toggleCategory(categoryName) {
    if (collapsedCategories.has(categoryName)) {
        collapsedCategories.delete(categoryName);
    } else {
        collapsedCategories.add(categoryName);
    }
    
    const section = document.querySelector(`[data-category="${categoryName.toLowerCase()}"]`);
    const toggle = section.querySelector('.category-toggle');
    const grid = section.querySelector('.subcategory-grid');
    
    toggle.classList.toggle('collapsed');
    grid.classList.toggle('collapsed');
}

function toggleCollapseAll() {
    const btn = document.getElementById('collapseAllBtn');
    const isCollapsing = btn.textContent === 'Collapse All';
    
    if (isCollapsing) {
        categories.forEach(category => collapsedCategories.add(category.name));
        btn.textContent = 'Expand All';
    } else {
        collapsedCategories.clear();
        btn.textContent = 'Collapse All';
    }
    
    renderCategories();
}

async function saveBudgetEntry(category, subcategory, month, amount) {
    const year = parseInt(document.getElementById('yearSelect').value);
    const numAmount = parseFloat(amount) || 0;
    
    try {
        await fetch('/api/budget', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include',
            body: JSON.stringify({
                category,
                subcategory,
                month,
                year,
                amount: numAmount
            })
        });
        
        // Update local data
        const existingIndex = budgetData.findIndex(entry => 
            entry.category === category && 
            entry.subcategory === subcategory && 
            entry.month === month
        );
        
        if (existingIndex >= 0) {
            budgetData[existingIndex].amount = numAmount;
        } else {
            budgetData.push({
                category,
                subcategory,
                month,
                year,
                amount: numAmount
            });
        }
        
        updateSummary();
        updateYearTotals();
    } catch (error) {
        console.error('Failed to save budget entry:', error);
    }
}

function updateSummary() {
    let totalIncome = 0;
    let totalExpenses = 0;
    
    budgetData.forEach(entry => {
        const category = categories.find(cat => cat.name === entry.category);
        const amount = parseFloat(entry.amount) || 0;
        
        if (category && category.is_income) {
            totalIncome += amount;
        } else {
            totalExpenses += amount;
        }
    });
    
    const netAmount = totalIncome - totalExpenses;
    
    document.getElementById('totalIncome').textContent = formatCurrency(totalIncome);
    document.getElementById('totalExpenses').textContent = formatCurrency(totalExpenses);
    document.getElementById('netAmount').textContent = formatCurrency(netAmount);
    document.getElementById('netAmount').style.color = netAmount >= 0 ? '#28a745' : '#dc3545';
}

function updateYearTotals() {
    const rows = document.querySelectorAll('.subcategory-row');
    rows.forEach((row, index) => {
        if (index === 0) return; // Skip header row
        
        const inputs = row.querySelectorAll('.amount-input');
        let total = 0;
        
        inputs.forEach(input => {
            total += parseFloat(input.value) || 0;
        });
        
        const totalCell = row.querySelector('.year-total');
        if (totalCell) {
            totalCell.textContent = formatCurrency(total);
        }
    });
}

// Settings functions
async function confirmDeleteAccount() {
    if (confirm('Are you sure you want to delete your account? This action cannot be undone and will permanently delete all your data.')) {
        if (confirm('This is your final warning. Delete account and all data?')) {
            await deleteAccount();
        }
    }
}

async function deleteAccount() {
    try {
        const response = await fetch('/api/user', {
            method: 'DELETE',
            credentials: 'include'
        });
        
        if (response.ok) {
            alert('Your account has been successfully deleted.');
            currentUser = null;
            showAuthModal();
        } else {
            alert('Failed to delete account. Please try again.');
        }
    } catch (error) {
        console.error('Delete account error:', error);
        alert('Failed to delete account. Please try again.');
    }
}

// Utility functions
function formatCurrency(amount) {
    return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD'
    }).format(amount);
}

const months = ['JAN', 'FEB', 'MAR', 'APR', 'MAY', 'JUN', 'JUL', 'AUG', 'SEP', 'OCT', 'NOV', 'DEC'];

// Event listeners
document.getElementById('logoutBtn').addEventListener('click', logout);
document.getElementById('settingsBtn').addEventListener('click', showSettings);

// Add keyboard shortcuts
document.addEventListener('keydown', (e) => {
    if (e.ctrlKey || e.metaKey) {
        switch (e.key) {
            case 'f':
                e.preventDefault();
                document.getElementById('categorySearch').focus();
                break;
            case 's':
                e.preventDefault();
                // Auto-save all changes
                console.log('Auto-save triggered');
                break;
        }
    }
});
