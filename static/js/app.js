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
            document.getElementById('user-display-name').textContent = currentUser.display_name;
            showPage('main-app');
            loadDashboard();
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
    document.getElementById(pageId).classList.add('active');
}

// Event Listeners
function initializeEventListeners() {
    // Login tabs
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            const tab = btn.dataset.tab;
            document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
            btn.classList.add('active');
            document.getElementById(`${tab}-login`).classList.add('active');
        });
    });

    // Login form
    document.getElementById('login-form').addEventListener('submit', handleLogin);
    
    // Entra ID login
    document.getElementById('entra-login-btn').addEventListener('click', handleEntraLogin);
    
    // Logout
    document.getElementById('logout-btn').addEventListener('click', logout);
    
    // Navigation
    document.querySelectorAll('.nav-item').forEach(item => {
        item.addEventListener('click', (e) => {
            e.preventDefault();
            const page = item.dataset.page;
            switchContentPage(page);
        });
    });
    
    // Search and filters
    document.getElementById('search-btn').addEventListener('click', performSearch);
    document.getElementById('reset-btn').addEventListener('click', resetFilters);
    document.getElementById('export-csv-btn').addEventListener('click', exportToCSV);
    document.getElementById('per-page').addEventListener('change', (e) => {
        currentPerPage = parseInt(e.target.value);
        performSearch();
    });
    
    // Table sorting
    document.querySelectorAll('.data-table th[data-sort]').forEach(th => {
        th.addEventListener('click', () => {
            const sortBy = th.dataset.sort;
            if (currentSort.by === sortBy) {
                currentSort.order = currentSort.order === 'asc' ? 'desc' : 'asc';
            } else {
                currentSort.by = sortBy;
                currentSort.order = 'asc';
            }
            performSearch();
        });
    });
    
    // Modal close
    document.querySelectorAll('.modal-close').forEach(btn => {
        btn.addEventListener('click', () => {
            document.querySelectorAll('.modal').forEach(m => m.classList.remove('active'));
        });
    });
    
    // Click outside modal to close
    document.querySelectorAll('.modal').forEach(modal => {
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                modal.classList.remove('active');
            }
        });
    });
}

async function handleLogin(e) {
    e.preventDefault();
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    const errorDiv = document.getElementById('login-error');
    
    try {
        const response = await fetch(`${API_BASE}/auth/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username, password })
        });
        
        const result = await response.json();
        
        if (result.success) {
            authToken = result.data.token;
            currentUser = result.data.user;
            localStorage.setItem('authToken', authToken);
            document.getElementById('user-display-name').textContent = currentUser.display_name;
            showPage('main-app');
            loadDashboard();
        } else {
            errorDiv.textContent = result.error || 'Login failed';
            errorDiv.style.display = 'block';
        }
    } catch (error) {
        errorDiv.textContent = 'Network error. Please try again.';
        errorDiv.style.display = 'block';
    }
}

async function handleEntraLogin() {
    alert('Entra ID authentication would redirect to Microsoft login page.\nFor demo purposes, this is not implemented.');
}

function logout() {
    authToken = null;
    currentUser = null;
    localStorage.removeItem('authToken');
    showPage('login-page');
}

function switchContentPage(page) {
    document.querySelectorAll('.nav-item').forEach(item => {
        item.classList.remove('active');
    });
    document.querySelectorAll('.content-page').forEach(p => {
        p.classList.remove('active');
    });
    
    document.querySelector(`[data-page="${page}"]`).classList.add('active');
    document.getElementById(`${page}-page`).classList.add('active');
    
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
            document.getElementById('stat-total-jobs').textContent = stats.total_jobs.toLocaleString();
            document.getElementById('stat-total-folders').textContent = stats.total_folders.toLocaleString();
            document.getElementById('stat-critical-jobs').textContent = stats.critical_jobs.toLocaleString();
            document.getElementById('stat-cyclic-jobs').textContent = stats.cyclic_jobs.toLocaleString();
            document.getElementById('stat-file-transfer').textContent = stats.file_transfer_jobs.toLocaleString();
            document.getElementById('stat-cli-jobs').textContent = stats.cli_jobs.toLocaleString();
            
            console.log('📊 [DASHBOARD] Rendering charts...');
            renderBarChart('chart-applications', stats.jobs_by_application, 'application', 'count');
            renderBarChart('chart-folders', stats.jobs_by_folder, 'folder_name', 'job_count');
            renderBarChart('chart-task-types', stats.jobs_by_task_type, 'task_type', 'count');
            renderComplexityChart('chart-complexity', stats.complexity_distribution);
            
            const endTime = performance.now();
            console.log(`✅ [DASHBOARD] Dashboard loaded in ${(endTime - startTime).toFixed(2)}ms`);
        }
    } catch (error) {
        console.error('❌ [DASHBOARD] Failed to load dashboard:', error);
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
    document.getElementById('filter-job-name').value = '';
    $('#filter-folder').val('').trigger('change');
    $('#filter-application').val('').trigger('change');
    $('#filter-appl-type').val('').trigger('change');
    $('#filter-appl-ver').val('').trigger('change');
    document.getElementById('filter-task-type').value = '';
    document.getElementById('filter-critical').value = '';
    currentFilters = {};
    currentPage = 1;
    performSearch();
}

async function performSearch() {
    const startTime = performance.now();
    console.log('🔍 [SEARCH] Starting search operation...');
    showLoading(true);
    
    const jobName = document.getElementById('filter-job-name').value;
    const folder = $('#filter-folder').val();
    const application = $('#filter-application').val();
    const applType = $('#filter-appl-type').val();
    const applVer = $('#filter-appl-ver').val();
    const taskType = document.getElementById('filter-task-type').value;
    const critical = document.getElementById('filter-critical').value;
    
    currentFilters = {};
    if (jobName) currentFilters.job_name = jobName;
    if (folder) currentFilters.folder_name = folder;
    if (application) currentFilters.application = application;
    if (applType) currentFilters.appl_type = applType;
    if (applVer) currentFilters.appl_ver = applVer;
    if (taskType) currentFilters.task_type = taskType;
    if (critical) currentFilters.critical = critical === 'true';
    
    console.log('📋 [SEARCH] Filters:', currentFilters);
    console.log('📄 [SEARCH] Page:', currentPage, 'Per page:', currentPerPage);
    
    const params = new URLSearchParams({
        ...currentFilters,
        page: currentPage,
        per_page: currentPerPage,
        sort_by: currentSort.by,
        sort_order: currentSort.order
    });
    
    try {
        console.log('🌐 [SEARCH] Sending API request...');
        const fetchStart = performance.now();
        
        const response = await fetch(`${API_BASE}/jobs/search?${params}`, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
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
    const overlay = document.getElementById('loading-overlay');
    if (show) {
        overlay.classList.add('active');
    } else {
        overlay.classList.remove('active');
    }
}

function renderJobsTable(data) {
    const tbody = document.getElementById('jobs-table-body');
    const resultsInfo = document.getElementById('results-info');
    
    resultsInfo.textContent = `Showing ${data.jobs.length} of ${data.total} results`;
    
    if (data.jobs.length === 0) {
        tbody.innerHTML = '<tr><td colspan="7" class="text-center">No jobs found</td></tr>';
        return;
    }
    
    tbody.innerHTML = data.jobs.map(job => `
        <tr>
            <td title="${escapeHtml(job.job_name)}"><strong>${escapeHtml(job.job_name)}</strong></td>
            <td title="${escapeHtml(job.folder_name)}">${escapeHtml(job.folder_name)}</td>
            <td title="${escapeHtml(job.application || '-')}">${escapeHtml(job.application || '-')}</td>
            <td title="${escapeHtml(job.appl_type || '-')}">${escapeHtml(job.appl_type || '-')}</td>
            <td title="${escapeHtml(job.appl_ver || '-')}">${escapeHtml(job.appl_ver || '-')}</td>
            <td title="${escapeHtml(job.task_type || '-')}">${escapeHtml(job.task_type || '-')}</td>
            <td>
                ${job.critical ? '<span class="badge badge-danger">Critical</span>' : '<span class="badge badge-success">Normal</span>'}
            </td>
            <td>
                <span class="badge badge-info">${job.in_conditions_count} In</span>
                <span class="badge badge-info">${job.out_conditions_count} Out</span>
            </td>
            <td>
                <button class="btn btn-primary btn-icon btn-sm" onclick="viewJobDetail(${job.id})">
                    <i class="fas fa-eye"></i> View
                </button>
            </td>
        </tr>
    `).join('');
    
    renderPagination(data);
    
    // Add scroll detection
    setTimeout(() => {
        const tableWrapper = document.querySelector('.table-wrapper');
        if (tableWrapper) {
            const hasScroll = tableWrapper.scrollWidth > tableWrapper.clientWidth;
            if (hasScroll) {
                tableWrapper.classList.add('has-scroll');
                tableWrapper.addEventListener('scroll', function() {
                    const isNearEnd = this.scrollLeft >= (this.scrollWidth - this.clientWidth - 50);
                    if (isNearEnd) {
                        this.classList.remove('has-scroll');
                    } else if (!this.classList.contains('has-scroll')) {
                        this.classList.add('has-scroll');
                    }
                });
            }
        }
    }, 100);
}

function renderPagination(data) {
    const pagination = document.getElementById('pagination');
    const totalPages = data.total_pages;
    const current = data.page;
    
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

async function exportToCSV() {
    const startTime = performance.now();
    console.log('📥 [EXPORT] Starting CSV export...');
    showLoading(true);
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

// Make functions global for onclick handlers
window.goToPage = goToPage;
window.viewJobDetail = viewJobDetail;
