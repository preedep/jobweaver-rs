/**
 * Wave Migration Analysis
 * Dependency-Based Migration Approach
 */

let currentWaveFilters = {
    datacenter: 'neutron',
    folderOrderMethod: 'SYSTEM'
};

/**
 * Initialize Wave Migration page
 */
async function initWaveMigration() {
    console.log('📊 [WAVE] Initializing Wave Migration page...');
    
    // Setup event listeners first
    setupWaveEventListeners();
    
    // Load datacenter options and wait for completion
    await loadWaveDatacenterOptions();
    
    // Load initial data with default datacenter
    loadWaveData();
}

/**
 * Load datacenter options for filter
 */
async function loadWaveDatacenterOptions() {
    try {
        const response = await fetch(`${API_BASE}/filters`, {
            headers: { 'Authorization': `Bearer ${authToken}` }
        });
        const result = await response.json();
        
        console.log('📊 [WAVE] Filter options received:', result);
        console.log('📊 [WAVE] result.success:', result.success);
        console.log('📊 [WAVE] result.data:', result.data);
        console.log('📊 [WAVE] result.data.datacenters:', result.data?.datacenters);
        console.log('📊 [WAVE] Is array?:', Array.isArray(result.data?.datacenters));
        
        if (result.success && result.data) {
            const select = document.getElementById('wave-datacenter-filter');
            
            if (!select) {
                console.error('❌ [WAVE] wave-datacenter-filter element not found!');
                return;
            }
            
            // Clear all existing options
            select.innerHTML = '';
            
            // Add datacenter options
            if (result.data.datacenters && Array.isArray(result.data.datacenters)) {
                let hasNeutron = false;
                let firstDC = null;
                
                result.data.datacenters.forEach((dc, index) => {
                    const option = document.createElement('option');
                    option.value = dc;
                    option.textContent = dc;
                    select.appendChild(option);
                    
                    if (index === 0) firstDC = dc;
                    if (dc.toLowerCase() === 'neutron') {
                        hasNeutron = true;
                        option.selected = true;
                    }
                });
                
                // If neutron not found, select first datacenter
                if (!hasNeutron && firstDC) {
                    select.value = firstDC;
                    currentWaveFilters.datacenter = firstDC;
                    console.log('📊 [WAVE] Neutron not found, using', firstDC);
                } else if (hasNeutron) {
                    currentWaveFilters.datacenter = 'neutron';
                }
                
                console.log('✅ [WAVE] Loaded', result.data.datacenters.length, 'datacenters');
                console.log('📊 [WAVE] Selected datacenter:', currentWaveFilters.datacenter);
            } else {
                console.warn('⚠️ [WAVE] No datacenters found in response');
            }
        }
    } catch (error) {
        console.error('❌ [WAVE] Error loading datacenter options:', error);
    }
}

/**
 * Setup event listeners
 */
function setupWaveEventListeners() {
    // Filter changes
    document.getElementById('wave-datacenter-filter').addEventListener('change', (e) => {
        currentWaveFilters.datacenter = e.target.value;
        loadWaveData();
    });
    
    document.getElementById('wave-folder-filter').addEventListener('change', (e) => {
        currentWaveFilters.folderOrderMethod = e.target.value;
        loadWaveData();
    });
    
    // Wave tab switching
    document.querySelectorAll('.wave-tab-btn').forEach(btn => {
        btn.addEventListener('click', (e) => {
            const wave = e.currentTarget.dataset.wave;
            switchWaveTab(wave);
        });
    });
}

/**
 * Switch wave tab
 */
function switchWaveTab(wave) {
    // Update buttons
    document.querySelectorAll('.wave-tab-btn').forEach(btn => {
        btn.classList.toggle('active', btn.dataset.wave === wave);
    });
    
    // Update content
    document.querySelectorAll('.wave-content').forEach(content => {
        content.classList.remove('active');
    });
    document.getElementById(`wave-${wave}-content`).classList.add('active');
}

/**
 * Load wave data
 */
async function loadWaveData() {
    console.log('📊 [WAVE] Loading wave data...', currentWaveFilters);
    
    // Show loading state in all wave containers
    for (let i = 1; i <= 5; i++) {
        const container = document.getElementById(`wave-${i}-data`);
        if (container) {
            container.innerHTML = '<div class="loading-spinner"><i class="fas fa-spinner fa-spin"></i> Loading wave data...</div>';
        }
    }
    
    try {
        const params = new URLSearchParams();
        if (currentWaveFilters.datacenter) params.append('datacenter', currentWaveFilters.datacenter);
        if (currentWaveFilters.folderOrderMethod) params.append('folder_order_method', currentWaveFilters.folderOrderMethod);
        
        const startTime = performance.now();
        console.log('🌊 [WAVE] Fetching from:', `${API_BASE}/wave-migration/analysis?${params}`);
        
        const response = await fetch(`${API_BASE}/wave-migration/analysis?${params}`, {
            headers: { 'Authorization': `Bearer ${authToken}` }
        });
        
        const loadTime = (performance.now() - startTime).toFixed(2);
        console.log(`⏱️  [WAVE] API response received in ${loadTime}ms`);
        
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }
        
        const result = await response.json();
        
        if (result.success) {
            console.log('✅ [WAVE] Wave data loaded successfully');
            console.log('  - Wave 1 (Isolated):', result.data.wave1.total_jobs, 'jobs');
            console.log('  - Wave 2 (Self-Contained):', result.data.wave2.total_jobs, 'jobs');
            console.log('  - Wave 3 (Leaf):', result.data.wave3.total_jobs, 'jobs');
            console.log('  - Wave 4 (Root):', result.data.wave4.total_jobs, 'jobs');
            console.log('  - Wave 5 (Complex):', result.data.wave5.total_jobs, 'jobs');
            
            renderWaveSummary(result.data);
            renderWaveDetails(result.data);
        } else {
            console.error('❌ [WAVE] Failed to load wave data:', result.error || result.message);
            showWaveError('Failed to load wave data: ' + (result.error || result.message || 'Unknown error'));
        }
    } catch (error) {
        console.error('❌ [WAVE] Error loading wave data:', error);
        showWaveError('Error loading wave data: ' + error.message);
    }
}

/**
 * Show error message in wave containers
 */
function showWaveError(message) {
    for (let i = 1; i <= 5; i++) {
        const container = document.getElementById(`wave-${i}-data`);
        if (container) {
            container.innerHTML = `
                <div class="empty-state" style="color: #ef4444;">
                    <i class="fas fa-exclamation-triangle"></i>
                    <p>${message}</p>
                </div>
            `;
        }
    }
}

/**
 * Render wave summary cards
 */
function renderWaveSummary(data) {
    const container = document.getElementById('wave-summary-cards');
    
    const waves = [
        { id: 1, name: 'Isolated Jobs', icon: 'fa-star', color: '#0ea5e9', data: data.wave1 },
        { id: 2, name: 'Self-Contained', icon: 'fa-box', color: '#22c55e', data: data.wave2 },
        { id: 3, name: 'Leaf Jobs', icon: 'fa-arrow-down', color: '#eab308', data: data.wave3 },
        { id: 4, name: 'Root Jobs', icon: 'fa-arrow-up', color: '#f97316', data: data.wave4 },
        { id: 5, name: 'Complex', icon: 'fa-network-wired', color: '#ef4444', data: data.wave5 }
    ];
    
    container.innerHTML = waves.map(wave => `
        <div class="stat-card" style="cursor: pointer;" onclick="switchWaveTab('${wave.id}')">
            <div class="stat-icon" style="background: ${wave.color}20; color: ${wave.color};">
                <i class="fas ${wave.icon}"></i>
            </div>
            <div class="stat-info">
                <div class="stat-label">Wave ${wave.id}: ${wave.name}</div>
                <div class="stat-value">${(wave.data?.total_jobs || 0).toLocaleString()}</div>
                <div class="stat-sublabel">${(wave.data?.total_folders || 0).toLocaleString()} folders</div>
            </div>
        </div>
    `).join('');
}

/**
 * Render wave details
 */
function renderWaveDetails(data) {
    renderWave1Details(data.wave1);
    renderWave2Details(data.wave2);
    renderWave3Details(data.wave3);
    renderWave4Details(data.wave4);
    renderWave5Details(data.wave5);
}

/**
 * Render Wave 1: Isolated Jobs
 */
function renderWave1Details(data) {
    const container = document.getElementById('wave-1-data');
    
    if (!data || !data.jobs || data.jobs.length === 0) {
        container.innerHTML = '<div class="empty-state"><i class="fas fa-inbox"></i><p>No isolated jobs found</p></div>';
        return;
    }
    
    // Group by folder
    const byFolder = {};
    data.jobs.forEach(job => {
        if (!byFolder[job.folder_name]) {
            byFolder[job.folder_name] = [];
        }
        byFolder[job.folder_name].push(job);
    });
    
    container.innerHTML = `
        <div style="margin-bottom: 15px;">
            <strong>Total: ${data.total_jobs.toLocaleString()} jobs</strong> across ${Object.keys(byFolder).length} folders
        </div>
        ${Object.entries(byFolder).map(([folder, jobs]) => `
            <div style="margin-bottom: 20px; border: 1px solid #e2e8f0; border-radius: 6px; overflow: hidden;">
                <div style="background: #f8fafc; padding: 12px 15px; border-bottom: 1px solid #e2e8f0; font-weight: 600;">
                    <i class="fas fa-folder"></i> ${folder} <span style="color: #64748b; font-weight: normal;">(${jobs.length} jobs)</span>
                </div>
                <div style="padding: 10px;">
                    <table class="simple-table">
                        <thead>
                            <tr>
                                <th>Job Name</th>
                                <th>APPL Type</th>
                                <th>Application</th>
                            </tr>
                        </thead>
                        <tbody>
                            ${jobs.slice(0, 10).map(job => `
                                <tr>
                                    <td><strong>${job.job_name}</strong></td>
                                    <td>${job.appl_type || '-'}</td>
                                    <td>${job.application || '-'}</td>
                                </tr>
                            `).join('')}
                            ${jobs.length > 10 ? `
                                <tr>
                                    <td colspan="3" style="text-align: center; color: #64748b; font-style: italic;">
                                        ... and ${jobs.length - 10} more jobs
                                    </td>
                                </tr>
                            ` : ''}
                        </tbody>
                    </table>
                </div>
            </div>
        `).join('')}
    `;
}

/**
 * Render Wave 2: Self-Contained Folders
 */
function renderWave2Details(data) {
    const container = document.getElementById('wave-2-data');
    
    if (!data || !data.folders || data.folders.length === 0) {
        container.innerHTML = '<div class="empty-state"><i class="fas fa-inbox"></i><p>No self-contained folders found</p></div>';
        return;
    }
    
    container.innerHTML = `
        <div style="margin-bottom: 15px;">
            <strong>Total: ${data.total_folders} folders</strong> with ${data.total_jobs.toLocaleString()} jobs (all dependencies internal)
        </div>
        <table class="simple-table">
            <thead>
                <tr>
                    <th>Folder Name</th>
                    <th>Application</th>
                    <th>Total Jobs</th>
                    <th>Jobs with Internal Deps</th>
                    <th>Self-Contained %</th>
                </tr>
            </thead>
            <tbody>
                ${data.folders.map(folder => {
                    const percentage = folder.total_jobs > 0 ? (folder.jobs_with_internal_deps / folder.total_jobs * 100).toFixed(1) : 0;
                    return `
                        <tr>
                            <td><strong>${folder.folder_name}</strong></td>
                            <td>${folder.application || '-'}</td>
                            <td>${folder.total_jobs.toLocaleString()}</td>
                            <td>${folder.jobs_with_internal_deps.toLocaleString()}</td>
                            <td>
                                <div style="display: flex; align-items: center; gap: 10px;">
                                    <div style="flex: 1; background: #e2e8f0; height: 8px; border-radius: 4px; overflow: hidden;">
                                        <div style="width: ${percentage}%; height: 100%; background: #22c55e;"></div>
                                    </div>
                                    <span style="font-weight: 600; color: #15803d;">${percentage}%</span>
                                </div>
                            </td>
                        </tr>
                    `;
                }).join('')}
            </tbody>
        </table>
    `;
}

/**
 * Render Wave 3: Leaf Jobs
 */
function renderWave3Details(data) {
    const container = document.getElementById('wave-3-data');
    
    if (!data || !data.jobs || data.jobs.length === 0) {
        container.innerHTML = '<div class="empty-state"><i class="fas fa-inbox"></i><p>No leaf jobs found</p></div>';
        return;
    }
    
    container.innerHTML = `
        <div style="margin-bottom: 15px;">
            <strong>Total: ${data.total_jobs.toLocaleString()} leaf jobs</strong> (have in_conditions but no out_conditions)
        </div>
        <table class="simple-table">
            <thead>
                <tr>
                    <th>Job Name</th>
                    <th>Folder</th>
                    <th>APPL Type</th>
                    <th>In Conditions</th>
                </tr>
            </thead>
            <tbody>
                ${data.jobs.slice(0, 50).map(job => `
                    <tr>
                        <td><strong>${job.job_name}</strong></td>
                        <td>${job.folder_name}</td>
                        <td>${job.appl_type || '-'}</td>
                        <td>${job.in_conditions_count}</td>
                    </tr>
                `).join('')}
                ${data.jobs.length > 50 ? `
                    <tr>
                        <td colspan="4" style="text-align: center; color: #64748b; font-style: italic;">
                            ... and ${data.jobs.length - 50} more jobs
                        </td>
                    </tr>
                ` : ''}
            </tbody>
        </table>
    `;
}

/**
 * Render Wave 4: Root Jobs
 */
function renderWave4Details(data) {
    const container = document.getElementById('wave-4-data');
    
    if (!data || !data.jobs || data.jobs.length === 0) {
        container.innerHTML = '<div class="empty-state"><i class="fas fa-inbox"></i><p>No root jobs found</p></div>';
        return;
    }
    
    container.innerHTML = `
        <div style="margin-bottom: 15px;">
            <strong>Total: ${data.total_jobs.toLocaleString()} root jobs</strong> (no in_conditions but have out_conditions)
        </div>
        <table class="simple-table">
            <thead>
                <tr>
                    <th>Job Name</th>
                    <th>Folder</th>
                    <th>APPL Type</th>
                    <th>Out Conditions</th>
                </tr>
            </thead>
            <tbody>
                ${data.jobs.slice(0, 50).map(job => `
                    <tr>
                        <td><strong>${job.job_name}</strong></td>
                        <td>${job.folder_name}</td>
                        <td>${job.appl_type || '-'}</td>
                        <td>${job.out_conditions_count}</td>
                    </tr>
                `).join('')}
                ${data.jobs.length > 50 ? `
                    <tr>
                        <td colspan="4" style="text-align: center; color: #64748b; font-style: italic;">
                            ... and ${data.jobs.length - 50} more jobs
                        </td>
                    </tr>
                ` : ''}
            </tbody>
        </table>
    `;
}

/**
 * Render Wave 5: Complex Dependencies
 */
function renderWave5Details(data) {
    const container = document.getElementById('wave-5-data');
    
    if (!data || !data.folders || data.folders.length === 0) {
        container.innerHTML = '<div class="empty-state"><i class="fas fa-inbox"></i><p>No folders with complex dependencies found</p></div>';
        return;
    }
    
    container.innerHTML = `
        <div style="margin-bottom: 15px;">
            <strong>Total: ${data.total_folders} folders</strong> with ${data.total_jobs.toLocaleString()} jobs (have external dependencies)
        </div>
        <table class="simple-table">
            <thead>
                <tr>
                    <th>Folder Name</th>
                    <th>Total Jobs</th>
                    <th>Jobs with External Deps</th>
                    <th>External Dep %</th>
                </tr>
            </thead>
            <tbody>
                ${data.folders.map(folder => {
                    const percentage = folder.total_jobs > 0 ? (folder.jobs_with_external_deps / folder.total_jobs * 100).toFixed(1) : 0;
                    return `
                        <tr>
                            <td><strong>${folder.folder_name}</strong></td>
                            <td>${folder.total_jobs.toLocaleString()}</td>
                            <td>${folder.jobs_with_external_deps.toLocaleString()}</td>
                            <td>
                                <div style="display: flex; align-items: center; gap: 10px;">
                                    <div style="flex: 1; background: #e2e8f0; height: 8px; border-radius: 4px; overflow: hidden;">
                                        <div style="width: ${percentage}%; height: 100%; background: #ef4444;"></div>
                                    </div>
                                    <span style="font-weight: 600; color: #b91c1c;">${percentage}%</span>
                                </div>
                            </td>
                        </tr>
                    `;
                }).join('')}
            </tbody>
        </table>
    `;
}

/**
 * Export wave data to CSV
 */
function exportWaveToCSV(waveNumber) {
    console.log(`📥 [WAVE] Exporting Wave ${waveNumber} to CSV...`);
    
    if (!currentWaveData) {
        alert('No data available to export. Please load the wave data first.');
        return;
    }
    
    const waveKey = `wave${waveNumber}`;
    const waveData = currentWaveData[waveKey];
    
    if (!waveData) {
        alert('No data available for this wave.');
        return;
    }
    
    let csvContent = '';
    let filename = '';
    
    // Wave 1, 3, 4 have jobs
    if (waveData.jobs && waveData.jobs.length > 0) {
        const waveName = {
            1: 'Isolated_Jobs',
            3: 'Leaf_Jobs',
            4: 'Root_Jobs'
        }[waveNumber];
        
        filename = `wave${waveNumber}_${waveName}_${new Date().toISOString().split('T')[0]}.csv`;
        
        // CSV Header
        const headers = ['Job ID', 'Job Name', 'Folder Name', 'APPL Type', 'Application'];
        if (waveNumber === 3) headers.push('In Conditions Count');
        if (waveNumber === 4) headers.push('Out Conditions Count');
        
        csvContent = headers.join(',') + '\n';
        
        // CSV Rows
        waveData.jobs.forEach(job => {
            const row = [
                job.job_id,
                `"${(job.job_name || '').replace(/"/g, '""')}"`,
                `"${(job.folder_name || '').replace(/"/g, '""')}"`,
                `"${(job.appl_type || '').replace(/"/g, '""')}"`,
                `"${(job.application || '').replace(/"/g, '""')}"`
            ];
            
            if (waveNumber === 3) row.push(job.in_conditions_count || 0);
            if (waveNumber === 4) row.push(job.out_conditions_count || 0);
            
            csvContent += row.join(',') + '\n';
        });
    }
    // Wave 2, 5 have folders
    else if (waveData.folders && waveData.folders.length > 0) {
        const waveName = {
            2: 'Self_Contained_Folders',
            5: 'Complex_Dependencies'
        }[waveNumber];
        
        filename = `wave${waveNumber}_${waveName}_${new Date().toISOString().split('T')[0]}.csv`;
        
        // CSV Header
        const headers = ['Folder Name', 'Application', 'Total Jobs', 'Jobs with Internal Deps'];
        if (waveNumber === 5) headers.push('Jobs with External Deps');
        
        csvContent = headers.join(',') + '\n';
        
        // CSV Rows
        waveData.folders.forEach(folder => {
            const row = [
                `"${(folder.folder_name || '').replace(/"/g, '""')}"`,
                `"${(folder.application || '').replace(/"/g, '""')}"`,
                folder.total_jobs,
                folder.jobs_with_internal_deps
            ];
            
            if (waveNumber === 5) row.push(folder.jobs_with_external_deps || 0);
            
            csvContent += row.join(',') + '\n';
        });
    } else {
        alert('No data available to export for this wave.');
        return;
    }
    
    // Create and download CSV file
    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
    const link = document.createElement('a');
    const url = URL.createObjectURL(blob);
    
    link.setAttribute('href', url);
    link.setAttribute('download', filename);
    link.style.visibility = 'hidden';
    
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    
    console.log(`✅ [WAVE] Exported Wave ${waveNumber} to ${filename}`);
}

// Log module loaded
console.log('📊 [WAVE] Wave Migration module loaded');
