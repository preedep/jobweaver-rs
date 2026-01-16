/**
 * Root Jobs Dashboard Component
 * Displays top root jobs with dependency counts and visualization
 */

/**
 * Load and display top root jobs
 */
async function loadTopRootJobs(datacenter = null) {
    const container = document.getElementById('root-jobs-list');
    if (!container) return;
    
    // Show loading
    container.innerHTML = '<div class="loading-state"><i class="fas fa-spinner fa-spin"></i> Loading root jobs...</div>';
    
    try {
        const url = datacenter 
            ? `${API_BASE}/dashboard/root-jobs?datacenter=${encodeURIComponent(datacenter)}`
            : `${API_BASE}/dashboard/root-jobs`;
            
        const response = await fetch(url, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
        });
        
        const result = await response.json();
        
        if (result.success && result.data) {
            renderRootJobsList(result.data);
        } else {
            container.innerHTML = '<div class="error">Failed to load root jobs</div>';
        }
    } catch (error) {
        console.error('Error loading root jobs:', error);
        container.innerHTML = '<div class="error">Error loading root jobs</div>';
    }
}

/**
 * Render root jobs list
 */
function renderRootJobsList(rootJobs) {
    const container = document.getElementById('root-jobs-list');
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
    // Open job detail modal and switch to dependencies tab
    await viewJobDetail(jobId);
    
    // Wait a bit for modal to open, then switch to dependencies tab
    setTimeout(() => {
        if (typeof switchTab === 'function') {
            switchTab('dependencies');
        }
    }, 300);
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

// Auto-load when dashboard datacenter filter changes
document.addEventListener('DOMContentLoaded', () => {
    const datacenterFilter = document.getElementById('dashboard-datacenter-filter');
    if (datacenterFilter) {
        datacenterFilter.addEventListener('change', (e) => {
            const datacenter = e.target.value;
            loadTopRootJobs(datacenter || null);
        });
    }
});
