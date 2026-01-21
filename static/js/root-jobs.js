/**
 * Root Jobs Dashboard Component
 * Displays top root jobs with dependency counts and visualization
 */

/**
 * Load and display top root jobs
 */
async function loadTopRootJobs(datacenter = null, folderFilter = null, limit = 10, containerId = 'root-jobs-list') {
    const container = document.getElementById(containerId);
    if (!container) return;
    
    console.log('loadTopRootJobs called with:', { datacenter, folderFilter, limit, containerId });
    
    // Show skeleton loading
    showSkeletonTable(containerId, Math.min(limit, 10));
    
    try {
        const params = new URLSearchParams();
        if (datacenter) params.append('datacenter', datacenter);
        if (folderFilter) params.append('folder_order_method_filter', folderFilter);
        params.append('limit', limit);
        
        const url = `${API_BASE}/dashboard/root-jobs?${params.toString()}`;
        console.log('🚀 [ROOT-JOBS] Fetching from:', url);
        
        const startTime = performance.now();
        
        // Add timeout of 30 seconds
        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), 30000);
        
        const response = await fetch(url, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            },
            signal: controller.signal
        });
        
        clearTimeout(timeoutId);
        
        const loadTime = (performance.now() - startTime).toFixed(2);
        console.log(`⏱️  [ROOT-JOBS] Response received in ${loadTime}ms`);
        
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }
        
        const result = await response.json();
        
        if (result.success && result.data) {
            console.log(`✅ [ROOT-JOBS] Loaded ${result.data.length} root jobs`);
            // Small delay for smooth transition
            setTimeout(() => {
                renderRootJobsList(result.data, containerId);
                // Add smooth appear animation
                const tableWrapper = container.querySelector('.root-jobs-table-wrapper');
                if (tableWrapper) smoothAppear(tableWrapper);
            }, 150);
        } else {
            console.error('❌ [ROOT-JOBS] Failed:', result.error || result.message);
            container.innerHTML = '<div class="content-placeholder"><div class="placeholder-text">Failed to load root jobs</div></div>';
        }
    } catch (error) {
        if (error.name === 'AbortError') {
            console.error('❌ [ROOT-JOBS] Request timeout (>30s)');
            container.innerHTML = '<div class="content-placeholder"><div class="placeholder-text">Request timeout</div><div class="placeholder-subtext">The query is taking too long. Try filtering by datacenter.</div></div>';
        } else {
            console.error('❌ [ROOT-JOBS] Error:', error);
            container.innerHTML = '<div class="content-placeholder"><div class="placeholder-text">Error loading root jobs</div><div class="placeholder-subtext">Please try again</div></div>';
        }
    }
}

/**
 * Render root jobs list
 */
function renderRootJobsList(rootJobs, containerId = 'root-jobs-list') {
    const container = document.getElementById(containerId);
    if (!container) return;
    
    if (rootJobs.length === 0) {
        container.innerHTML = '<div class="empty-state"><i class="fas fa-inbox"></i><p>No root jobs found</p></div>';
        return;
    }
    
    let html = '<div class="root-jobs-table-wrapper"><table class="root-jobs-table">';
    html += `
        <thead>
            <tr>
                <th style="width: 5%;">#</th>
                <th style="width: 30%;">Job Name</th>
                <th style="width: 25%;">Folder</th>
                <th style="width: 15%;">Datacenter</th>
                <th style="width: 15%;">Dependencies</th>
                <th style="width: 10%;">Actions</th>
            </tr>
        </thead>
        <tbody>
    `;
    
    rootJobs.forEach((job, index) => {
        const rankClass = index < 3 ? 'rank-top' : '';
        const rankIcon = index === 0 ? '🥇' : index === 1 ? '🥈' : index === 2 ? '🥉' : '';
        
        html += `
            <tr class="${rankClass}">
                <td class="rank-cell">${rankIcon || (index + 1)}</td>
                <td class="job-name-cell">
                    <strong>${escapeHtml(job.job_name)}</strong>
                </td>
                <td class="folder-cell">
                    <span class="folder-badge">${escapeHtml(job.folder_name)}</span>
                </td>
                <td class="datacenter-cell">
                    <span class="datacenter-badge datacenter-${job.datacenter.toLowerCase()}">${escapeHtml(job.datacenter)}</span>
                </td>
                <td class="deps-cell">
                    <span class="deps-count">${job.downstream_count}</span> jobs
                </td>
                <td class="actions-cell">
                    <button class="btn btn-sm btn-primary" onclick="viewRootJobGraph(${job.id}, '${escapeHtml(job.job_name)}')">
                        <i class="fas fa-project-diagram"></i> View Graph
                    </button>
                </td>
            </tr>
        `;
    });
    
    html += '</tbody></table></div>';
    
    container.innerHTML = html;
}

/**
 * View dependency graph for a root job
 */
async function viewRootJobGraph(jobId, jobName) {
    // Check if viewJobDetail exists
    if (typeof viewJobDetail !== 'function') {
        console.error('viewJobDetail function not found');
        return;
    }
    
    // Open job detail modal
    await viewJobDetail(jobId);
    
    // Wait for modal to fully load, then switch to dependencies tab
    setTimeout(() => {
        if (typeof switchTab === 'function') {
            switchTab('dependencies');
        } else {
            // Fallback: manually trigger dependencies tab
            const depsButton = document.querySelector('[data-tab="dependencies"]');
            if (depsButton) {
                depsButton.click();
            }
        }
    }, 500);
}

/**
 * Escape HTML to prevent XSS
 */
function escapeHtml(text) {
    if (!text) return '';
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

/**
 * Open Root Jobs modal
 */
function openRootJobsModal() {
    const modal = document.getElementById('root-jobs-modal');
    if (modal) {
        modal.classList.add('active');
        
        // Reset modal limit to 10
        const modalLimit = document.getElementById('root-jobs-modal-limit');
        if (modalLimit) {
            modalLimit.value = '10';
        }
        
        // Clear search
        const searchInput = document.getElementById('root-jobs-search');
        if (searchInput) {
            searchInput.value = '';
        }
        
        // Load root jobs in modal using dashboard filters
        const datacenterFilter = document.getElementById('dashboard-datacenter-filter');
        const folderFilter = document.getElementById('dashboard-folder-filter');
        const datacenter = datacenterFilter ? (datacenterFilter.value || null) : null;
        const folder = folderFilter ? (folderFilter.value || null) : null;
        
        console.log('Opening modal with filters:', { datacenter, folder });
        loadTopRootJobs(datacenter, folder, 10, 'root-jobs-modal-list');
    }
}

/**
 * Close Root Jobs modal
 */
function closeRootJobsModal() {
    const modal = document.getElementById('root-jobs-modal');
    if (modal) {
        modal.classList.remove('active');
    }
}

/**
 * Filter root jobs in modal by search term
 */
function filterRootJobsInModal(searchTerm) {
    const rows = document.querySelectorAll('#root-jobs-modal-list table tbody tr');
    
    rows.forEach(row => {
        const jobName = row.querySelector('.job-name-cell')?.textContent.toLowerCase() || '';
        const folder = row.querySelector('.folder-cell')?.textContent.toLowerCase() || '';
        
        if (jobName.includes(searchTerm.toLowerCase()) || folder.includes(searchTerm.toLowerCase())) {
            row.style.display = '';
        } else {
            row.style.display = 'none';
        }
    });
}

// Auto-load when dashboard filters change
document.addEventListener('DOMContentLoaded', () => {
    const datacenterFilter = document.getElementById('dashboard-datacenter-filter');
    const folderFilter = document.getElementById('dashboard-folder-filter');
    
    // Dashboard filter changes
    if (datacenterFilter) {
        datacenterFilter.addEventListener('change', () => {
            const datacenter = datacenterFilter.value || null;
            const folder = folderFilter ? folderFilter.value : null;
            loadTopRootJobs(datacenter, folder, 10, 'root-jobs-list');
            
            // Also update modal if it's open
            const modal = document.getElementById('root-jobs-modal');
            if (modal && modal.classList.contains('active')) {
                const modalLimit = document.getElementById('root-jobs-modal-limit');
                const limit = modalLimit ? parseInt(modalLimit.value) : 10;
                loadTopRootJobs(datacenter, folder, limit, 'root-jobs-modal-list');
            }
        });
    }
    
    if (folderFilter) {
        folderFilter.addEventListener('change', () => {
            const datacenter = datacenterFilter ? datacenterFilter.value : null;
            const folder = folderFilter.value || null;
            loadTopRootJobs(datacenter, folder, 10, 'root-jobs-list');
            
            // Also update modal if it's open
            const modal = document.getElementById('root-jobs-modal');
            if (modal && modal.classList.contains('active')) {
                const modalLimit = document.getElementById('root-jobs-modal-limit');
                const limit = modalLimit ? parseInt(modalLimit.value) : 10;
                loadTopRootJobs(datacenter, folder, limit, 'root-jobs-modal-list');
            }
        });
    }
    
    // Modal controls
    const modalLimit = document.getElementById('root-jobs-modal-limit');
    if (modalLimit) {
        modalLimit.addEventListener('change', (e) => {
            // Always read fresh values from dashboard filter elements
            const dcFilter = document.getElementById('dashboard-datacenter-filter');
            const fmFilter = document.getElementById('dashboard-folder-filter');
            const datacenter = dcFilter && dcFilter.value ? dcFilter.value : null;
            const folder = fmFilter && fmFilter.value ? fmFilter.value : null;
            const limit = parseInt(e.target.value);
            console.log('Modal limit changed:', { datacenter, folder, limit });
            loadTopRootJobs(datacenter, folder, limit, 'root-jobs-modal-list');
        });
    }
    
    // Modal search
    const modalSearch = document.getElementById('root-jobs-search');
    if (modalSearch) {
        modalSearch.addEventListener('input', (e) => {
            filterRootJobsInModal(e.target.value);
        });
    }
    
    // Close modal on background click
    const modal = document.getElementById('root-jobs-modal');
    if (modal) {
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                closeRootJobsModal();
            }
        });
    }
});
