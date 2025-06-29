let currentUser = null;
let budgetData = [];
let categories = [];

// Category management variables
let allCategories = [];
let filteredCategories = [];
let collapsedCategories = new Set();
let isGuest = false;

// Initialize app
document.addEventListener('DOMContentLoaded', async () => {
    await loadCategories();
    await checkAuth();

    // Theme switcher
    const themeSwitcher = document.getElementById('themeSwitcher');
    if (themeSwitcher) {
        themeSwitcher.addEventListener('click', toggleTheme);
        // Set initial theme
        if (localStorage.getItem('theme') === 'dark') setDarkTheme();
    }

    // Set current year as default if available, otherwise select the closest year
    const yearSelect = document.getElementById('yearSelect');
    const currentYear = new Date().getFullYear();
    let found = false;
    for (let i = 0; i < yearSelect.options.length; i++) {
        if (parseInt(yearSelect.options[i].value) === currentYear) {
            yearSelect.selectedIndex = i;
            found = true;
            break;
        }
    }
    if (!found) {
        yearSelect.selectedIndex = 0; // fallback to first option
    }

    // Guest sign-in button logic
    const guestSignInBtnContainer = document.getElementById('guestSignInBtnContainer');
    if (guestSignInBtnContainer) {
        guestSignInBtnContainer.style.display = 'none';
    }
});

// Authentication functions
async function checkAuth() {
    try {
        const response = await fetch('/api/user', {
            credentials: 'include'
        });
        
        if (response.ok) {
            currentUser = await response.json();
            isGuest = false;
            showBudgetApp();
        } else {
            showAuthModal();
        }
    } catch (error) {
        showAuthModal();
    }
}

function showAuthModal() {
    isGuest = false;
    document.getElementById('authModal').style.display = 'block';
    document.getElementById('budgetApp').style.display = 'none';
    document.getElementById('userInfo').style.display = 'none';
    // Hide guest sign-in button if present
    const guestSignInBtnContainer = document.getElementById('guestSignInBtnContainer');
    if (guestSignInBtnContainer) guestSignInBtnContainer.style.display = 'none';
    // Autofocus login email for better UX
    setTimeout(() => {
        const emailInput = document.getElementById('loginEmail');
        if (emailInput) emailInput.focus();
    }, 100);
}

// Allow closing modal for guest/demo mode
function closeAuthModal() {
    continueAsGuest();
}

// Guest/demo mode logic
function continueAsGuest() {
    isGuest = true;
    currentUser = null;
    document.getElementById('authModal').style.display = 'none';
    document.getElementById('settingsModal').style.display = 'none';
    document.getElementById('budgetApp').style.display = 'block';
    document.getElementById('userInfo').style.display = 'none';
    // Show guest sign-in button
    const guestSignInBtnContainer = document.getElementById('guestSignInBtnContainer');
    if (guestSignInBtnContainer) guestSignInBtnContainer.style.display = 'block';
    loadBudget();
}

// Show login/register modal from guest/demo mode
function showAuthModalFromGuest() {
    isGuest = false;
    showAuthModal();
}

function showBudgetApp() {
    document.getElementById('authModal').style.display = 'none';
    document.getElementById('settingsModal').style.display = 'none';
    document.getElementById('budgetApp').style.display = 'block';
    document.getElementById('userInfo').style.display = 'flex';
    document.getElementById('userName').textContent = currentUser.full_name;
    // Show analytics button
    const analyticsBtn = document.getElementById('analyticsBtn');
    if (analyticsBtn) analyticsBtn.style.display = 'inline-block';
    // Show avatar initial
    const avatar = document.getElementById('userAvatar');
    if (avatar && currentUser.full_name) {
        avatar.textContent = currentUser.full_name[0].toUpperCase();
    }
    // Hide guest sign-in button
    const guestSignInBtnContainer = document.getElementById('guestSignInBtnContainer');
    if (guestSignInBtnContainer) guestSignInBtnContainer.style.display = 'none';
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
        } else {
            // Improved: Show specific error messages
            let errorMsg = 'Registration failed. Please try again.';
            try {
                const data = await response.json();
                if (data.errors && Array.isArray(data.errors)) {
                    errorMsg = data.errors.join('\n');
                } else if (data.error) {
                    errorMsg = data.error;
                }
            } catch (_) {
                // Ignore JSON parse errors
            }
            alert(errorMsg);
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
    if (isGuest) {
        alert('Sign in to access account settings.');
        return;
    }
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
    if (isGuest) {
        // Demo: Use localStorage or just empty data for demo
        budgetData = [];
        renderBudgetGrid();
        updateSummary();
        return;
    }
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

    // Modernized category controls layout
    const controlsDiv = document.createElement('div');
    controlsDiv.className = 'category-controls';
    controlsDiv.innerHTML = `
        <input type="text" class="search-box" placeholder="Search categories or subcategories..." id="categorySearch" />
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

function handleSearch(event) {
    const searchTerm = event.target.value.toLowerCase();
    
    if (searchTerm === '') {
        filteredCategories = [...categories];
    } else {
        filteredCategories = categories.filter(category => {
            // Search in category name
            if (category.name.toLowerCase().includes(searchTerm)) {
                return true;
            }
            // Search in subcategories
            return category.subcategories.some(sub => 
                sub.toLowerCase().includes(searchTerm)
            );
        });
    }
    
    renderCategories();
}

function handleFilter(event) {
    const filterType = event.target.dataset.filter;
    
    // Update active button
    document.querySelectorAll('.filter-btn').forEach(btn => {
        btn.classList.remove('active');
    });
    event.target.classList.add('active');
    
    // Filter categories
    switch (filterType) {
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

function toggleCollapseAll() {
    const btn = document.getElementById('collapseAllBtn');
    const isCollapsing = btn.textContent.includes('Collapse');
    
    if (isCollapsing) {
        // Collapse all
        categories.forEach(category => {
            collapsedCategories.add(category.name);
        });
        btn.textContent = 'Expand All';
    } else {
        // Expand all
        collapsedCategories.clear();
        btn.textContent = 'Collapse All';
    }
    
    renderCategories();
}

function renderCategories() {
    const grid = document.getElementById('budgetGrid');
    const existingControls = grid.querySelector('.category-controls');
    grid.innerHTML = '';
    if (existingControls) {
        grid.appendChild(existingControls);
    }

    // Determine current month and selected year
    const now = new Date();
    const selectedYear = parseInt(document.getElementById('yearSelect').value);
    const currentMonth = (now.getFullYear() === selectedYear) ? now.getMonth() + 1 : null;

    filteredCategories.forEach((category, categoryIndex) => {
        const section = document.createElement('div');
        section.className = `category-section ${category.is_income ? 'income' : 'expense'}`;
        section.setAttribute('data-category', category.name.toLowerCase());

        const header = document.createElement('div');
        header.className = 'category-header';

        const toggleBtn = document.createElement('button');
        toggleBtn.className = `category-toggle ${collapsedCategories.has(category.name) ? 'collapsed' : ''}`;
        toggleBtn.innerHTML = `
            ${category.name} 
            <span class="subcategory-count">${category.subcategories.length} items</span>
        `;
        toggleBtn.addEventListener('click', () => toggleCategory(category.name));

        header.appendChild(toggleBtn);
        section.appendChild(header);

        const subcategoryGrid = document.createElement('div');
        subcategoryGrid.className = `subcategory-grid ${collapsedCategories.has(category.name) ? 'collapsed' : ''}`;

        // Header row with month labels (desktop only)
        const headerRow = document.createElement('div');
        headerRow.className = 'subcategory-row month-header-row';
        // --- Uniform grid: left subtitle cell, right month grid ---
        headerRow.innerHTML = `
            <div class="subcategory-name"></div>
            <div class="month-inputs">
                ${months.map((month, idx) => 
                    `<div class="month-input-group">
                        <div class="month-label${currentMonth === idx + 1 ? ' current-month' : ''}">${month}</div>
                    </div>`
                ).join('')}
            </div>
        `;
        subcategoryGrid.appendChild(headerRow);

        // Subcategory rows
        category.subcategories.forEach((subcategory, subIndex) => {
            const row = document.createElement('div');
            row.className = 'subcategory-row';
            row.setAttribute('data-subcategory', subcategory.toLowerCase());

            // --- Uniform grid: left subtitle cell, right month grid ---
            // Left: subtitle cell
            const nameCell = document.createElement('div');
            nameCell.className = 'subcategory-name';

            // Create a flex container for name and autofill button
            const nameText = document.createElement('span');
            nameText.textContent = subcategory;
            nameText.style.flex = '1 1 auto';
            nameText.style.overflow = 'hidden';
            nameText.style.textOverflow = 'ellipsis';
            nameText.style.whiteSpace = 'nowrap';

            // Autofill button for bulk entry
            const autofillBtn = document.createElement('button');
            autofillBtn.textContent = 'Fill All';
            autofillBtn.className = 'autofill-btn';
            autofillBtn.title = 'Fill all months with a value';
            autofillBtn.onclick = () => {
                const value = prompt('Enter amount to fill all months:');
                if (value !== null && !isNaN(value)) {
                    const numValue = parseFloat(value) || 0;
                    row.querySelectorAll('.amount-input').forEach((input, idx) => {
                        input.value = numValue;
                        input.classList.toggle('has-value', numValue !== 0);
                        // Save for each month
                        saveBudgetEntry(category.name, subcategory, idx + 1, numValue);
                        updateRowYearTotal(row);
                    });
                }
            };

            // Clear nameCell and append nameText and autofillBtn
            nameCell.appendChild(nameText);
            nameCell.appendChild(autofillBtn);

            row.appendChild(nameCell);

            // Right: month inputs container
            const monthInputsContainer = document.createElement('div');
            monthInputsContainer.className = 'month-inputs';

            let yearTotal = 0;

            // Create month inputs
            for (let month = 1; month <= 12; month++) {
                const monthGroup = document.createElement('div');
                monthGroup.className = 'month-input-group';

                // Always show month label above input (handled in header row)
                // For data rows, just input
                const input = document.createElement('input');
                input.type = 'number';
                input.step = '0.01';
                input.className = `amount-input${currentMonth === month ? ' current-month' : ''}`;
                input.placeholder = '0.00';
                input.setAttribute('inputmode', 'decimal');

                const existingEntry = budgetData.find(entry =>
                    entry.category === category.name &&
                    entry.subcategory === subcategory &&
                    entry.month === month
                );

                if (existingEntry) {
                    const amount = parseFloat(existingEntry.amount);
                    input.value = amount;
                    yearTotal += amount;
                    if (amount !== 0) {
                        input.classList.add('has-value');
                    }
                }

                // Input event handlers
                input.addEventListener('focus', function() {
                    setTimeout(() => {
                        input.select();
                        row.classList.add('editing');
                    }, 10);
                });

                input.addEventListener('blur', function() {
                    row.classList.remove('editing');
                    const value = parseFloat(input.value) || 0;
                    
                    if (value !== 0) {
                        input.classList.add('has-value');
                    } else {
                        input.classList.remove('has-value');
                    }
                    
                    saveBudgetEntry(category.name, subcategory, month, input.value);
                });

                input.addEventListener('input', function() {
                    const value = parseFloat(input.value) || 0;
                    if (value !== 0) {
                        input.classList.add('has-value');
                    } else {
                        input.classList.remove('has-value');
                    }
                    
                    updateRowYearTotal(row);
                });

                // Keyboard navigation
                input.addEventListener('keydown', function(e) {
                    if (['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'Enter', 'Tab'].includes(e.key)) {
                        const allInputs = Array.from(document.querySelectorAll('.amount-input'));
                        const idx = allInputs.indexOf(input);
                        let nextIdx = idx;
                        
                        if (e.key === 'ArrowLeft') nextIdx = Math.max(0, idx - 1);
                        if (e.key === 'ArrowRight') nextIdx = Math.min(allInputs.length - 1, idx + 1);
                        if (e.key === 'ArrowUp') nextIdx = idx - 12 >= 0 ? idx - 12 : idx;
                        if (e.key === 'ArrowDown') nextIdx = idx + 12 < allInputs.length ? idx + 12 : idx;
                        
                        if (nextIdx !== idx && allInputs[nextIdx]) {
                            e.preventDefault();
                            allInputs[nextIdx].focus();
                        }
                    }
                });

                monthGroup.appendChild(input);
                monthInputsContainer.appendChild(monthGroup);
            }

            row.appendChild(monthInputsContainer);

            // Year total display
            const yearTotalDisplay = document.createElement('div');
            yearTotalDisplay.className = 'year-total-display';
            yearTotalDisplay.innerHTML = `
                <div class="year-total-label">Year Total</div>
                <div class="year-total-amount ${yearTotal >= 0 ? 'positive' : 'negative'}">${formatCurrency(yearTotal)}</div>
            `;
            row.appendChild(yearTotalDisplay);

            subcategoryGrid.appendChild(row);
        });

        section.appendChild(subcategoryGrid);
        grid.appendChild(section);
    });
}

function updateRowYearTotal(row) {
    const inputs = row.querySelectorAll('.amount-input');
    let total = 0;
    
    inputs.forEach(input => {
        total += parseFloat(input.value) || 0;
    });
    
    const totalDisplay = row.querySelector('.year-total-amount');
    if (totalDisplay) {
        totalDisplay.textContent = formatCurrency(total);
        totalDisplay.className = `year-total-amount ${total >= 0 ? 'positive' : 'negative'}`;
        
        // Add update animation
        totalDisplay.style.transform = 'scale(1.1)';
        setTimeout(() => {
            totalDisplay.style.transform = 'scale(1)';
        }, 200);
    }
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
    
    // Enhanced toggle animation
    toggle.classList.toggle('collapsed');
    
    if (grid.classList.contains('collapsed')) {
        grid.classList.remove('collapsed');
        // Expand animation
        grid.style.maxHeight = grid.scrollHeight + 'px';
        setTimeout(() => {
            grid.style.maxHeight = 'none';
        }, 400);
    } else {
        grid.classList.add('collapsed');
        // Collapse animation
        grid.style.maxHeight = grid.scrollHeight + 'px';
        requestAnimationFrame(() => {
            grid.style.maxHeight = '0';
        });
    }
}

async function saveBudgetEntry(category, subcategory, month, amount) {
    const year = parseInt(document.getElementById('yearSelect').value);
    const numAmount = parseFloat(amount) || 0;
    
    // Add loading state
    const row = document.querySelector(`[data-subcategory="${subcategory.toLowerCase()}"]`);
    if (row) {
        row.classList.add('loading');
        setTimeout(() => row.classList.remove('loading'), 500);
    }
    
    if (isGuest) {
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
        return;
    }
    
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
    } catch (error) {
        console.error('Failed to save budget entry:', error);
        // Show error feedback
        if (row) {
            row.style.background = '#fee2e2';
            setTimeout(() => {
                row.style.background = '';
            }, 2000);
        }
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
    if (isGuest) {
        alert('Sign in to delete your account.');
        return;
    }
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

// Security modal logic
function showSecurityModal() {
    document.getElementById('securityModal').style.display = 'block';
}
function closeSecurityModal() {
    document.getElementById('securityModal').style.display = 'none';
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

// Theme switcher logic
function toggleTheme() {
    if (document.body.classList.contains('dark-theme')) {
        setLightTheme();
    } else {
        setDarkTheme();
    }
}
function setDarkTheme() {
    document.body.classList.add('dark-theme');
    localStorage.setItem('theme', 'dark');
    document.getElementById('themeSwitcher').textContent = '☀️';
}
function setLightTheme() {
    document.body.classList.remove('dark-theme');
    localStorage.setItem('theme', 'light');
    document.getElementById('themeSwitcher').textContent = '🌙';
}

// (No changes needed, your file already contains logic matching the modernized UI and CSS.)
// (No changes needed, your file already contains logic matching the modernized UI and CSS.)
// (No changes needed, your file already contains logic matching the modernized UI and CSS.)
