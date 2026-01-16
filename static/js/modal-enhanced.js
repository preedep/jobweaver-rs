/**
 * Enhanced Job Detail Modal Functions
 * Handles tab switching and data population for all job fields
 */

// Tab Switching
function switchTab(tabName) {
    // Remove active class from all tabs and buttons
    document.querySelectorAll('.tab-content').forEach(tab => {
        tab.classList.remove('active');
    });
    document.querySelectorAll('.tab-button').forEach(btn => {
        btn.classList.remove('active');
    });
    
    // Add active class to selected tab and button
    document.getElementById(`tab-${tabName}`).classList.add('active');
    document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');
    
    // Load dependency graph when Dependencies tab is opened
    if (tabName === 'dependencies' && window.currentJobIdForGraph && typeof loadDependencyGraph === 'function') {
        loadDependencyGraph(window.currentJobIdForGraph);
    }
}

// Initialize tab switching on page load
document.addEventListener('DOMContentLoaded', () => {
    document.querySelectorAll('.tab-button').forEach(button => {
        button.addEventListener('click', () => {
            const tabName = button.getAttribute('data-tab');
            switchTab(tabName);
        });
    });
});

// Helper function to create detail item
function createDetailItem(label, value) {
    const displayValue = value !== null && value !== undefined && value !== '' ? value : '-';
    const isEmpty = displayValue === '-';
    return `
        <div class="detail-item">
            <div class="detail-item-label">${label}</div>
            <div class="detail-item-value ${isEmpty ? 'empty' : ''}">${escapeHtml(String(displayValue))}</div>
        </div>
    `;
}

// Populate Basic Tab
function populateBasicTab(data) {
    const job = data.job;
    const basicTab = document.getElementById('tab-basic');
    
    let html = `
        <div class="detail-section">
            <h3><i class="fas fa-info-circle"></i> Job Identification</h3>
            <div class="detail-grid">
                ${createDetailItem('Job Name', job.job_name)}
                ${createDetailItem('Job ISN', job.jobisn)}
                ${createDetailItem('Member Name', job.memname)}
                ${createDetailItem('Group', job.group)}
                ${createDetailItem('Folder', job.folder_name)}
                ${createDetailItem('Datacenter', job.datacenter)}
                ${createDetailItem('Folder Order Method', job.folder_order_method)}
            </div>
        </div>
        
        <div class="detail-section">
            <h3><i class="fas fa-briefcase"></i> Application Info</h3>
            <div class="detail-grid">
                ${createDetailItem('Application', job.application)}
                ${createDetailItem('Sub Application', job.sub_application)}
                ${createDetailItem('APPL Type', job.appl_type)}
                ${createDetailItem('APPL Version', job.appl_ver)}
                ${createDetailItem('APPL Form', job.appl_form)}
                ${createDetailItem('Task Type', job.task_type)}
            </div>
        </div>
        
        <div class="detail-section">
            <h3><i class="fas fa-user"></i> Execution & Ownership</h3>
            <div class="detail-grid">
                ${createDetailItem('Owner', job.owner)}
                ${createDetailItem('Run As', job.run_as)}
                ${createDetailItem('Author', job.author)}
                ${createDetailItem('Node ID', job.node_id)}
                ${createDetailItem('Priority', job.priority)}
                ${createDetailItem('Multi Agent', job.multy_agent)}
            </div>
        </div>
        
        <div class="detail-section">
            <h3><i class="fas fa-book"></i> Documentation</h3>
            <div class="detail-grid">
                ${createDetailItem('Doc Library', job.doclib)}
                ${createDetailItem('Doc Member', job.docmem)}
                ${createDetailItem('Member Library', job.memlib)}
                ${createDetailItem('Override Library', job.overlib)}
                ${createDetailItem('Override Path', job.override_path)}
            </div>
        </div>
        
        <div class="detail-section">
            <h3><i class="fas fa-flag"></i> Status Flags</h3>
            <div class="detail-grid">
                ${createDetailItem('Critical', job.critical ? 'Yes' : 'No')}
                ${createDetailItem('Cyclic', job.cyclic ? 'Yes' : 'No')}
            </div>
        </div>
    `;
    
    if (job.description) {
        html += `
            <div class="detail-section">
                <h3><i class="fas fa-align-left"></i> Description</h3>
                <div style="background: #f9fafb; padding: 16px; border-radius: 8px; border-left: 3px solid #667eea;">
                    ${escapeHtml(job.description)}
                </div>
            </div>
        `;
    }
    
    if (job.cmdline) {
        html += `
            <div class="detail-section">
                <h3><i class="fas fa-terminal"></i> Command Line</h3>
                <pre style="background: #1f2937; color: #10b981; padding: 16px; border-radius: 8px; overflow-x: auto; font-family: 'Courier New', monospace; font-size: 13px;">${escapeHtml(job.cmdline)}</pre>
            </div>
        `;
    }
    
    basicTab.innerHTML = html;
}

// Populate Scheduling Tab
function populateSchedulingTab(data) {
    const job = data.job;
    const scheduling = data.scheduling || {};
    
    const timeHtml = `
        ${createDetailItem('Time From', scheduling.time_from)}
        ${createDetailItem('Time To', scheduling.time_to)}
    `;
    
    const daysHtml = `
        ${createDetailItem('Days', job.days)}
        ${createDetailItem('Weekdays', job.weekdays)}
        ${createDetailItem('Date', job.date)}
        ${createDetailItem('Days AND/OR', job.days_and_or)}
        ${createDetailItem('Days Calendar', scheduling.days_calendar)}
        ${createDetailItem('Weeks Calendar', scheduling.weeks_calendar)}
        ${createDetailItem('Conf Calendar', scheduling.conf_calendar)}
    `;
    
    const monthsHtml = `
        ${createDetailItem('January', job.jan === '1' ? 'Yes' : 'No')}
        ${createDetailItem('February', job.feb === '1' ? 'Yes' : 'No')}
        ${createDetailItem('March', job.mar === '1' ? 'Yes' : 'No')}
        ${createDetailItem('April', job.apr === '1' ? 'Yes' : 'No')}
        ${createDetailItem('May', job.may === '1' ? 'Yes' : 'No')}
        ${createDetailItem('June', job.jun === '1' ? 'Yes' : 'No')}
        ${createDetailItem('July', job.jul === '1' ? 'Yes' : 'No')}
        ${createDetailItem('August', job.aug === '1' ? 'Yes' : 'No')}
        ${createDetailItem('September', job.sep === '1' ? 'Yes' : 'No')}
        ${createDetailItem('October', job.oct === '1' ? 'Yes' : 'No')}
        ${createDetailItem('November', job.nov === '1' ? 'Yes' : 'No')}
        ${createDetailItem('December', job.dec === '1' ? 'Yes' : 'No')}
    `;
    
    const cyclicHtml = `
        ${createDetailItem('Cyclic', job.cyclic ? 'Yes' : 'No')}
        ${createDetailItem('Interval', job.interval)}
        ${createDetailItem('Cyclic Type', job.cyclic_type)}
        ${createDetailItem('Cyclic Tolerance', job.cyclic_tolerance)}
        ${createDetailItem('Cyclic Interval Sequence', job.cyclic_interval_sequence)}
        ${createDetailItem('Cyclic Times Sequence', job.cyclic_times_sequence)}
        ${createDetailItem('Independent Cyclic', job.ind_cyclic)}
    `;
    
    document.getElementById('scheduling-time').innerHTML = timeHtml;
    document.getElementById('scheduling-days').innerHTML = daysHtml;
    document.getElementById('scheduling-calendars').innerHTML = monthsHtml;
    document.getElementById('scheduling-cyclic').innerHTML = cyclicHtml;
}

// Populate Limits Tab
function populateLimitsTab(data) {
    const job = data.job;
    
    const executionHtml = `
        ${createDetailItem('Max Wait', job.maxwait)}
        ${createDetailItem('Max Rerun', job.maxrerun)}
        ${createDetailItem('Max Days', job.maxdays)}
        ${createDetailItem('Max Runs', job.maxruns)}
        ${createDetailItem('Rerun Member', job.rerunmem)}
        ${createDetailItem('Minimum', job.minimum)}
        ${createDetailItem('Category', job.category)}
    `;
    
    const shiftHtml = `
        ${createDetailItem('Shift', job.shift)}
        ${createDetailItem('Shift Number', job.shiftnum)}
        ${createDetailItem('Priority', job.priority)}
        ${createDetailItem('Task Class', job.task_class)}
        ${createDetailItem('Previous Day', job.prev_day)}
    `;
    
    const otherHtml = `
        ${createDetailItem('Confirm', job.confirm)}
        ${createDetailItem('Retro', job.retro)}
        ${createDetailItem('Auto Archive', job.autoarch)}
        ${createDetailItem('Adjust Condition', job.adjust_cond)}
        ${createDetailItem('Jobs in Group', job.jobs_in_group)}
        ${createDetailItem('Large Size', job.large_size)}
        ${createDetailItem('Prevent NCT2', job.preventnct2)}
        ${createDetailItem('Option', job.option_field)}
        ${createDetailItem('From', job.from_field)}
        ${createDetailItem('Parameter', job.par)}
        ${createDetailItem('System DB', job.sysdb)}
        ${createDetailItem('Due Out', job.due_out)}
        ${createDetailItem('Due Out Days Offset', job.due_out_daysoffset)}
        ${createDetailItem('From Days Offset', job.from_daysoffset)}
        ${createDetailItem('To Days Offset', job.to_daysoffset)}
        ${createDetailItem('Retention Days', job.reten_days)}
        ${createDetailItem('Retention Generation', job.reten_gen)}
        ${createDetailItem('PDS Name', job.pdsname)}
    `;
    
    document.getElementById('limits-execution').innerHTML = executionHtml;
    document.getElementById('limits-shift').innerHTML = shiftHtml;
    document.getElementById('limits-other').innerHTML = otherHtml;
}

// Populate Dependencies Tab
function populateDependenciesTab(data) {
    // Store job ID for lazy loading when tab is clicked
    if (data.job && data.job.id) {
        window.currentJobIdForGraph = data.job.id;
    }
    
    // In Conditions
    const inConditions = data.in_conditions || [];
    document.getElementById('in-cond-count').textContent = inConditions.length;
    
    if (inConditions.length > 0) {
        const inCondHtml = `
            <table class="resource-table">
                <thead>
                    <tr>
                        <th>Condition Name</th>
                        <th>ODATE</th>
                        <th>AND/OR</th>
                    </tr>
                </thead>
                <tbody>
                    ${inConditions.map(c => `
                        <tr>
                            <td><strong>${escapeHtml(c.condition_name)}</strong></td>
                            <td>${escapeHtml(c.odate || '-')}</td>
                            <td>${escapeHtml(c.and_or || '-')}</td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
        document.getElementById('in-conditions-list').innerHTML = inCondHtml;
    } else {
        document.getElementById('in-conditions-list').innerHTML = '<div class="empty-state"><i class="fas fa-inbox"></i><p>No in conditions</p></div>';
    }
    
    // Out Conditions
    const outConditions = data.out_conditions || [];
    document.getElementById('out-cond-count').textContent = outConditions.length;
    
    if (outConditions.length > 0) {
        const outCondHtml = `
            <table class="resource-table">
                <thead>
                    <tr>
                        <th>Condition Name</th>
                        <th>ODATE</th>
                        <th>Sign</th>
                    </tr>
                </thead>
                <tbody>
                    ${outConditions.map(c => `
                        <tr>
                            <td><strong>${escapeHtml(c.condition_name)}</strong></td>
                            <td>${escapeHtml(c.odate || '-')}</td>
                            <td>${escapeHtml(c.and_or || '-')}</td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
        document.getElementById('out-conditions-list').innerHTML = outCondHtml;
    } else {
        document.getElementById('out-conditions-list').innerHTML = '<div class="empty-state"><i class="fas fa-inbox"></i><p>No out conditions</p></div>';
    }
    
    // Control Resources
    const controlResources = data.control_resources || [];
    document.getElementById('ctrl-res-count').textContent = controlResources.length;
    
    if (controlResources.length > 0) {
        const ctrlResHtml = `
            <table class="resource-table">
                <thead>
                    <tr>
                        <th>Resource Name</th>
                        <th>Type</th>
                        <th>On Fail</th>
                    </tr>
                </thead>
                <tbody>
                    ${controlResources.map(r => `
                        <tr>
                            <td><strong>${escapeHtml(r.resource_name)}</strong></td>
                            <td>${escapeHtml(r.resource_type || '-')}</td>
                            <td>${escapeHtml(r.on_fail || '-')}</td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
        document.getElementById('control-resources-list').innerHTML = ctrlResHtml;
    } else {
        document.getElementById('control-resources-list').innerHTML = '<div class="empty-state"><i class="fas fa-inbox"></i><p>No control resources</p></div>';
    }
    
    // Quantitative Resources
    const quantResources = data.quantitative_resources || [];
    document.getElementById('quant-res-count').textContent = quantResources.length;
    
    if (quantResources.length > 0) {
        const quantResHtml = `
            <table class="resource-table">
                <thead>
                    <tr>
                        <th>Resource Name</th>
                        <th>Quantity</th>
                    </tr>
                </thead>
                <tbody>
                    ${quantResources.map(r => `
                        <tr>
                            <td><strong>${escapeHtml(r.resource_name)}</strong></td>
                            <td>${r.quantity || 0}</td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
        document.getElementById('quantitative-resources-list').innerHTML = quantResHtml;
    } else {
        document.getElementById('quantitative-resources-list').innerHTML = '<div class="empty-state"><i class="fas fa-inbox"></i><p>No quantitative resources</p></div>';
    }
}

// Populate Variables Tab
function populateVariablesTab(data) {
    // Variables
    const variables = data.variables || [];
    document.getElementById('var-count').textContent = variables.length;
    
    if (variables.length > 0) {
        const varHtml = `
            <table class="resource-table">
                <thead>
                    <tr>
                        <th>Variable Name</th>
                        <th>Value</th>
                    </tr>
                </thead>
                <tbody>
                    ${variables.map(v => `
                        <tr>
                            <td><strong>${escapeHtml(v.name)}</strong></td>
                            <td><code style="background: #f3f4f6; padding: 2px 6px; border-radius: 4px;">${escapeHtml(v.value)}</code></td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
        document.getElementById('variables-list').innerHTML = varHtml;
    } else {
        document.getElementById('variables-list').innerHTML = '<div class="empty-state"><i class="fas fa-inbox"></i><p>No variables</p></div>';
    }
    
    // ON Conditions
    const onConditions = data.on_conditions || [];
    document.getElementById('on-cond-count').textContent = onConditions.length;
    
    if (onConditions.length > 0) {
        const onCondHtml = onConditions.map(oc => `
            <div class="list-item">
                <div class="list-item-header">
                    <div class="list-item-title">
                        <i class="fas fa-bolt"></i>
                        ${escapeHtml(oc.stmt || 'ON Condition')}
                    </div>
                    <div class="list-item-meta">
                        ${oc.code ? `Code: ${escapeHtml(oc.code)}` : ''}
                    </div>
                </div>
                ${oc.pattern ? `<div class="list-item-content">Pattern: ${escapeHtml(oc.pattern)}</div>` : ''}
                ${oc.actions && oc.actions.length > 0 ? `
                    <div class="list-item-actions">
                        ${oc.actions.map(a => `
                            <div class="action-item">
                                <i class="fas fa-check-circle"></i>
                                <span>${escapeHtml(a.action_type || 'Action')}: ${escapeHtml(a.action_value || '')}</span>
                            </div>
                        `).join('')}
                    </div>
                ` : ''}
            </div>
        `).join('');
        document.getElementById('on-conditions-list').innerHTML = onCondHtml;
    } else {
        document.getElementById('on-conditions-list').innerHTML = '<div class="empty-state"><i class="fas fa-inbox"></i><p>No ON conditions</p></div>';
    }
    
    // Auto Edits
    const autoEdits = data.auto_edits || [];
    if (autoEdits.length > 0) {
        const autoEditHtml = `
            <table class="resource-table">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Value</th>
                    </tr>
                </thead>
                <tbody>
                    ${autoEdits.map(ae => `
                        <tr>
                            <td><strong>${escapeHtml(ae.name)}</strong></td>
                            <td>${escapeHtml(ae.value)}</td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
        document.getElementById('auto-edits-list').innerHTML = autoEditHtml;
    } else {
        document.getElementById('auto-edits-list').innerHTML = '<div class="empty-state"><i class="fas fa-inbox"></i><p>No auto edits</p></div>';
    }
}

// Populate Metadata Tab
function populateMetadataTab(data) {
    const job = data.job;
    
    const creationHtml = `
        ${createDetailItem('Created By', job.created_by)}
        ${createDetailItem('Creation User', job.creation_user)}
        ${createDetailItem('Creation Date', job.creation_date)}
        ${createDetailItem('Creation Time', job.creation_time)}
    `;
    
    const modificationHtml = `
        ${createDetailItem('Change User ID', job.change_userid)}
        ${createDetailItem('Change Date', job.change_date)}
        ${createDetailItem('Change Time', job.change_time)}
    `;
    
    const versionHtml = `
        ${createDetailItem('Job Version', job.job_version)}
        ${createDetailItem('CM Version', job.cm_ver)}
        ${createDetailItem('Version Opcode', job.version_opcode)}
        ${createDetailItem('Is Current Version', job.is_current_version)}
        ${createDetailItem('Version Serial', job.version_serial)}
        ${createDetailItem('Version Host', job.version_host)}
        ${createDetailItem('APPL Form', job.appl_form)}
        ${createDetailItem('Timezone', job.timezone)}
        ${createDetailItem('Active From', job.active_from)}
        ${createDetailItem('Active Till', job.active_till)}
        ${createDetailItem('Rule Based Calendar Relationship', job.rule_based_calendar_relationship)}
        ${createDetailItem('Tag Relationship', job.tag_relationship)}
    `;
    
    const environmentHtml = `
        ${createDetailItem('Scheduling Environment', job.scheduling_environment)}
        ${createDetailItem('System Affinity', job.system_affinity)}
        ${createDetailItem('Request NJE Node', job.request_nje_node)}
        ${createDetailItem('Statistical Calendar', job.stat_cal)}
        ${createDetailItem('Instream JCL', job.instream_jcl)}
        ${createDetailItem('Use Instream JCL', job.use_instream_jcl)}
    `;
    
    const hierarchyHtml = `
        ${createDetailItem('Parent Folder', job.parent_folder)}
        ${createDetailItem('Parent Table', job.parent_table)}
        ${createDetailItem('End Folder', job.end_folder)}
        ${createDetailItem('ODATE', job.odate)}
        ${createDetailItem('From Procedures', job.fprocs)}
        ${createDetailItem('To Programs', job.tpgms)}
        ${createDetailItem('To Procedures', job.tprocs)}
    `;
    
    document.getElementById('metadata-creation').innerHTML = creationHtml;
    document.getElementById('metadata-modification').innerHTML = modificationHtml;
    document.getElementById('metadata-version').innerHTML = versionHtml + environmentHtml;
    document.getElementById('metadata-hierarchy').innerHTML = hierarchyHtml;
}

// Make functions global
window.switchTab = switchTab;
window.populateBasicTab = populateBasicTab;
window.populateSchedulingTab = populateSchedulingTab;
window.populateLimitsTab = populateLimitsTab;
window.populateDependenciesTab = populateDependenciesTab;
window.populateVariablesTab = populateVariablesTab;
window.populateMetadataTab = populateMetadataTab;
