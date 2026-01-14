// API Configuration
const API_BASE = '/api';
let authToken = null;
let currentUser = null;
let currentPage = 1;
let currentPerPage = 50;
let currentSort = { by: 'job_name', order: 'asc' };
let currentFilters = {};

// Initialize App
document.addEventListener('DOMContentLoaded', () => {
    checkAuth();
    initializeEventListeners();
});

// Authentication
function checkAuth() {
    authToken = localStorage.getItem('authToken');
    if (authToken) {
        fetchCurrentUser();
    } else {
        showPage('login-page');
    }
}

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

function showPage(pageId) {
    document.querySelectorAll('.page').forEach(page => {
        page.classList.remove('active');
    });
    const targetPage = document.getElementById(pageId);
    if (targetPage) {
        targetPage.classList.add('active');
    }
}

// Event Listeners
function initializeEventListeners() {
    initializeLoginListeners();
    initializeNavigationListeners();
    initializeSearchListeners();
    initializeTableListeners();
    initializeModalListeners();
}

function initializeLoginListeners() {
    initializeLoginTabs();
    initializeLoginForm();
    initializeEntraLogin();
    initializeLogout();
}

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

function initializeEntraLogin() {
    const entraLoginBtn = document.getElementById('entra-login-btn');
    if (entraLoginBtn) {
        entraLoginBtn.addEventListener('click', handleEntraLogin);
    }
}

function initializeLogout() {
    const logoutBtn = document.getElementById('logout-btn');
    if (logoutBtn) {
        logoutBtn.addEventListener('click', logout);
    }
}

function initializeNavigationListeners() {
    document.querySelectorAll('.nav-item').forEach(item => {
        item.addEventListener('click', (e) => {
            e.preventDefault();
            const page = item.dataset.page;
            switchContentPage(page);
        });
    });
}

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

function initializeTableListeners() {
    document.querySelectorAll('.data-table th[data-sort]').forEach(th => {
        th.addEventListener('click', () => {
            handleTableSort(th.dataset.sort);
        });
    });
}

function handleTableSort(sortBy) {
    if (currentSort.by === sortBy) {
        currentSort.order = currentSort.order === 'asc' ? 'desc' : 'asc';
    } else {
        currentSort.by = sortBy;
        currentSort.order = 'asc';
    }
    performSearch();
}

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

function closeAllModals() {
    document.querySelectorAll('.modal').forEach(m => m.classList.remove('active'));
}

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

function logout() {
    authToken = null;
    currentUser = null;
    localStorage.removeItem('authToken');
    showPage('login-page');
}

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
        loadDashboard();
    } else if (page === 'jobs') {
        loadFilterOptions();
        performSearch();
    }
}

// Dashboard
async function loadDashboard() {
    const startTime = performance.now();
    console.log('📊 [DASHBOARD] Loading dashboard statistics...');
    
    try {
        const fetchStart = performance.now();
        const response = await fetch(`${API_BASE}/dashboard/stats`, {
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

// Jobs Search
function initializeSelect2() {
    $('.select2-dropdown').select2({
        placeholder: 'Select an option',
        allowClear: true,
        width: '100%'
    });
}

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
            
            console.log('🎨 [FILTERS] Populating dropdowns...');
            populateSelect('filter-folder', options.folders);
            populateSelect('filter-application', options.applications);
            populateSelect('filter-appl-type', options.appl_types);
            populateSelect('filter-appl-ver', options.appl_vers);
            populateSelect('filter-task-type', options.task_types);
            
            console.log('🔍 [FILTERS] Initializing Select2...');
            initializeSelect2();
            
            const endTime = performance.now();
            console.log(`✅ [FILTERS] Filters loaded in ${(endTime - startTime).toFixed(2)}ms`);
        }
    } catch (error) {
        console.error('❌ [FILTERS] Failed to load filter options:', error);
    }
}

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
    document.getElementById('filter-has-variables').value = '';
    document.getElementById('filter-min-variables').value = '';
    currentPage = 1;
    performSearch();
}

async function performSearch() {
    const startTime = performance.now();
    console.log('🔍 [SEARCH] Starting search operation...');
    
    showLoading(true); console.log("[SEARCH] Starting search...");
    
    currentFilters = {};
    
    // Get filter values (use jQuery for Select2 dropdowns)
    const jobName = document.getElementById('filter-job-name')?.value?.trim();
    const folderName = $('#filter-folder').val(); // Select2
    const application = $('#filter-application').val(); // Select2
    const applType = $('#filter-appl-type').val(); // Select2
    const applVer = $('#filter-appl-ver').val(); // Select2
    const taskType = $('#filter-task-type').val(); // Select2
    const critical = document.getElementById('filter-critical')?.value;
    const minDeps = document.getElementById('filter-min-deps')?.value?.trim();
    const maxDeps = document.getElementById('filter-max-deps')?.value?.trim();
    const hasVars = document.getElementById('filter-has-variables')?.value;
    const minVars = document.getElementById('filter-min-variables')?.value?.trim();
    
    console.log('🔍 [SEARCH] Raw filter values:', {
        jobName, folderName, application, applType, applVer, taskType, 
        critical, minDeps, maxDeps, hasVars, minVars
    });
    
    // Build filters object
    if (jobName) currentFilters.job_name = jobName;
    if (folderName) currentFilters.folder_name = folderName;
    if (application) currentFilters.application = application;
    if (applType) currentFilters.appl_type = applType;
    if (applVer) currentFilters.appl_ver = applVer;
    if (taskType) currentFilters.task_type = taskType;
    if (critical && critical !== '') currentFilters.critical = critical === 'true';
    if (minDeps) currentFilters.min_dependencies = parseInt(minDeps);
    if (maxDeps) currentFilters.max_dependencies = parseInt(maxDeps);
    if (hasVars && hasVars !== '') currentFilters.has_variables = hasVars === 'true';
    if (minVars) currentFilters.min_variables = parseInt(minVars);
    
    console.log('📋 [SEARCH] Filters to send:', currentFilters);
    console.log('📄 [SEARCH] Page:', currentPage, 'Per page:', currentPerPage);
    
    try {
        console.log('🌐 [SEARCH] Sending API request...');
        const fetchStart = performance.now();
        
        const response = await fetch(`${API_BASE}/jobs/search`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${authToken}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                ...currentFilters,
                page: currentPage,
                per_page: currentPerPage,
                sort_by: currentSort.by,
                sort_order: currentSort.order
            })
        });
        
        const fetchEnd = performance.now();
        console.log(`⏱️  [SEARCH] API response received in ${(fetchEnd - fetchStart).toFixed(2)}ms`);
        
        console.log('📦 [SEARCH] Parsing response...');
        const result = await response.json();
        
        if (result.success) {
            console.log(`✅ [SEARCH] Found ${result.data.total} jobs (showing ${result.data.jobs.length})`);
            console.log('🎨 [SEARCH] Rendering table...');
            const renderStart = performance.now();
            
            renderJobsTable(result.data);
            
            const renderEnd = performance.now();
            console.log(`✅ [SEARCH] Table rendered in ${(renderEnd - renderStart).toFixed(2)}ms`);
        } else {
            console.error('❌ [SEARCH] Search failed:', result.error);
        }
    } catch (error) {
        console.error('❌ [SEARCH] Error:', error);
    } finally {
        showLoading(false);
        const endTime = performance.now();
        console.log(`🏁 [SEARCH] Total search time: ${(endTime - startTime).toFixed(2)}ms`);
        console.log('─────────────────────────────────────');
    }
}

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
function renderJobsTable(data) {
    console.log('[TABLE] Rendering jobs table with', data.jobs.length, 'jobs');
    const tbody = document.getElementById('jobs-table-body');
    if (!tbody) {
        console.error('[TABLE] Table body not found!');
        return;
    }
    
    if (data.jobs.length === 0) {
        tbody.innerHTML = '<tr><td colspan="14" class="text-center">No jobs found</td></tr>';
        updateResultsInfo({ jobs: [], total: 0 });
        return;
    }
    
    tbody.innerHTML = data.jobs.map(renderJobRow).join('');
    updateResultsInfo(data);
    renderPagination(data);
    initializeTableFeatures();
    console.log('[TABLE] Table rendered successfully');
}

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

function renderJobRow(job) {
    const criticalBadge = job.critical 
        ? '<span class="badge badge-danger">Yes</span>' 
        : '<span class="badge badge-success">No</span>';
    
    return `
        <tr>
            <td><strong>${escapeHtml(job.job_name)}</strong></td>
            <td><span title="${escapeHtml(job.folder_name)}">${escapeHtml(job.folder_name)}</span></td>
            <td>${escapeHtml(job.application || '-')}</td>
            <td>${escapeHtml(job.sub_application || '-')}</td>
            <td>${escapeHtml(job.appl_type || '-')}</td>
            <td>${escapeHtml(job.appl_ver || '-')}</td>
            <td>${escapeHtml(job.task_type || '-')}</td>
            <td>${criticalBadge}</td>
            <td>${escapeHtml(job.owner || '-')}</td>
            <td><span class="badge badge-info">${job.control_resources_count || 0}</span></td>
            <td><span class="badge badge-info">${job.variables_count || 0}</span></td>
            <td><span class="badge badge-success">${job.in_conditions_count || 0}</span></td>
            <td><span class="badge badge-danger">${job.out_conditions_count || 0}</span></td>
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

function goToPage(page) {
    currentPage = page;
    performSearch();
}

// Job Detail Modal
async function viewJobDetail(jobId) {
    const modal = document.getElementById('job-detail-modal');
    const modalBody = document.getElementById('modal-job-details');
    
    modal.classList.add('active');
    modalBody.innerHTML = '<div class="loading"><i class="fas fa-spinner fa-spin"></i> Loading...</div>';
    
    try {
        const response = await fetch(`${API_BASE}/jobs/${jobId}`, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
        });
        
        const result = await response.json();
        
        if (result.success) {
            renderJobDetail(result.data);
        } else {
            modalBody.innerHTML = '<p class="text-center text-muted">Failed to load job details</p>';
        }
    } catch (error) {
        console.error('Failed to load job detail:', error);
        modalBody.innerHTML = '<p class="text-center text-muted">Failed to load job details</p>';
    }
}

function renderJobDetail(data) {
    const job = data.job;
    const modalName = document.getElementById('modal-job-name');
    const modalBody = document.getElementById('modal-job-details');
    
    modalName.textContent = job.job_name;
    
    let html = `
        <div class="detail-section">
            <h3>Basic Information</h3>
            <div class="detail-grid">
                <div class="detail-item">
                    <div class="detail-label">Job Name</div>
                    <div class="detail-value">${escapeHtml(job.job_name)}</div>
                </div>
                <div class="detail-item">
                    <div class="detail-label">Folder</div>
                    <div class="detail-value">${escapeHtml(job.folder_name)}</div>
                </div>
                <div class="detail-item">
                    <div class="detail-label">Application</div>
                    <div class="detail-value">${escapeHtml(job.application || '-')}</div>
                </div>
                <div class="detail-item">
                    <div class="detail-label">Task Type</div>
                    <div class="detail-value">${escapeHtml(job.task_type || '-')}</div>
                </div>
                <div class="detail-item">
                    <div class="detail-label">Owner</div>
                    <div class="detail-value">${escapeHtml(job.owner || '-')}</div>
                </div>
                <div class="detail-item">
                    <div class="detail-label">Critical</div>
                    <div class="detail-value">${job.critical ? 'Yes' : 'No'}</div>
                </div>
            </div>
        </div>
    `;
    
    if (job.description) {
        html += `
            <div class="detail-section">
                <h3>Description</h3>
                <p>${escapeHtml(job.description)}</p>
            </div>
        `;
    }
    
    if (job.cmdline) {
        html += `
            <div class="detail-section">
                <h3>Command Line</h3>
                <pre style="background: #f3f4f6; padding: 12px; border-radius: 6px; overflow-x: auto;">${escapeHtml(job.cmdline)}</pre>
            </div>
        `;
    }
    
    if (data.in_conditions && data.in_conditions.length > 0) {
        html += `
            <div class="detail-section">
                <h3>In Conditions (${data.in_conditions.length})</h3>
                <ul class="detail-list">
                    ${data.in_conditions.map(c => `<li>${escapeHtml(c.condition_name)}</li>`).join('')}
                </ul>
            </div>
        `;
    }
    
    if (data.out_conditions && data.out_conditions.length > 0) {
        html += `
            <div class="detail-section">
                <h3>Out Conditions (${data.out_conditions.length})</h3>
                <ul class="detail-list">
                    ${data.out_conditions.map(c => `<li>${escapeHtml(c.condition_name)}</li>`).join('')}
                </ul>
            </div>
        `;
    }
    
    if (data.variables && data.variables.length > 0) {
        html += `
            <div class="detail-section">
                <h3>Variables (${data.variables.length})</h3>
                <ul class="detail-list">
                    ${data.variables.map(v => `<li><strong>${escapeHtml(v.name)}:</strong> ${escapeHtml(v.value)}</li>`).join('')}
                </ul>
            </div>
        `;
    }
    
    modalBody.innerHTML = html;
}

// Utility Functions
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// Column Resize Functionality
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

async function exportToCSV() {
    const startTime = performance.now();
    console.log('📥 [EXPORT] Starting CSV export...');
    showLoading(true); console.log("[SEARCH] Starting search...");
    document.querySelector('#loading-overlay .loading-spinner p').textContent = 'Exporting to CSV...';
    
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
    
    const params = new URLSearchParams(filters);
    
    try {
        console.log('🌐 [EXPORT] Sending export request...');
        const fetchStart = performance.now();
        
        const response = await fetch(`${API_BASE}/jobs/export/csv?${params}`, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
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
        document.querySelector('#loading-overlay .loading-spinner p').textContent = 'Searching jobs...';
        const endTime = performance.now();
        console.log(`🏁 [EXPORT] Total export time: ${(endTime - startTime).toFixed(2)}ms`);
        console.log('─────────────────────────────────────');
    }
}

// Job Dependency Graph
let currentNetwork = null;

async function showJobGraph(jobId) {
    const modal = document.getElementById('graph-modal');
    const graphContainer = document.getElementById('graph-container');
    
    modal.classList.add('active');
    graphContainer.innerHTML = '<div style="display: flex; align-items: center; justify-content: center; height: 100%;"><div class="loading"><i class="fas fa-spinner fa-spin"></i> Loading graph...</div></div>';
    
    try {
        const response = await fetch(`${API_BASE}/jobs/${jobId}/graph`, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
        });
        
        if (!response.ok) {
            throw new Error('Failed to load graph data');
        }
        
        const result = await response.json();
        
        if (result.success && result.data) {
            renderJobGraph(result.data);
        } else {
            graphContainer.innerHTML = '<div style="padding: 40px; text-align: center;"><p class="error">Failed to load graph data</p></div>';
        }
    } catch (error) {
        console.error('Error loading job graph:', error);
        graphContainer.innerHTML = '<div style="padding: 40px; text-align: center;"><p class="error">Error loading graph: ' + error.message + '</p></div>';
    }
}

function renderJobGraph(graphData) {
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
    
    const nodes = new vis.DataSet(graphData.nodes.map(node => ({
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
        title: `<b>${node.label}</b><br/>Folder: ${node.folder}`,
        borderWidth: node.is_current ? 4 : 2,
        shadow: true
    })));
    
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
    
    const options = {
        layout: {
            hierarchical: {
                direction: 'UD',
                sortMethod: 'directed',
                nodeSpacing: 200,
                levelSeparation: 250,
                treeSpacing: 250
            }
        },
        physics: {
            enabled: false
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
