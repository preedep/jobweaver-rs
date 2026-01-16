/**
 * Chart Detail Modal - Enhanced Statistics and Visualization
 * Provides detailed view of dashboard charts with comprehensive statistics
 */

let currentChartData = null;
let currentChartType = null;

/**
 * Opens the chart detail modal with enhanced statistics
 * @param {string} chartType - Type of chart (appl_type, task_type, applications, folders)
 * @param {Array} data - Chart data array
 * @param {string} title - Chart title
 * @param {string} subtitle - Chart subtitle
 */
function openChartDetail(chartType, data, title, subtitle) {
    currentChartData = data;
    currentChartType = chartType;
    
    // Set modal title and subtitle
    document.getElementById('chart-modal-title').textContent = title;
    document.getElementById('chart-modal-subtitle').textContent = subtitle;
    
    // Apply default limit of 10 items
    const defaultLimit = 10;
    const displayData = data.slice(0, defaultLimit);
    
    // Render initial chart with default settings (Top 10)
    renderDetailedChart(displayData, chartType);
    
    // Render statistics
    renderChartStatistics(displayData, chartType);
    
    // Render data table
    renderChartDataTable(displayData, chartType);
    
    // Render insights
    renderChartInsights(displayData, chartType);
    
    // Show modal
    document.getElementById('chart-detail-modal').classList.add('active');
    
    // Setup event listeners
    setupChartControls();
}

/**
 * Closes the chart detail modal
 */
function closeChartModal() {
    document.getElementById('chart-detail-modal').classList.remove('active');
    currentChartData = null;
    currentChartType = null;
}

/**
 * Setup event listeners for chart controls
 */
function setupChartControls() {
    const searchInput = document.getElementById('chart-search');
    const sortSelect = document.getElementById('chart-sort');
    const limitSelect = document.getElementById('chart-limit');
    
    // Remove old listeners
    const newSearchInput = searchInput.cloneNode(true);
    searchInput.parentNode.replaceChild(newSearchInput, searchInput);
    
    const newSortSelect = sortSelect.cloneNode(true);
    sortSelect.parentNode.replaceChild(newSortSelect, sortSelect);
    
    const newLimitSelect = limitSelect.cloneNode(true);
    limitSelect.parentNode.replaceChild(newLimitSelect, limitSelect);
    
    // Add new listeners
    newSearchInput.addEventListener('input', () => applyChartFilters());
    newSortSelect.addEventListener('change', () => applyChartFilters());
    newLimitSelect.addEventListener('change', () => applyChartFilters());
}

/**
 * Apply filters and re-render chart
 */
function applyChartFilters() {
    if (!currentChartData) return;
    
    const searchTerm = document.getElementById('chart-search').value.toLowerCase();
    const sortBy = document.getElementById('chart-sort').value;
    const limit = document.getElementById('chart-limit').value;
    
    console.log(`🔄 [CHART] Applying filters - search: "${searchTerm}", sort: ${sortBy}, limit: ${limit}`);
    
    // Filter data
    let filteredData = currentChartData.filter(item => {
        const name = (item.name || item.application || item.folder_name || item.appl_type || item.task_type || '').toLowerCase();
        return name.includes(searchTerm);
    });
    
    // Sort data
    filteredData = sortChartData(filteredData, sortBy);
    
    // Limit data
    if (limit !== 'all') {
        filteredData = filteredData.slice(0, parseInt(limit));
    }
    
    console.log(`📊 [CHART] Filtered data: ${filteredData.length} items`);
    
    // Re-render everything with filtered data
    renderDetailedChart(filteredData, currentChartType);
    renderChartStatistics(filteredData, currentChartType);
    renderChartDataTable(filteredData, currentChartType);
    renderChartInsights(filteredData, currentChartType);
}

/**
 * Sort chart data based on criteria
 */
function sortChartData(data, sortBy) {
    const sorted = [...data];
    
    switch (sortBy) {
        case 'count-desc':
            return sorted.sort((a, b) => b.count - a.count);
        case 'count-asc':
            return sorted.sort((a, b) => a.count - b.count);
        case 'name-asc':
            return sorted.sort((a, b) => {
                const nameA = (a.name || a.application || a.folder_name || a.appl_type || a.task_type || '').toLowerCase();
                const nameB = (b.name || b.application || b.folder_name || b.appl_type || b.task_type || '').toLowerCase();
                return nameA.localeCompare(nameB);
            });
        case 'name-desc':
            return sorted.sort((a, b) => {
                const nameA = (a.name || a.application || a.folder_name || a.appl_type || a.task_type || '').toLowerCase();
                const nameB = (b.name || b.application || b.folder_name || b.appl_type || b.task_type || '').toLowerCase();
                return nameB.localeCompare(nameA);
            });
        default:
            return sorted;
    }
}

/**
 * Render detailed chart with enhanced visualization
 */
function renderDetailedChart(data, chartType) {
    const canvas = document.getElementById('chart-detail-canvas');
    canvas.innerHTML = '';
    
    // Create larger bar chart
    const chartContainer = document.createElement('div');
    chartContainer.style.minHeight = '600px';
    
    const labels = data.map(item => 
        item.name || item.application || item.folder_name || item.appl_type || item.task_type || 'Unknown'
    );
    const values = data.map(item => item.count);
    const total = values.reduce((sum, val) => sum + val, 0);
    const percentages = values.map(val => ((val / total) * 100).toFixed(1));
    
    // Create HTML bar chart
    const html = `
        <div class="enhanced-bar-chart">
            ${data.map((item, index) => {
                const name = item.name || item.application || item.folder_name || item.appl_type || item.task_type || 'Unknown';
                const count = item.count;
                const percent = percentages[index];
                const maxCount = Math.max(...values);
                const barWidth = (count / maxCount) * 100;
                
                return `
                    <div class="bar-item">
                        <div class="bar-label">
                            <span class="bar-rank">#${index + 1}</span>
                            <span class="bar-name" title="${escapeHtml(name)}">${escapeHtml(name)}</span>
                        </div>
                        <div class="bar-container">
                            <div class="bar-fill" style="width: ${barWidth}%">
                                <span class="bar-value">${count.toLocaleString()}</span>
                            </div>
                            <span class="bar-percent">${percent}%</span>
                        </div>
                    </div>
                `;
            }).join('')}
        </div>
    `;
    
    canvas.innerHTML = html + `
        <style>
            .enhanced-bar-chart {
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .bar-item {
                display: flex;
                flex-direction: column;
                gap: 6px;
            }
            .bar-label {
                display: flex;
                align-items: center;
                gap: 8px;
                font-size: 14px;
            }
            .bar-rank {
                font-weight: 700;
                color: #64748b;
                min-width: 35px;
            }
            .bar-name {
                font-weight: 600;
                color: #1e293b;
                flex: 1;
                overflow: hidden;
                text-overflow: ellipsis;
                white-space: nowrap;
            }
            .bar-container {
                display: flex;
                align-items: center;
                gap: 12px;
                height: 32px;
            }
            .bar-fill {
                height: 100%;
                background: linear-gradient(90deg, #3b82f6, #2563eb);
                border-radius: 6px;
                display: flex;
                align-items: center;
                padding: 0 12px;
                min-width: 60px;
                transition: width 0.3s ease;
                box-shadow: 0 2px 4px rgba(59, 130, 246, 0.2);
            }
            .bar-value {
                color: white;
                font-weight: 700;
                font-size: 13px;
            }
            .bar-percent {
                color: #3b82f6;
                font-weight: 600;
                font-size: 13px;
                min-width: 50px;
            }
        </style>
    `;
}

/**
 * Render comprehensive statistics
 */
function renderChartStatistics(data, chartType) {
    const statsGrid = document.getElementById('chart-stats-grid');
    
    const counts = data.map(item => item.count);
    const total = counts.reduce((sum, val) => sum + val, 0);
    const average = total / counts.length;
    const median = calculateMedian(counts);
    const min = Math.min(...counts);
    const max = Math.max(...counts);
    
    const stats = [
        { 
            label: 'Total Items', 
            value: data.length.toLocaleString(), 
            class: 'primary',
            tooltip: 'Number of items displayed in this view'
        },
        { 
            label: 'Total Count', 
            value: total.toLocaleString(), 
            class: 'primary',
            tooltip: 'Sum of all jobs across displayed items'
        },
        { 
            label: 'Average', 
            value: Math.round(average).toLocaleString(), 
            class: '',
            tooltip: 'Mean jobs per item (Total ÷ Items)'
        },
        { 
            label: 'Median', 
            value: median.toLocaleString(), 
            class: '',
            tooltip: 'Middle value - half have more, half have less'
        },
        { 
            label: 'Highest', 
            value: max.toLocaleString(), 
            class: 'success',
            tooltip: 'Item with most jobs - highest complexity'
        },
        { 
            label: 'Lowest', 
            value: min.toLocaleString(), 
            class: 'warning',
            tooltip: 'Item with fewest jobs - lowest complexity'
        },
    ];
    
    // Add type-specific stats
    if (chartType === 'applications' || chartType === 'folders') {
        const top3Total = counts.slice(0, 3).reduce((sum, val) => sum + val, 0);
        const top3Percent = ((top3Total / total) * 100).toFixed(1);
        stats.push({ 
            label: 'Top 3 Share', 
            value: `${top3Percent}%`, 
            class: 'primary',
            tooltip: 'Percentage of jobs in top 3 items - measures concentration'
        });
    }
    
    statsGrid.innerHTML = stats.map((stat, index) => `
        <div class="stat-item" data-stat-index="${index}">
            <span class="stat-item-label">
                ${stat.label}
                <i class="fas fa-info-circle stat-info-icon" data-tooltip-trigger="${index}"></i>
            </span>
            <span class="stat-item-value ${stat.class}">${stat.value}</span>
            <div class="stat-tooltip" data-tooltip="${index}">${stat.tooltip}</div>
        </div>
    `).join('');
    
    // Add click event listeners to info icons
    stats.forEach((stat, index) => {
        const icon = statsGrid.querySelector(`[data-tooltip-trigger="${index}"]`);
        const tooltip = statsGrid.querySelector(`[data-tooltip="${index}"]`);
        
        if (icon && tooltip) {
            icon.addEventListener('click', (e) => {
                e.stopPropagation();
                
                // Hide all other tooltips
                document.querySelectorAll('.stat-tooltip.show').forEach(t => {
                    if (t !== tooltip) t.classList.remove('show');
                });
                
                // Toggle this tooltip
                tooltip.classList.toggle('show');
            });
        }
    });
    
    // Close tooltip when clicking outside
    document.addEventListener('click', () => {
        document.querySelectorAll('.stat-tooltip.show').forEach(t => {
            t.classList.remove('show');
        });
    });
}

/**
 * Render data table with detailed breakdown
 */
function renderChartDataTable(data, chartType) {
    const tableContainer = document.getElementById('chart-data-table');
    
    const total = data.reduce((sum, item) => sum + item.count, 0);
    const maxCount = Math.max(...data.map(item => item.count));
    
    const html = `
        <table>
            <thead>
                <tr>
                    <th>#</th>
                    <th>Name</th>
                    <th>Count</th>
                    <th>%</th>
                    <th>Bar</th>
                </tr>
            </thead>
            <tbody>
                ${data.map((item, index) => {
                    const name = item.name || item.application || item.folder_name || item.appl_type || item.task_type || 'Unknown';
                    const count = item.count;
                    const percent = ((count / total) * 100).toFixed(1);
                    const barWidth = (count / maxCount) * 100;
                    
                    return `
                        <tr>
                            <td class="rank-cell">${index + 1}</td>
                            <td title="${escapeHtml(name)}">${escapeHtml(name)}</td>
                            <td>${count.toLocaleString()}</td>
                            <td class="percent-cell">${percent}%</td>
                            <td class="bar-cell">
                                <div class="mini-bar">
                                    <div class="mini-bar-fill" style="width: ${barWidth}%"></div>
                                </div>
                            </td>
                        </tr>
                    `;
                }).join('')}
            </tbody>
        </table>
    `;
    
    tableContainer.innerHTML = html;
}

/**
 * Render insights and recommendations
 */
function renderChartInsights(data, chartType) {
    const insightsContainer = document.getElementById('chart-insights');
    
    const insights = generateInsights(data, chartType);
    
    const html = insights.map(insight => `
        <div class="insight-item">
            <span class="insight-icon ${insight.type}">
                <i class="fas ${insight.icon}"></i>
            </span>
            <span class="insight-text">${insight.text}</span>
        </div>
    `).join('');
    
    insightsContainer.innerHTML = html;
}

/**
 * Generate insights based on data analysis
 */
function generateInsights(data, chartType) {
    const insights = [];
    const total = data.reduce((sum, item) => sum + item.count, 0);
    const counts = data.map(item => item.count);
    const average = total / counts.length;
    const median = calculateMedian(counts);
    
    // 1. Distribution Analysis
    const top3Total = counts.slice(0, Math.min(3, counts.length)).reduce((sum, val) => sum + val, 0);
    const top3Percent = ((top3Total / total) * 100).toFixed(1);
    
    if (top3Percent > 50) {
        insights.push({
            type: 'warning',
            icon: 'fa-chart-pie',
            text: `<strong>Concentrated Workload:</strong> Top 3 items contain ${top3Percent}% of jobs. <em>Recommendation:</em> Prioritize these high-volume items for early migration to achieve quick wins.`
        });
    } else if (top3Percent < 30) {
        insights.push({
            type: 'success',
            icon: 'fa-balance-scale',
            text: `<strong>Well-Distributed:</strong> Top 3 items represent only ${top3Percent}% of total. <em>Recommendation:</em> Consider parallel migration across multiple items for faster completion.`
        });
    } else {
        insights.push({
            type: 'info',
            icon: 'fa-chart-bar',
            text: `<strong>Moderate Distribution:</strong> Top 3 items account for ${top3Percent}% of jobs. <em>Recommendation:</em> Balance between focusing on high-volume items and parallel processing.`
        });
    }
    
    // 2. Complexity Variance
    const max = Math.max(...counts);
    const min = Math.min(...counts);
    const variance = max / min;
    
    if (variance > 3) {
        const highComplexity = data.filter(item => item.count > average * 1.5).length;
        insights.push({
            type: 'warning',
            icon: 'fa-layer-group',
            text: `<strong>High Complexity Variance:</strong> ${highComplexity} items exceed 1.5x average size (${Math.round(average).toLocaleString()} jobs). <em>Recommendation:</em> Allocate additional resources and testing time for larger items.`
        });
    }
    
    // 3. Average vs Median Analysis
    const avgMedianRatio = average / median;
    if (avgMedianRatio > 1.2) {
        insights.push({
            type: 'info',
            icon: 'fa-chart-line',
            text: `<strong>Skewed Distribution:</strong> Average (${Math.round(average).toLocaleString()}) exceeds median (${median.toLocaleString()}) by ${((avgMedianRatio - 1) * 100).toFixed(0)}%. <em>Insight:</em> Few large items are pulling the average up - most items are smaller than average.`
        });
    }
    
    // 4. Type-Specific Insights
    if (chartType === 'appl_type') {
        const fileTransfer = data.find(item => 
            (item.appl_type || '').toLowerCase().includes('file') || 
            (item.appl_type || '').toLowerCase().includes('trans')
        );
        if (fileTransfer && fileTransfer.count > 100) {
            insights.push({
                type: 'info',
                icon: 'fa-exchange-alt',
                text: `<strong>File Transfer Workload:</strong> ${fileTransfer.count.toLocaleString()} file transfer jobs identified. <em>Recommendation:</em> Plan dedicated file transfer infrastructure and security review.`
            });
        }
    }
    
    if (chartType === 'applications') {
        if (data.length > 20) {
            insights.push({
                type: 'warning',
                icon: 'fa-sitemap',
                text: `<strong>Large Application Portfolio:</strong> ${data.length} applications detected. <em>Recommendation:</em> Group related applications and migrate in 3-5 application batches to manage complexity.`
            });
        } else {
            insights.push({
                type: 'success',
                icon: 'fa-cube',
                text: `<strong>Manageable Scope:</strong> ${data.length} applications identified. <em>Recommendation:</em> Migrate by application to maintain logical groupings and simplify testing.`
            });
        }
    }
    
    if (chartType === 'folders') {
        const avgJobsPerFolder = Math.round(average);
        if (avgJobsPerFolder > 1000) {
            insights.push({
                type: 'warning',
                icon: 'fa-folder-open',
                text: `<strong>Large Folders:</strong> Average ${avgJobsPerFolder.toLocaleString()} jobs per folder. <em>Recommendation:</em> Consider breaking down large folders into smaller, manageable units before migration.`
            });
        } else {
            insights.push({
                type: 'success',
                icon: 'fa-folder',
                text: `<strong>Optimal Folder Size:</strong> Average ${avgJobsPerFolder.toLocaleString()} jobs per folder. <em>Recommendation:</em> Folder-based migration batches are well-sized for efficient processing.`
            });
        }
    }
    
    if (chartType === 'task_type') {
        const commandJobs = data.find(item => 
            (item.task_type || '').toLowerCase().includes('command') ||
            (item.task_type || '').toLowerCase().includes('script')
        );
        if (commandJobs && commandJobs.count > total * 0.3) {
            insights.push({
                type: 'info',
                icon: 'fa-terminal',
                text: `<strong>Script-Heavy Workload:</strong> ${((commandJobs.count / total) * 100).toFixed(0)}% are command/script jobs. <em>Recommendation:</em> Ensure script compatibility and environment setup are prioritized.`
            });
        }
    }
    
    return insights;
}

/**
 * Calculate median value
 */
function calculateMedian(values) {
    const sorted = [...values].sort((a, b) => a - b);
    const mid = Math.floor(sorted.length / 2);
    return sorted.length % 2 === 0 
        ? Math.round((sorted[mid - 1] + sorted[mid]) / 2)
        : sorted[mid];
}

/**
 * Export chart data as CSV
 */
function exportChartData() {
    if (!currentChartData) return;
    
    const headers = ['Rank', 'Name', 'Count', 'Percentage'];
    const total = currentChartData.reduce((sum, item) => sum + item.count, 0);
    
    const rows = currentChartData.map((item, index) => {
        const name = item.name || item.application || item.folder_name || item.appl_type || item.task_type || 'Unknown';
        const count = item.count;
        const percent = ((count / total) * 100).toFixed(2);
        return [index + 1, name, count, percent];
    });
    
    const csv = [
        headers.join(','),
        ...rows.map(row => row.map(cell => `"${cell}"`).join(','))
    ].join('\n');
    
    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `chart-data-${currentChartType}-${new Date().toISOString().split('T')[0]}.csv`;
    a.click();
    URL.revokeObjectURL(url);
}

/**
 * Make chart cards clickable
 */
function makeChartsClickable() {
    // This will be called after dashboard charts are rendered
    const chartCards = document.querySelectorAll('.chart-card');
    
    chartCards.forEach(card => {
        const chartBody = card.querySelector('.chart-body');
        if (!chartBody) return;
        
        const chartId = chartBody.id;
        let chartType = '';
        let title = '';
        let subtitle = '';
        
        // Determine chart type from ID
        if (chartId === 'chart-appl-types') {
            chartType = 'appl_type';
            title = 'APPL Type Distribution';
            subtitle = 'Detailed breakdown of jobs by application type';
        } else if (chartId === 'chart-task-types') {
            chartType = 'task_type';
            title = 'Task Type Distribution';
            subtitle = 'Detailed breakdown of jobs by task type';
        } else if (chartId === 'chart-applications') {
            chartType = 'applications';
            title = 'Top Applications';
            subtitle = 'Applications with most jobs - detailed analysis';
        } else if (chartId === 'chart-folders') {
            chartType = 'folders';
            title = 'Top Folders';
            subtitle = 'Folders with most jobs - detailed analysis';
        }
        
        if (chartType) {
            card.style.cursor = 'pointer';
            card.addEventListener('click', () => {
                // Get data from the chart (stored globally when rendered)
                const data = window[`chartData_${chartType}`];
                if (data && data.length > 0) {
                    openChartDetail(chartType, data, title, subtitle);
                }
            });
        }
    });
}
