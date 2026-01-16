/**
 * JobWeaver Web Application
 * 
 * Main JavaScript application for the JobWeaver Control-M job analysis tool.
 * Provides a single-page application with authentication, job search, filtering,
 * dashboard statistics, and dependency graph visualization.
 */

// ============================================================================
// CONFIGURATION AND STATE
// ============================================================================

/** Base URL for API endpoints */
const API_BASE = '/api';

/** JWT authentication token */
let authToken = null;

/** Currently authenticated user information */
let currentUser = null;

/** Current page number for pagination */
let currentPage = 1;

/** Number of items to display per page */
let currentPerPage = 50;

/** Current sort configuration */
let currentSort = { by: 'job_name', order: 'asc' };

/** Current active filters for job search */
let currentFilters = {};

// ============================================================================
// APPLICATION INITIALIZATION
// ============================================================================

/**
 * Initialize the application when DOM is ready
 * Sets up authentication check and event listeners
 */
document.addEventListener('DOMContentLoaded', () => {
    checkAuth();
    initializeEventListeners();
});

// ============================================================================
// AUTHENTICATION
// ============================================================================

/**
 * Checks if user is authenticated
 * Retrieves token from localStorage and validates with server
 */
function checkAuth() {
    authToken = localStorage.getItem('authToken');
    if (authToken) {
        fetchCurrentUser();
    } else {
        showPage('login-page');
    }
}

/**
 * Fetches current user information from the server
 * Uses stored JWT token for authentication
 */
async function fetchCurrentUser() {
    try {
        const response = await fetch(`${API_BASE}/auth/me`, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
        });
        
        if (response.ok) {
            const result = await response.json();
            currentUser = result.data;
            const userDisplay = document.getElementById('current-user');
            if (userDisplay) {
                userDisplay.textContent = currentUser.display_name || currentUser.username;
            }
            showPage('main-app');
            switchContentPage('dashboard');
        } else {
            logout();
        }
    } catch (error) {
        console.error('Auth check failed:', error);
        logout();
    }
}

/**
 * Shows a specific page and hides others
 * 
 * @param {string} pageId - ID of the page element to show
 */
function showPage(pageId) {
    document.querySelectorAll('.page').forEach(page => {
        page.classList.remove('active');
    });
    const targetPage = document.getElementById(pageId);
    if (targetPage) {
        targetPage.classList.add('active');
    }
}

// ============================================================================
// EVENT LISTENERS INITIALIZATION
// ============================================================================

/**
 * Initializes all event listeners for the application
 * Sets up login, navigation, search, table, and modal listeners
 */
function initializeEventListeners() {
    initializeLoginListeners();
    initializeNavigationListeners();
    initializeSearchListeners();
    initializeTableListeners();
    initializeModalListeners();
    initializeDashboardListeners();
}

/**
 * Initializes login-related event listeners
 * Sets up tabs, form submission, Entra ID login, and logout
 */
function initializeLoginListeners() {
    initializeLoginTabs();
    initializeLoginForm();
    initializeEntraLogin();
    initializeLogout();
}

/**
 * Initializes login tab switching functionality
 * Allows switching between local and Entra ID authentication
 */
function initializeLoginTabs() {
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            const tab = btn.dataset.tab;
            document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
            btn.classList.add('active');
            document.getElementById(`${tab}-login`).classList.add('active');
        });
    });
}

/**
 * Initializes the login form event handlers
 * Handles form submission and Enter key press
 */
function initializeLoginForm() {
    const loginForm = document.getElementById('login-form');
    if (loginForm) {
        loginForm.addEventListener('submit', async (e) => {
            e.preventDefault();
            await handleLogin(e);
        });
    }
    
    const passwordField = document.getElementById('password');
    if (passwordField) {
        passwordField.addEventListener('keypress', async (e) => {
            if (e.key === 'Enter') {
                e.preventDefault();
                await handleLogin(e);
            }
        });
    }
}

/**
 * Initializes Entra ID (Azure AD) login button
 */
function initializeEntraLogin() {
    const entraLoginBtn = document.getElementById('entra-login-btn');
    if (entraLoginBtn) {
        entraLoginBtn.addEventListener('click', handleEntraLogin);
    }
}

/**
 * Initializes logout button event handler
 */
function initializeLogout() {
    const logoutBtn = document.getElementById('logout-btn');
    if (logoutBtn) {
        logoutBtn.addEventListener('click', logout);
    }
}

/**
 * Initializes navigation menu event listeners
 * Handles switching between dashboard and jobs pages
 */
function initializeNavigationListeners() {
    document.querySelectorAll('.nav-item').forEach(item => {
        item.addEventListener('click', (e) => {
            e.preventDefault();
            const page = item.dataset.page;
            switchContentPage(page);
        });
    });
}

/**
 * Initializes search and filter control event listeners
 * Sets up search, reset, export, and pagination controls
 */
function initializeSearchListeners() {
    const searchBtn = document.getElementById('search-btn');
    const resetBtn = document.getElementById('reset-btn');
    const exportBtn = document.getElementById('export-csv-btn');
    const perPageSelect = document.getElementById('per-page');
    
    if (searchBtn) searchBtn.addEventListener('click', performSearch);
    if (resetBtn) resetBtn.addEventListener('click', resetFilters);
    if (exportBtn) exportBtn.addEventListener('click', exportToCSV);
    if (perPageSelect) {
        perPageSelect.addEventListener('change', (e) => {
            currentPerPage = parseInt(e.target.value);
            performSearch();
        });
    }
}

/**
 * Initializes table column header click listeners for sorting
 */
function initializeTableListeners() {
    document.querySelectorAll('.data-table th[data-sort]').forEach(th => {
        th.addEventListener('click', () => {
            handleTableSort(th.dataset.sort);
        });
    });
}

/**
 * Handles table column sorting
 * Toggles sort order if same column, otherwise sets to ascending
 * 
 * @param {string} sortBy - Column name to sort by
 */
function handleTableSort(sortBy) {
    if (currentSort.by === sortBy) {
        currentSort.order = currentSort.order === 'asc' ? 'desc' : 'asc';
    } else {
        currentSort.by = sortBy;
        currentSort.order = 'asc';
    }
    performSearch();
}

/**
 * Initializes modal dialog event listeners
 * Sets up close buttons and click-outside-to-close functionality
 */
function initializeModalListeners() {
    document.querySelectorAll('.modal-close').forEach(btn => {
        btn.addEventListener('click', closeAllModals);
    });
    
    document.querySelectorAll('.modal').forEach(modal => {
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                modal.classList.remove('active');
            }
        });
    });
}

/**
 * Closes all open modal dialogs
 */
function closeAllModals() {
    document.querySelectorAll('.modal').forEach(m => m.classList.remove('active'));
}

/**
 * Initializes dashboard filter event listeners
 */
function initializeDashboardListeners() {
    const dashboardFilter = document.getElementById('dashboard-folder-filter');
    console.log('📊 [DASHBOARD] Initializing dashboard listeners...');
    console.log('📊 [DASHBOARD] Filter element found:', dashboardFilter);
    
    if (dashboardFilter) {
        // Remove any existing listener to prevent duplicates
        const newFilter = dashboardFilter.cloneNode(true);
        dashboardFilter.parentNode.replaceChild(newFilter, dashboardFilter);
        
        // Add the event listener to the new element
        newFilter.addEventListener('change', (e) => {
            const filterValue = e.target.value;
            console.log(`📊 [DASHBOARD] Filter changed to: ${filterValue || 'All Jobs'}`);
            loadDashboard(filterValue);
        });
        console.log('📊 [DASHBOARD] Event listener attached successfully');
    } else {
        console.warn('📊 [DASHBOARD] Filter element not found - will retry when dashboard page is shown');
    }
}

/**
 * Handles user login form submission
 * Validates credentials and stores JWT token on success
 * 
 * @param {Event} e - Form submit event
 * @returns {Promise<boolean>} False to prevent default form submission
 */
async function handleLogin(e) {
    console.log('🔐 [LOGIN] ========== LOGIN FUNCTION CALLED ==========');
    console.log('[LOGIN] Event:', e);
    
    if (e) {
        e.preventDefault();
        console.log('[LOGIN] preventDefault() called');
    }
    
    const username = document.getElementById('username')?.value;
    const password = document.getElementById('password')?.value;
    const errorDiv = document.getElementById('login-error');
    
    console.log('[LOGIN] Username field value:', username);
    console.log('[LOGIN] Password field exists:', !!password);
    console.log('[LOGIN] Error div exists:', !!errorDiv);
    
    if (errorDiv) {
        errorDiv.style.display = 'none';
    }
    
    if (!username || !password) {
        console.error('[LOGIN] ❌ Missing credentials');
        if (errorDiv) {
            errorDiv.textContent = 'Please enter both username and password';
            errorDiv.style.display = 'block';
        }
        return false;
    }
    
    try {
        console.log('[LOGIN] 🌐 Sending POST request to:', `${API_BASE}/auth/login`);
        console.log('[LOGIN] Request body:', { username, password: '***' });
        
        const response = await fetch(`${API_BASE}/auth/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username, password })
        });
        
        console.log('[LOGIN] 📡 Response status:', response.status);
        console.log('[LOGIN] Response ok:', response.ok);
        
        const result = await response.json();
        console.log('[LOGIN] 📦 Response data:', result);
        
        if (result.success && result.data) {
            console.log('[LOGIN] ✅ Login successful!');
            authToken = result.data.token;
            currentUser = result.data.user || { username: username, display_name: username };
            localStorage.setItem('authToken', authToken);
            localStorage.setItem('currentUser', JSON.stringify(currentUser));
            
            console.log('[LOGIN] Token saved:', authToken.substring(0, 20) + '...');
            console.log('[LOGIN] User:', currentUser);
            
            const userDisplayElement = document.getElementById('current-user');
            if (userDisplayElement) {
                userDisplayElement.textContent = currentUser.display_name || currentUser.username;
                console.log('[LOGIN] User display updated');
            }
            
            console.log('[LOGIN] Switching to main-app...');
            showPage('main-app');
            switchContentPage('dashboard');
            console.log('[LOGIN] ✅ Login complete!');
        } else {
            console.error('[LOGIN] ❌ Login failed:', result.error);
            if (errorDiv) {
                errorDiv.textContent = result.error || 'Invalid username or password';
                errorDiv.style.display = 'block';
            }
        }
    } catch (error) {
        console.error('[LOGIN] 💥 Exception:', error);
        console.error('[LOGIN] Error stack:', error.stack);
        if (errorDiv) {
            errorDiv.textContent = 'Connection error. Please try again.';
            errorDiv.style.display = 'block';
        }
    }
    
    console.log('[LOGIN] ========== LOGIN FUNCTION END ==========');
    return false;
}
/**
 * Handles Entra ID (Azure AD) login
 * Currently shows a message that Entra ID is not configured
 * 
 * @returns {Promise<void>}
 */
async function handleEntraLogin() {
    const errorDiv = document.getElementById('login-error');
    
    try {
        // Note: Entra ID OAuth flow would typically redirect to Microsoft's endpoint
        // For now, show a message that it's not configured
        errorDiv.textContent = 'Entra ID login is not configured yet. Please use username/password to login.';
        errorDiv.style.display = 'block';
        errorDiv.style.color = '#f59e0b'; // Warning color
        
        // Example of what the redirect would look like when configured:
        // window.location.href = `${API_BASE}/auth/entra-login`;
    } catch (error) {
        console.error('Entra ID login error:', error);
        errorDiv.textContent = 'Failed to initiate Entra ID login.';
        errorDiv.style.display = 'block';
        errorDiv.style.color = '#ef4444'; // Error color
    }
}

/**
 * Logs out the current user
 * Clears authentication state and returns to login page
 */
function logout() {
    authToken = null;
    currentUser = null;
    localStorage.removeItem('authToken');
    showPage('login-page');
}

/**
 * Switches between content pages (dashboard and jobs)
 * Updates navigation state and loads page-specific data
 * 
 * @param {string} page - Page identifier ('dashboard' or 'jobs')
 */
function switchContentPage(page) {
    console.log(`🔄 [NAV] Switching to ${page} page`);
    
    document.querySelectorAll('.nav-item').forEach(item => {
        item.classList.remove('active');
    });
    document.querySelectorAll('.content-page').forEach(p => {
        p.classList.remove('active');
    });
    
    const navItem = document.querySelector(`[data-page="${page}"]`);
    const pageElement = document.getElementById(`${page}-page`);
    
    if (navItem) navItem.classList.add('active');
    if (pageElement) pageElement.classList.add('active');
    
    if (page === 'dashboard') {
        initializeDashboardListeners(); // Re-attach event listener
        loadDashboard();
    } else if (page === 'jobs') {
        loadFilterOptions();
        performSearch();
    }
}

// ============================================================================
// DASHBOARD
// ============================================================================

/**
 * Loads and displays dashboard statistics
 * Fetches aggregated data and renders charts
 * 
 * @returns {Promise<void>}
 */
async function loadDashboard(filter = '') {
    const startTime = performance.now();
    console.log('📊 [DASHBOARD] Loading dashboard statistics...');
    
    try {
        const fetchStart = performance.now();
        const url = filter ? `${API_BASE}/dashboard/stats?folder_order_method_filter=${filter}` : `${API_BASE}/dashboard/stats`;
        const response = await fetch(url, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
        });
        
        const fetchEnd = performance.now();
        console.log(`⏱️  [DASHBOARD] Stats received in ${(fetchEnd - fetchStart).toFixed(2)}ms`);
        
        const result = await response.json();
        
        if (result.success) {
            const stats = result.data;
            
            console.log('📈 [DASHBOARD] Statistics:');
            console.log(`  - Total Jobs: ${stats.total_jobs}`);
            console.log(`  - Total Folders: ${stats.total_folders}`);
            console.log(`  - Critical Jobs: ${stats.critical_jobs}`);
            console.log(`  - Cyclic Jobs: ${stats.cyclic_jobs}`);
            console.log(`  - File Transfer Jobs: ${stats.file_transfer_jobs}`);
            console.log(`  - CLI Jobs: ${stats.cli_jobs}`);
            
            console.log('🎨 [DASHBOARD] Updating stat cards...');
            const statElements = {
                'stat-total-jobs': stats.total_jobs,
                'stat-total-folders': stats.total_folders,
                'stat-critical-jobs': stats.critical_jobs,
                'stat-cyclic-jobs': stats.cyclic_jobs,
                'stat-file-transfer': stats.file_transfer_jobs,
                'stat-cli-jobs': stats.cli_jobs
            };
            
            for (const [id, value] of Object.entries(statElements)) {
                const element = document.getElementById(id);
                if (element) {
                    element.textContent = value.toLocaleString();
                } else {
                    console.warn(`[DASHBOARD] Element not found: ${id}`);
                }
            }
            
            console.log('📊 [DASHBOARD] Rendering charts...');
            renderBarChart('chart-appl-types', stats.jobs_by_appl_type, 'appl_type', 'count');
            renderBarChart('chart-task-types', stats.jobs_by_task_type, 'task_type', 'count');
            renderBarChart('chart-applications', stats.jobs_by_application, 'application', 'count');
            renderBarChart('chart-folders', stats.jobs_by_folder, 'folder_name', 'job_count');
            
            const endTime = performance.now();
            console.log(`✅ [DASHBOARD] Dashboard loaded in ${(endTime - startTime).toFixed(2)}ms`);
        } else {
            console.error('❌ [DASHBOARD] API returned error:', result.error);
        }
    } catch (error) {
        console.error('❌ [DASHBOARD] Failed to load dashboard:', error);
        console.error('Error details:', error.stack);
    }
}

/**
 * Renders a horizontal bar chart
 * 
 * @param {string} containerId - ID of the container element
 * @param {Array} data - Array of data objects
 * @param {string} labelKey - Key for label values
 * @param {string} valueKey - Key for numeric values
 */
function renderBarChart(containerId, data, labelKey, valueKey) {
    const container = document.getElementById(containerId);
    if (!data || data.length === 0) {
        container.innerHTML = '<p class="text-muted text-center">No data available</p>';
        return;
    }
    
    const maxValue = Math.max(...data.map(item => item[valueKey]));
    
    container.innerHTML = data.map(item => {
        const percentage = (item[valueKey] / maxValue) * 100;
        return `
            <div class="chart-bar">
                <div class="chart-label">${item[labelKey] || 'Unknown'}</div>
                <div class="chart-bar-container">
                    <div class="chart-bar-fill" style="width: ${percentage}%"></div>
                </div>
                <div class="chart-value">${item[valueKey]}</div>
            </div>
        `;
    }).join('');
}

/**
 * Renders a complexity distribution chart
 * 
 * @param {string} containerId - ID of the container element
 * @param {Object} data - Object with low, medium, high counts
 */
function renderComplexityChart(containerId, data) {
    const container = document.getElementById(containerId);
    const total = data.low + data.medium + data.high;
    
    if (total === 0) {
        container.innerHTML = '<p class="text-muted text-center">No data available</p>';
        return;
    }
    
    const items = [
        { label: 'Low Complexity', value: data.low, color: '#10b981' },
        { label: 'Medium Complexity', value: data.medium, color: '#f59e0b' },
        { label: 'High Complexity', value: data.high, color: '#ef4444' }
    ];
    
    container.innerHTML = items.map(item => {
        const percentage = (item.value / total) * 100;
        return `
            <div class="chart-bar">
                <div class="chart-label">${item.label}</div>
                <div class="chart-bar-container">
                    <div class="chart-bar-fill" style="width: ${percentage}%; background: ${item.color}"></div>
                </div>
                <div class="chart-value">${item.value}</div>
            </div>
        `;
    }).join('');
}

// ============================================================================
// JOBS SEARCH
// ============================================================================

/**
 * Initializes Select2 dropdowns for enhanced filtering
 */
function initializeSelect2() {
    $('.select2-dropdown').select2({
        placeholder: 'Select an option',
        allowClear: true,
        width: '100%'
    });
}

/**
 * Loads available filter options from the server
 * Populates dropdown menus with unique values
 * 
 * @returns {Promise<void>}
 */
async function loadFilterOptions() {
    const startTime = performance.now();
    console.log('🔧 [FILTERS] Loading filter options...');
    
    try {
        const response = await fetch(`${API_BASE}/filters`, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
        });
        
        const result = await response.json();
        
        if (result.success) {
            const options = result.data;
            
            console.log('📊 [FILTERS] Filter counts:');
            console.log(`  - Folders: ${options.folders.length}`);
            console.log(`  - Applications: ${options.applications.length}`);
            console.log(`  - APPL_TYPE: ${options.appl_types.length}`);
            console.log(`  - APPL_VER: ${options.appl_vers.length}`);
            console.log(`  - Task Types: ${options.task_types.length}`);
            console.log(`  - Datacenters: ${options.datacenters.length}`);
            console.log(`  - Folder Order Methods: ${options.folder_order_methods.length}`);
            
            console.log('🎨 [FILTERS] Populating dropdowns...');
            populateSelect('filter-folder', options.folders);
            populateSelect('filter-application', options.applications);
            populateSelect('filter-appl-type', options.appl_types);
            populateSelect('filter-appl-ver', options.appl_vers);
            populateSelect('filter-task-type', options.task_types);
            populateSelect('filter-datacenter', options.datacenters);
            populateSelect('filter-folder-order-method', options.folder_order_methods);
            
            console.log('🔍 [FILTERS] Initializing Select2...');
            initializeSelect2();
            
            const endTime = performance.now();
            console.log(`✅ [FILTERS] Filters loaded in ${(endTime - startTime).toFixed(2)}ms`);
        }
    } catch (error) {
        console.error('❌ [FILTERS] Failed to load filter options:', error);
    }
}

/**
 * Populates a select dropdown with options
 * 
 * @param {string} selectId - ID of the select element
 * @param {Array<string>} options - Array of option values
 */
function populateSelect(selectId, options) {
    const select = document.getElementById(selectId);
    const currentValue = select.value;
    
    // Keep first option (All)
    const firstOption = select.options[0];
    select.innerHTML = '';
    select.appendChild(firstOption);
    
    options.forEach(option => {
        const opt = document.createElement('option');
        opt.value = option;
        opt.textContent = option;
        select.appendChild(opt);
    });
    
    select.value = currentValue;
}

/**
 * Collects current filter values from form inputs
 * 
 * @returns {Object} Object containing all filter values
 */
function collectFilterValues() {
    return {
        jobName: document.getElementById('filter-job-name')?.value?.trim(),
        folderName: $('#filter-folder').val(),
        application: $('#filter-application').val(),
        applType: $('#filter-appl-type').val(),
        applVer: $('#filter-appl-ver').val(),
        taskType: $('#filter-task-type').val(),
        datacenter: $('#filter-datacenter').val(),
        folderOrderMethod: $('#filter-folder-order-method').val(),
        critical: document.getElementById('filter-critical')?.value,
        minDeps: document.getElementById('filter-min-deps')?.value?.trim(),
        maxDeps: document.getElementById('filter-max-deps')?.value?.trim(),
        minOnConds: document.getElementById('filter-min-on-conds')?.value?.trim(),
        maxOnConds: document.getElementById('filter-max-on-conds')?.value?.trim(),
        hasVars: document.getElementById('filter-has-variables')?.value,
        minVars: document.getElementById('filter-min-variables')?.value?.trim(),
        hasOdate: document.getElementById('filter-has-odate')?.value
    };
}

/**
 * Builds a filters object for API request
 * Converts form values to API-compatible format
 * 
 * @param {Object} values - Raw filter values from form
 * @returns {Object} Formatted filters object
 */
function buildFiltersObject(values) {
    const filters = {};
    
    if (values.jobName) filters.job_name = values.jobName;
    if (values.folderName) filters.folder_name = values.folderName;
    if (values.application) filters.application = values.application;
    if (values.applType) filters.appl_type = values.applType;
    if (values.applVer) filters.appl_ver = values.applVer;
    if (values.taskType) filters.task_type = values.taskType;
    if (values.datacenter) filters.datacenter = values.datacenter;
    if (values.folderOrderMethod) filters.folder_order_method = values.folderOrderMethod;
    if (values.critical && values.critical !== '') filters.critical = values.critical === 'true';
    if (values.minDeps) filters.min_dependencies = parseInt(values.minDeps);
    if (values.maxDeps) filters.max_dependencies = parseInt(values.maxDeps);
    if (values.minOnConds) filters.min_on_conditions = parseInt(values.minOnConds);
    if (values.maxOnConds) filters.max_on_conditions = parseInt(values.maxOnConds);
    if (values.hasVars && values.hasVars !== '') filters.has_variables = values.hasVars === 'true';
    if (values.minVars) filters.min_variables = parseInt(values.minVars);
    if (values.hasOdate && values.hasOdate !== '') filters.has_odate = values.hasOdate === 'true';
    
    return filters;
}

/**
 * Executes a job search API request
 * 
 * @param {Object} filters - Search filters
 * @returns {Promise<Object>} Search results data
 * @throws {Error} If search fails
 */
async function executeSearchRequest(filters) {
    const response = await fetch(`${API_BASE}/jobs/search`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${authToken}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            ...filters,
            page: currentPage,
            per_page: currentPerPage,
            sort_by: currentSort.by,
            sort_order: currentSort.order
        })
    });
    
    const result = await response.json();
    
    if (!result.success) {
        throw new Error(result.error || 'Search failed');
    }
    
    return result.data;
}

/**
 * Resets all search filters to default values
 * Clears form inputs and performs a new search
 */
function resetFilters() {
    console.log('🔄 [SEARCH] Resetting all filters');
    document.getElementById('filter-job-name').value = '';
    $('#filter-folder').val(null).trigger('change');
    $('#filter-application').val(null).trigger('change');
    $('#filter-appl-type').val(null).trigger('change');
    $('#filter-appl-ver').val(null).trigger('change');
    $('#filter-task-type').val(null).trigger('change');
    document.getElementById('filter-critical').value = '';
    document.getElementById('filter-min-deps').value = '';
    document.getElementById('filter-max-deps').value = '';
    document.getElementById('filter-min-on-conds').value = '';
    document.getElementById('filter-max-on-conds').value = '';
    document.getElementById('filter-has-variables').value = '';
    document.getElementById('filter-min-variables').value = '';
    currentPage = 1;
    performSearch();
}

/**
 * Performs a job search with current filters
 * Fetches data from API and renders results table
 * 
 * @returns {Promise<void>}
 */
async function performSearch() {
    const startTime = performance.now();
    console.log('🔍 [SEARCH] Starting search operation...');
    
    showLoading(true);
    
    try {
        // Collect and build filters
        const filterValues = collectFilterValues();
        console.log('🔍 [SEARCH] Raw filter values:', filterValues);
        
        currentFilters = buildFiltersObject(filterValues);
        console.log('📋 [SEARCH] Filters to send:', currentFilters);
        console.log('📄 [SEARCH] Page:', currentPage, 'Per page:', currentPerPage);
        
        // Execute search request
        console.log('🌐 [SEARCH] Sending API request...');
        const fetchStart = performance.now();
        
        const data = await executeSearchRequest(currentFilters);
        
        const fetchEnd = performance.now();
        console.log(`⏱️  [SEARCH] API response received in ${(fetchEnd - fetchStart).toFixed(2)}ms`);
        console.log(`✅ [SEARCH] Found ${data.total} jobs (showing ${data.jobs.length})`);
        
        // Render results
        console.log('🎨 [SEARCH] Rendering table...');
        const renderStart = performance.now();
        
        renderJobsTable(data);
        
        const renderEnd = performance.now();
        console.log(`✅ [SEARCH] Table rendered in ${(renderEnd - renderStart).toFixed(2)}ms`);
        
    } catch (error) {
        console.error('❌ [SEARCH] Error:', error);
    } finally {
        showLoading(false);
        const endTime = performance.now();
        console.log(`🏁 [SEARCH] Total search time: ${(endTime - startTime).toFixed(2)}ms`);
        console.log('─────────────────────────────────────');
    }
}

/**
 * Shows or hides loading indicators
 * 
 * @param {boolean} show - Whether to show loading state
 */
function showLoading(show) {
    const loadingDiv = document.querySelector('.loading-state');
    const loadingSpinner = document.querySelector('.loading');
    
    if (loadingDiv) {
        loadingDiv.style.display = show ? 'block' : 'none';
    }
    if (loadingSpinner) {
        loadingSpinner.style.display = show ? 'block' : 'none';
    }
    
    console.log('[LOADING] Loading state:', show ? 'shown' : 'hidden');
}
/**
 * Renders the jobs table with search results
 * 
 * @param {Object} data - Search results data
 * @param {Array} data.jobs - Array of job objects
 * @param {number} data.total - Total number of results
 * @param {number} data.page - Current page number
 * @param {number} data.per_page - Items per page
 */
function renderJobsTable(data) {
    console.log('[TABLE] Rendering jobs table with', data.jobs.length, 'jobs');
    const tbody = document.getElementById('jobs-table-body');
    if (!tbody) {
        console.error('[TABLE] Table body not found!');
        return;
    }
    
    if (data.jobs.length === 0) {
        tbody.innerHTML = '<tr><td colspan="17" class="text-center">No jobs found</td></tr>';
        updateResultsInfo({ jobs: [], total: 0 });
        return;
    }
    
    tbody.innerHTML = data.jobs.map(renderJobRow).join('');
    updateResultsInfo(data);
    renderPagination(data);
    console.log('[TABLE] Table rendered successfully');
}

/**
 * Updates the results information display
 * 
 * @param {Object} data - Search results data
 */
function updateResultsInfo(data) {
    const resultsInfo = document.getElementById('results-info');
    if (resultsInfo) {
        if (data.total === 0) {
            resultsInfo.textContent = 'No results found';
        } else {
            const start = ((data.page - 1) * data.per_page) + 1;
            const end = Math.min(start + data.jobs.length - 1, data.total);
            resultsInfo.textContent = `Showing ${start}-${end} of ${data.total.toLocaleString()} results`;
        }
    }
    
    // Update page stats
    updatePageStats(data.total, data.total);
}

function updatePageStats(total, filtered) {
    const totalStat = document.getElementById('total-jobs-stat');
    const filteredStat = document.getElementById('filtered-jobs-stat');
    
    if (totalStat) totalStat.textContent = total.toLocaleString();
    if (filteredStat) filteredStat.textContent = filtered.toLocaleString();
}

/**
 * Renders a single job row for the table
 * 
 * @param {Object} job - Job data object
 * @returns {string} HTML string for table row
 */
function renderJobRow(job) {
    const criticalBadge = job.critical 
        ? '<span class="badge badge-danger">Yes</span>' 
        : '<span class="badge badge-success">No</span>';
    const cyclicBadge = job.cyclic 
        ? '<span class="badge badge-warning">Yes</span>' 
        : '<span class="badge badge-secondary">No</span>';
    
    return `
        <tr>
            <td><strong>${escapeHtml(job.job_name)}</strong></td>
            <td><span title="${escapeHtml(job.folder_name)}">${escapeHtml(job.folder_name)}</span></td>
            <td>${escapeHtml(job.datacenter || '-')}</td>
            <td>${escapeHtml(job.folder_order_method || '-')}</td>
            <td>${escapeHtml(job.application || '-')}</td>
            <td>${escapeHtml(job.sub_application || '-')}</td>
            <td>${escapeHtml(job.appl_type || '-')}</td>
            <td>${escapeHtml(job.appl_ver || '-')}</td>
            <td>${escapeHtml(job.task_type || '-')}</td>
            <td>${criticalBadge}</td>
            <td>${cyclicBadge}</td>
            <td><span title="${escapeHtml(job.node_id || '')}">${escapeHtml(job.node_id || '-')}</span></td>
            <td>${escapeHtml(job.group || '-')}</td>
            <td>${escapeHtml(job.memname || '-')}</td>
            <td>${escapeHtml(job.owner || '-')}</td>
            <td>${job.maxwait !== null && job.maxwait !== undefined ? job.maxwait : '-'}</td>
            <td>${job.maxrerun !== null && job.maxrerun !== undefined ? job.maxrerun : '-'}</td>
            <td>${escapeHtml(job.shift || '-')}</td>
            <td><span class="badge badge-info">${job.control_resources_count || 0}</span></td>
            <td><span class="badge badge-info">${job.variables_count || 0}</span></td>
            <td><span class="badge badge-success">${job.in_conditions_count || 0}</span></td>
            <td><span class="badge badge-danger">${job.out_conditions_count || 0}</span></td>
            <td><span class="badge badge-warning">${job.on_conditions_count || 0}</span></td>
            <td>
                <button class="btn btn-icon" onclick="viewJobDetail(${job.id})" title="View Details">
                    <i class="fas fa-eye"></i>
                </button>
                <button class="btn btn-icon" onclick="showJobGraph(${job.id})" title="View Graph">
                    <i class="fas fa-project-diagram"></i>
                </button>
            </td>
        </tr>
    `;
}
/**
 * Renders pagination controls
 * 
 * @param {Object} data - Search results data with pagination info
 */
function renderPagination(data) {
    const pagination = document.getElementById('pagination');
    const totalPages = data.total_pages;
    const current = data.page;
    
    // Hide pagination if no results
    if (data.total === 0 || totalPages === 0) {
        pagination.innerHTML = '';
        return;
    }
    
    let html = '';
    
    // Previous button
    html += `<button ${current === 1 ? 'disabled' : ''} onclick="goToPage(${current - 1})">
        <i class="fas fa-chevron-left"></i>
    </button>`;
    
    // Page numbers
    const startPage = Math.max(1, current - 2);
    const endPage = Math.min(totalPages, current + 2);
    
    if (startPage > 1) {
        html += `<button onclick="goToPage(1)">1</button>`;
        if (startPage > 2) html += '<button disabled>...</button>';
    }
    
    for (let i = startPage; i <= endPage; i++) {
        html += `<button class="${i === current ? 'active' : ''}" onclick="goToPage(${i})">${i}</button>`;
    }
    
    if (endPage < totalPages) {
        if (endPage < totalPages - 1) html += '<button disabled>...</button>';
        html += `<button onclick="goToPage(${totalPages})">${totalPages}</button>`;
    }
    
    // Next button
    html += `<button ${current === totalPages ? 'disabled' : ''} onclick="goToPage(${current + 1})">
        <i class="fas fa-chevron-right"></i>
    </button>`;
    
    pagination.innerHTML = html;
}

/**
 * Navigates to a specific page
 * 
 * @param {number} page - Page number to navigate to
 */
function goToPage(page) {
    currentPage = page;
    performSearch();
}

// ============================================================================
// JOB DETAIL MODAL
// ============================================================================

/**
 * Displays detailed information for a job in a modal
 * 
 * @param {number} jobId - ID of the job to display
 * @returns {Promise<void>}
 */
async function viewJobDetail(jobId) {
    const modal = document.getElementById('job-detail-modal');
    
    modal.classList.add('active');
    
    // Reset to first tab
    if (typeof switchTab === 'function') {
        switchTab('basic');
    }
    
    // Show loading in basic tab
    const basicTab = document.getElementById('tab-basic');
    if (basicTab) {
        basicTab.innerHTML = '<div class="loading"><i class="fas fa-spinner fa-spin"></i> Loading...</div>';
    }
    
    try {
        const response = await fetch(`${API_BASE}/jobs/${jobId}`, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
        });
        
        const result = await response.json();
        
        if (result.success) {
            const job = result.data;
            
            // Update modal header
            document.getElementById('modal-job-name').textContent = job.job.job_name;
            const subtitle = document.getElementById('modal-job-subtitle');
            if (subtitle) {
                subtitle.textContent = `${job.job.folder_name} • ${job.job.application || 'N/A'}`;
            }
            
            // Populate all tabs using functions from modal-enhanced.js
            if (typeof populateBasicTab === 'function') populateBasicTab(job);
            if (typeof populateSchedulingTab === 'function') populateSchedulingTab(job);
            if (typeof populateLimitsTab === 'function') populateLimitsTab(job);
            if (typeof populateDependenciesTab === 'function') populateDependenciesTab(job);
            if (typeof populateVariablesTab === 'function') populateVariablesTab(job);
            if (typeof populateMetadataTab === 'function') populateMetadataTab(job);
        }
    } catch (error) {
        console.error('Error loading job details:', error);
        const basicTab = document.getElementById('tab-basic');
        if (basicTab) {
            basicTab.innerHTML = '<div class="error">Failed to load job details</div>';
        }
    }
}

/**
 * Populates the basic tab with job information
 * 
 * @param {Object} job - Job data object
 */
function populateBasicTab(job) {
    const basicTab = document.getElementById('tab-basic');
    const jobName = job.job.job_name;
    const folderName = job.job.folder_name;
    const application = job.job.application || '-';
    const taskType = job.job.task_type || '-';
    const owner = job.job.owner || '-';
    const critical = job.job.critical ? 'Yes' : 'No';
    
    let html = `
        <div class="detail-section">
            <h3>Basic Information</h3>
            <div class="detail-grid">
                <div class="detail-item">
                    <div class="detail-label">Job Name</div>
                    <div class="detail-value">${escapeHtml(jobName)}</div>
                </div>
                <div class="detail-item">
                    <div class="detail-label">Folder</div>
                    <div class="detail-value">${escapeHtml(folderName)}</div>
                </div>
                <div class="detail-item">
                    <div class="detail-label">Application</div>
                    <div class="detail-value">${escapeHtml(application)}</div>
                </div>
                <div class="detail-item">
                    <div class="detail-label">Task Type</div>
                    <div class="detail-value">${escapeHtml(taskType)}</div>
                </div>
                <div class="detail-item">
                    <div class="detail-label">Owner</div>
                    <div class="detail-value">${escapeHtml(owner)}</div>
                </div>
                <div class="detail-item">
                    <div class="detail-label">Critical</div>
                    <div class="detail-value">${critical}</div>
                </div>
            </div>
        </div>
    `;
    
    if (job.job.description) {
        html += `
            <div class="detail-section">
                <h3>Description</h3>
                <p>${escapeHtml(job.job.description)}</p>
            </div>
        `;
    }
    
    if (job.job.cmdline) {
        html += `
            <div class="detail-section">
                <h3>Command Line</h3>
                <pre style="background: #f3f4f6; padding: 12px; border-radius: 6px; overflow-x: auto;">${escapeHtml(job.job.cmdline)}</pre>
            </div>
        `;
    }
    
    basicTab.innerHTML = html;
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/**
 * Escapes HTML special characters to prevent XSS attacks
 * 
 * @param {string} text - Text to escape
 * @returns {string} HTML-escaped text
 */
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// ============================================================================
// COLUMN RESIZE FUNCTIONALITY
// ============================================================================

/**
 * Initializes column resize functionality for the jobs table
 * Allows users to drag column borders to resize columns
 */
function initializeColumnResize() {
    const table = document.getElementById('jobs-table');
    if (!table) return;
    
    const headers = table.querySelectorAll('th');
    headers.forEach((header, index) => {
        // Skip last column (Actions)
        if (index === headers.length - 1) return;
        
        // Add resize handle
        const resizeHandle = document.createElement('div');
        resizeHandle.className = 'resize-handle';
        header.appendChild(resizeHandle);
        
        let startX, startWidth;
        
        resizeHandle.addEventListener('mousedown', (e) => {
            e.preventDefault();
            e.stopPropagation();
            
            startX = e.pageX;
            startWidth = header.offsetWidth;
            
            resizeHandle.classList.add('resizing');
            header.classList.add('resizing');
            document.body.style.cursor = 'col-resize';
            document.body.style.userSelect = 'none';
            
            const onMouseMove = (e) => {
                const diff = e.pageX - startX;
                const newWidth = Math.max(100, startWidth + diff);
                header.style.width = newWidth + 'px';
                header.style.minWidth = newWidth + 'px';
                
                // Update corresponding td cells
                const columnIndex = Array.from(header.parentElement.children).indexOf(header);
                const rows = table.querySelectorAll('tbody tr');
                rows.forEach(row => {
                    const cell = row.children[columnIndex];
                    if (cell) {
                        cell.style.width = newWidth + 'px';
                        cell.style.minWidth = newWidth + 'px';
                        cell.style.maxWidth = newWidth + 'px';
                    }
                });
            };
            
            const onMouseUp = () => {
                resizeHandle.classList.remove('resizing');
                header.classList.remove('resizing');
                document.body.style.cursor = '';
                document.body.style.userSelect = '';
                
                document.removeEventListener('mousemove', onMouseMove);
                document.removeEventListener('mouseup', onMouseUp);
            };
            
            document.addEventListener('mousemove', onMouseMove);
            document.addEventListener('mouseup', onMouseUp);
        });
    });
}

// ============================================================================
// CSV EXPORT FUNCTIONALITY
// ============================================================================

/**
 * Exports current search results to CSV file
 * Applies the same filters as the current search
 * 
 * @returns {Promise<void>}
 */
async function exportToCSV() {
    const startTime = performance.now();
    console.log('📥 [EXPORT] Starting CSV export...');
    showLoading(true);
    console.log("[SEARCH] Starting search...");
    
    // Update loading text if element exists
    const loadingText = document.querySelector('#loading-overlay .loading-text');
    if (loadingText) {
        loadingText.textContent = 'Exporting to CSV...';
    }
    
    const jobName = document.getElementById('filter-job-name').value;
    const folder = $('#filter-folder').val();
    const application = $('#filter-application').val();
    const applType = $('#filter-appl-type').val();
    const applVer = $('#filter-appl-ver').val();
    const taskType = document.getElementById('filter-task-type').value;
    const critical = document.getElementById('filter-critical').value;
    
    const filters = {};
    if (jobName) filters.job_name = jobName;
    if (folder) filters.folder_name = folder;
    if (application) filters.application = application;
    if (applType) filters.appl_type = applType;
    if (applVer) filters.appl_ver = applVer;
    if (taskType) filters.task_type = taskType;
    if (critical) filters.critical = critical === 'true';
    
    console.log('📋 [EXPORT] Filters:', filters);
    
    try {
        console.log('🌐 [EXPORT] Sending export request...');
        const fetchStart = performance.now();
        
        const response = await fetch(`${API_BASE}/jobs/export`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${authToken}`
            },
            body: JSON.stringify(filters)
        });
        
        const fetchEnd = performance.now();
        console.log(`⏱️  [EXPORT] Response received in ${(fetchEnd - fetchStart).toFixed(2)}ms`);
        
        if (response.ok) {
            console.log('📦 [EXPORT] Creating blob...');
            const blob = await response.blob();
            console.log(`📊 [EXPORT] CSV size: ${(blob.size / 1024).toFixed(2)} KB`);
            
            console.log('💾 [EXPORT] Triggering download...');
            const url = window.URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `jobs_export_${new Date().toISOString().split('T')[0]}.csv`;
            document.body.appendChild(a);
            a.click();
            window.URL.revokeObjectURL(url);
            document.body.removeChild(a);
            
            console.log('✅ [EXPORT] CSV exported successfully');
        } else {
            console.error('❌ [EXPORT] Export failed with status:', response.status);
            alert('Failed to export CSV');
        }
    } catch (error) {
        console.error('❌ [EXPORT] Error:', error);
        alert('Failed to export CSV');
    } finally {
        showLoading(false);
        
        // Reset loading text if element exists
        const loadingText = document.querySelector('#loading-overlay .loading-text');
        if (loadingText) {
            loadingText.textContent = 'Searching jobs...';
        }
        
        const endTime = performance.now();
        console.log(`🏁 [EXPORT] Total export time: ${(endTime - startTime).toFixed(2)}ms`);
        console.log('─────────────────────────────────────');
    }
}

// Job Dependency Graph
let currentNetwork = null;
let currentGraphJobId = null;
let currentGraphMode = 'direct'; // 'direct' or 'e2e'

async function showJobGraph(jobId) {
    currentGraphJobId = jobId;
    currentGraphMode = 'direct';
    
    // Reset UI
    document.getElementById('btn-direct-graph').classList.add('active');
    document.getElementById('btn-e2e-graph').classList.remove('active');
    document.getElementById('depth-selector').style.display = 'none';
    
    const modal = document.getElementById('graph-modal');
    const graphContainer = document.getElementById('graph-container');
    
    modal.classList.add('active');
    graphContainer.innerHTML = '<div style="display: flex; align-items: center; justify-content: center; height: 100%;"><div class="loading"><i class="fas fa-spinner fa-spin"></i> Loading graph...</div></div>';
    
    await loadGraph(jobId, 'direct');
}

async function loadGraph(jobId, mode, depth = null) {
    const graphContainer = document.getElementById('graph-container');
    
    try {
        let url = `${API_BASE}/jobs/${jobId}/graph`;
        if (mode === 'e2e') {
            url = `${API_BASE}/jobs/${jobId}/graph/end-to-end`;
            if (depth) {
                url += `?depth=${depth}`;
            }
        }
        
        const response = await fetch(url, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
        });
        
        if (!response.ok) {
            throw new Error('Failed to load graph data');
        }
        
        const result = await response.json();
        
        if (result.success && result.data) {
            renderJobGraph(result.data, mode);
        } else {
            graphContainer.innerHTML = '<div style="padding: 40px; text-align: center;"><p class="error">Failed to load graph data</p></div>';
        }
    } catch (error) {
        console.error('Error loading job graph:', error);
        graphContainer.innerHTML = '<div style="padding: 40px; text-align: center;"><p class="error">Error loading graph: ' + error.message + '</p></div>';
    }
}

function toggleGraphMode(mode) {
    currentGraphMode = mode;
    
    // Update button states
    if (mode === 'direct') {
        document.getElementById('btn-direct-graph').classList.add('active');
        document.getElementById('btn-e2e-graph').classList.remove('active');
        document.getElementById('depth-selector').style.display = 'none';
    } else {
        document.getElementById('btn-direct-graph').classList.remove('active');
        document.getElementById('btn-e2e-graph').classList.add('active');
        document.getElementById('depth-selector').style.display = 'flex';
    }
    
    // Reload graph with new mode
    reloadGraph();
}

function reloadGraph() {
    if (!currentGraphJobId) return;
    
    const graphContainer = document.getElementById('graph-container');
    graphContainer.innerHTML = '<div style="display: flex; align-items: center; justify-content: center; height: 100%;"><div class="loading"><i class="fas fa-spinner fa-spin"></i> Loading graph...</div></div>';
    
    const depth = currentGraphMode === 'e2e' ? parseInt(document.getElementById('graph-depth').value) : null;
    loadGraph(currentGraphJobId, currentGraphMode, depth);
}

function renderJobGraph(graphData, mode = 'direct') {
    const graphContainer = document.getElementById('graph-container');
    
    // Update header info
    document.getElementById('graph-job-name').textContent = graphData.job_name;
    document.getElementById('graph-job-folder').textContent = graphData.folder_name;
    
    const depsIn = graphData.edges.filter(e => e.edge_type === 'in').length;
    const depsOut = graphData.edges.filter(e => e.edge_type === 'out').length;
    
    document.getElementById('stat-nodes').textContent = graphData.nodes.length;
    document.getElementById('stat-edges').textContent = graphData.edges.length;
    document.getElementById('stat-deps-in').textContent = depsIn;
    document.getElementById('stat-deps-out').textContent = depsOut;
    
    const nodes = new vis.DataSet(graphData.nodes.map(node => {
        // Build tooltip with application and description using newline characters
        let tooltipParts = [
            `${node.label}`,
            `Folder: ${node.folder}`
        ];
        
        if (node.application) {
            tooltipParts.push(`Application: ${node.application}`);
        }
        
        if (node.description) {
            tooltipParts.push('');  // Empty line for spacing
            tooltipParts.push(`${node.description}`);
        }
        
        const tooltip = tooltipParts.join('\n');
        
        return {
            id: node.id,
            label: node.label,
            color: {
                background: node.color,
                border: node.is_current ? '#2d5016' : (node.color === '#2196F3' ? '#0d47a1' : '#e65100'),
                highlight: {
                    background: node.color,
                    border: '#000000'
                }
            },
            font: { 
                size: 16, 
                color: '#ffffff',
                face: 'Arial',
                bold: node.is_current
            },
            title: tooltip,
            borderWidth: node.is_current ? 4 : 2,
            shadow: true
        };
    }));
    
    const edges = new vis.DataSet(graphData.edges.map(edge => ({
        from: edge.from,
        to: edge.to,
        arrows: {
            to: {
                enabled: true,
                scaleFactor: 1.2
            }
        },
        color: { 
            color: edge.edge_type === 'in' ? '#2196F3' : '#FF9800',
            highlight: '#000000'
        },
        width: 3,
        smooth: {
            type: 'cubicBezier',
            roundness: 0.5
        }
    })));
    
    const data = { nodes, edges };
    
    // Use hierarchical layout for end-to-end mode
    const options = mode === 'e2e' ? {
        layout: {
            hierarchical: {
                enabled: true,
                direction: 'LR', // Left to Right
                sortMethod: 'directed',
                levelSeparation: 200,
                nodeSpacing: 150,
                treeSpacing: 200
            }
        },
        physics: {
            enabled: false // Disable physics for hierarchical layout
        },
        edges: {
            smooth: {
                type: 'cubicBezier',
                forceDirection: 'horizontal',
                roundness: 0.4
            }
        },
        interaction: {
            dragNodes: true,
            dragView: true,
            zoomView: true
        }
    } : {
        layout: {
            randomSeed: 42,
            improvedLayout: true
        },
        physics: {
            enabled: true,
            stabilization: {
                enabled: true,
                iterations: 200,
                updateInterval: 25
            },
            barnesHut: {
                gravitationalConstant: -8000,
                centralGravity: 0.3,
                springLength: 200,
                springConstant: 0.04,
                damping: 0.09,
                avoidOverlap: 0.5
            }
        },
        interaction: {
            dragNodes: true,
            dragView: true,
            zoomView: true,
            hover: true,
            tooltipDelay: 100,
            navigationButtons: false
        },
        nodes: {
            shape: 'box',
            margin: 15,
            widthConstraint: {
                minimum: 150,
                maximum: 250
            },
            heightConstraint: {
                minimum: 40
            }
        }
    };
    
    if (currentNetwork) {
        currentNetwork.destroy();
    }
    
    currentNetwork = new vis.Network(graphContainer, data, options);
    
    currentNetwork.once('stabilizationIterationsDone', function() {
        currentNetwork.fit({
            animation: {
                duration: 1000,
                easingFunction: 'easeInOutQuad'
            }
        });
    });
}

function closeGraphModal() {
    const modal = document.getElementById('graph-modal');
    modal.classList.remove('active');
    
    if (currentNetwork) {
        currentNetwork.destroy();
        currentNetwork = null;
    }
}

function fitGraphToScreen() {
    if (currentNetwork) {
        currentNetwork.fit({
            animation: {
                duration: 500,
                easingFunction: 'easeInOutQuad'
            }
        });
    }
}

function zoomIn() {
    if (currentNetwork) {
        const scale = currentNetwork.getScale();
        currentNetwork.moveTo({
            scale: scale * 1.2,
            animation: {
                duration: 300,
                easingFunction: 'easeInOutQuad'
            }
        });
    }
}

function zoomOut() {
    if (currentNetwork) {
        const scale = currentNetwork.getScale();
        currentNetwork.moveTo({
            scale: scale * 0.8,
            animation: {
                duration: 300,
                easingFunction: 'easeInOutQuad'
            }
        });
    }
}

function resetGraph() {
    if (currentNetwork) {
        currentNetwork.moveTo({
            position: {x: 0, y: 0},
            scale: 1.0,
            animation: {
                duration: 500,
                easingFunction: 'easeInOutQuad'
            }
        });
    }
}

function closeJobDetailModal() {
    const modal = document.getElementById('job-detail-modal');
    if (modal) {
        modal.classList.remove('active');
    }
}

// Make functions global for onclick handlers
window.goToPage = goToPage;
window.viewJobDetail = viewJobDetail;
window.showJobGraph = showJobGraph;
window.closeGraphModal = closeGraphModal;
window.closeJobDetailModal = closeJobDetailModal;
window.fitGraphToScreen = fitGraphToScreen;
window.zoomIn = zoomIn;
window.toggleGraphMode = toggleGraphMode;
window.reloadGraph = reloadGraph;
window.zoomOut = zoomOut;
window.resetGraph = resetGraph;

// Filter collapse toggle
document.addEventListener('DOMContentLoaded', () => {
    const toggleBtn = document.getElementById('toggle-filters');
    const filterBody = document.getElementById('filter-body');
    
    if (toggleBtn && filterBody) {
        toggleBtn.addEventListener('click', () => {
            filterBody.classList.toggle('collapsed');
            toggleBtn.classList.toggle('collapsed');
        });
    }
});

// Update stats
function updatePageStats(total, filtered) {
    const totalStat = document.getElementById('total-jobs-stat');
    const filteredStat = document.getElementById('filtered-jobs-stat');
    
    if (totalStat) totalStat.textContent = total.toLocaleString();
    if (filteredStat) filteredStat.textContent = filtered.toLocaleString();
}

// Show active filters
function showActiveFilters() {
    const activeFiltersDiv = document.getElementById('active-filters');
    const filterChipsDiv = document.getElementById('filter-chips');
    
    if (!activeFiltersDiv || !filterChipsDiv) return;
    
    const chips = [];
    
    if (currentFilters.job_name) {
        chips.push(`Job: "${currentFilters.job_name}"`);
    }
    if (currentFilters.folder_name) {
        chips.push(`Folder: ${currentFilters.folder_name}`);
    }
    if (currentFilters.application) {
        chips.push(`App: ${currentFilters.application}`);
    }
    if (currentFilters.task_type) {
        chips.push(`Task: ${currentFilters.task_type}`);
    }
    if (currentFilters.critical !== undefined) {
        chips.push(`Critical: ${currentFilters.critical ? 'Yes' : 'No'}`);
    }
    if (currentFilters.min_dependencies) {
        chips.push(`Min Deps: ${currentFilters.min_dependencies}`);
    }
    
    if (chips.length > 0) {
        filterChipsDiv.innerHTML = chips.map(chip => 
            `<span class="filter-chip">${chip}</span>`
        ).join('');
        activeFiltersDiv.style.display = 'flex';
    } else {
        activeFiltersDiv.style.display = 'none';
    }
}
