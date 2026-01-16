/**
 * Dependency Graph Visualization
 * Simple SVG-based dependency graph renderer
 */

let currentGraphData = null;
let currentFilter = 'all';
let currentDepth = 2;
let currentViewMode = 'hierarchical';
let highlightedNodes = [];

/**
 * Calculate depth/level for each node using BFS
 */
function calculateNodeDepths(graphData) {
    const depths = new Map();
    const queue = [{ id: graphData.root_job_id, depth: 0 }];
    const visited = new Set();
    
    depths.set(graphData.root_job_id, 0);
    visited.add(graphData.root_job_id);
    
    while (queue.length > 0) {
        const { id, depth } = queue.shift();
        
        // Find all edges from this node
        graphData.edges.forEach(edge => {
            if (edge.source_id === id && !visited.has(edge.target_id)) {
                depths.set(edge.target_id, depth + 1);
                visited.add(edge.target_id);
                queue.push({ id: edge.target_id, depth: depth + 1 });
            }
        });
    }
    
    return depths;
}

/**
 * Filter graph by depth
 */
function filterGraphByDepth(graphData, maxDepth) {
    const depths = calculateNodeDepths(graphData);
    
    const filteredNodes = graphData.nodes.filter(node => {
        const depth = depths.get(node.id);
        return depth !== undefined && depth <= maxDepth;
    });
    
    const nodeIds = new Set(filteredNodes.map(n => n.id));
    const filteredEdges = graphData.edges.filter(edge => 
        nodeIds.has(edge.source_id) && nodeIds.has(edge.target_id)
    );
    
    return { nodes: filteredNodes, edges: filteredEdges };
}

/**
 * Load and render dependency graph for a job
 */
async function loadDependencyGraph(jobId) {
    const container = document.getElementById('dependency-graph-container');
    if (!container) return;
    
    // Show loading
    container.innerHTML = '<div class="loading" style="display: flex; align-items: center; justify-content: center; height: 100%;"><i class="fas fa-spinner fa-spin"></i> Loading dependency graph...</div>';
    
    try {
        const response = await fetch(`${API_BASE}/jobs/${jobId}/dependencies`, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
        });
        
        const result = await response.json();
        
        if (result.success) {
            currentGraphData = result.data;
            renderDependencyGraph(currentGraphData, currentFilter, currentDepth, currentViewMode);
            updateDependencyStats(currentGraphData.stats);
            initializeGraphControls();
        } else {
            container.innerHTML = '<div class="error">Failed to load dependency graph</div>';
        }
    } catch (error) {
        console.error('Error loading dependency graph:', error);
        container.innerHTML = '<div class="error">Error loading dependency graph</div>';
    }
}

/**
 * Render dependency graph using vis.js hierarchical directed graph
 */
function renderDependencyGraph(graphData, filter = 'all', maxDepth = 2, viewMode = 'hierarchical') {
    const container = document.getElementById('dependency-graph-container');
    if (!container) return;
    
    // Apply depth filter first
    let depthFiltered = filterGraphByDepth(graphData, maxDepth);
    
    // Then apply scope filter
    let filteredNodes = depthFiltered.nodes;
    let filteredEdges = depthFiltered.edges;
    
    if (filter === 'internal') {
        filteredNodes = filteredNodes.filter(n => n.is_internal);
        filteredEdges = filteredEdges.filter(e => {
            const source = filteredNodes.find(n => n.id === e.source_id);
            const target = filteredNodes.find(n => n.id === e.target_id);
            return source && target;
        });
    } else if (filter === 'external') {
        filteredNodes = filteredNodes.filter(n => !n.is_internal || n.id === graphData.root_job_id);
        filteredEdges = filteredEdges.filter(e => {
            const source = filteredNodes.find(n => n.id === e.source_id);
            const target = filteredNodes.find(n => n.id === e.target_id);
            return source && target && (!source.is_internal || !target.is_internal);
        });
    }
    
    if (filteredNodes.length === 0) {
        container.innerHTML = '<div style="display: flex; align-items: center; justify-content: center; height: 100%; color: #999;">No dependencies to display</div>';
        return;
    }
    
    // Clear container
    container.innerHTML = '';
    
    // Prepare nodes for vis.js
    const visNodes = filteredNodes.map(node => {
        const isRoot = node.id === graphData.root_job_id;
        const isInternal = node.is_internal;
        
        return {
            id: node.id,
            label: truncateText(node.job_name, 25),
            title: `${node.job_name}\n${node.folder_name}\n${node.datacenter}`,
            color: {
                background: isRoot ? '#2196F3' : (isInternal ? '#4CAF50' : '#FF9800'),
                border: isRoot ? '#1976D2' : (isInternal ? '#388E3C' : '#F57C00'),
                highlight: {
                    background: isRoot ? '#1976D2' : (isInternal ? '#66BB6A' : '#FFB74D'),
                    border: isRoot ? '#0D47A1' : (isInternal ? '#2E7D32' : '#E65100')
                }
            },
            font: {
                color: '#ffffff',
                size: isRoot ? 14 : 12,
                face: 'Arial',
                bold: isRoot
            },
            shape: 'box',
            margin: 10,
            borderWidth: 2,
            shadow: true,
            level: undefined // Will be set based on hierarchy
        };
    });
    
    // Prepare edges for vis.js
    const visEdges = filteredEdges.map(edge => {
        const source = filteredNodes.find(n => n.id === edge.source_id);
        const target = filteredNodes.find(n => n.id === edge.target_id);
        const isInternal = source && target && source.is_internal && target.is_internal;
        
        return {
            from: edge.source_id,
            to: edge.target_id,
            arrows: 'to',
            color: {
                color: isInternal ? '#4CAF50' : '#FF9800',
                highlight: isInternal ? '#66BB6A' : '#FFB74D'
            },
            width: 2,
            smooth: {
                type: 'cubicBezier',
                forceDirection: 'vertical',
                roundness: 0.4
            },
            title: edge.condition_name
        };
    });
    
    // Create vis.js network
    const data = {
        nodes: new vis.DataSet(visNodes),
        edges: new vis.DataSet(visEdges)
    };
    
    // Configure layout based on view mode
    let layoutConfig = {};
    if (viewMode === 'hierarchical') {
        layoutConfig = {
            hierarchical: {
                enabled: true,
                direction: 'UD',
                sortMethod: 'directed',
                nodeSpacing: 150,
                levelSeparation: 150,
                treeSpacing: 200
            }
        };
    } else if (viewMode === 'radial') {
        layoutConfig = {
            hierarchical: {
                enabled: true,
                direction: 'UD',
                sortMethod: 'directed',
                nodeSpacing: 200,
                levelSeparation: 200,
                treeSpacing: 250,
                shakeTowards: 'roots'
            }
        };
    } else {
        layoutConfig = {
            improvedLayout: true
        };
    }
    
    const options = {
        layout: layoutConfig,
        physics: {
            enabled: false
        },
        interaction: {
            hover: true,
            tooltipDelay: 100,
            zoomView: true,
            dragView: true
        },
        nodes: {
            shape: 'box',
            margin: 10,
            widthConstraint: {
                maximum: 200
            }
        },
        edges: {
            smooth: {
                type: 'cubicBezier',
                forceDirection: 'vertical'
            }
        }
    };
    
    // Destroy existing network if any
    if (window.dependencyNetwork) {
        window.dependencyNetwork.destroy();
    }
    
    // Create new network
    window.dependencyNetwork = new vis.Network(container, data, options);
    
    // Add click event to nodes
    window.dependencyNetwork.on('click', function(params) {
        if (params.nodes.length > 0) {
            const nodeId = params.nodes[0];
            const node = filteredNodes.find(n => n.id === nodeId);
            if (node) {
                console.log('Clicked node:', node);
                // Could open job detail here
            }
        }
    });
    
    // Fit network to container
    setTimeout(() => {
        window.dependencyNetwork.fit({
            animation: {
                duration: 500,
                easingFunction: 'easeInOutQuad'
            }
        });
    }, 100);
}

/**
 * Update dependency statistics display
 */
function updateDependencyStats(stats) {
    document.getElementById('dep-total').textContent = stats.total_dependencies;
    document.getElementById('dep-internal').textContent = stats.internal_dependencies;
    document.getElementById('dep-external').textContent = stats.external_dependencies;
}

/**
 * Truncate text to max length
 */
function truncateText(text, maxLength) {
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength - 3) + '...';
}

/**
 * Initialize dependency graph controls
 */
function initializeGraphControls() {
    // Scope filter buttons
    const buttons = document.querySelectorAll('.dependency-filter-buttons button');
    buttons.forEach(button => {
        button.addEventListener('click', () => {
            buttons.forEach(b => b.classList.remove('active'));
            button.classList.add('active');
            currentFilter = button.dataset.filter;
            if (currentGraphData) {
                renderDependencyGraph(currentGraphData, currentFilter, currentDepth, currentViewMode);
            }
        });
    });
    
    // Depth control
    const depthControl = document.getElementById('graph-depth-control');
    if (depthControl) {
        depthControl.addEventListener('change', (e) => {
            currentDepth = parseInt(e.target.value);
            if (currentGraphData) {
                renderDependencyGraph(currentGraphData, currentFilter, currentDepth, currentViewMode);
            }
        });
    }
    
    // View mode
    const viewMode = document.getElementById('graph-view-mode');
    if (viewMode) {
        viewMode.addEventListener('change', (e) => {
            currentViewMode = e.target.value;
            if (currentGraphData) {
                renderDependencyGraph(currentGraphData, currentFilter, currentDepth, currentViewMode);
            }
        });
    }
    
    // Search
    const searchInput = document.getElementById('graph-search');
    if (searchInput) {
        searchInput.addEventListener('input', (e) => {
            searchAndHighlight(e.target.value);
        });
    }
}

/**
 * Search and highlight nodes
 */
function searchAndHighlight(searchTerm) {
    if (!window.dependencyNetwork || !currentGraphData) return;
    
    const nodes = window.dependencyNetwork.body.data.nodes;
    
    if (!searchTerm) {
        // Reset all nodes to original colors
        nodes.forEach(node => {
            const originalNode = currentGraphData.nodes.find(n => n.id === node.id);
            if (originalNode) {
                const isRoot = node.id === currentGraphData.root_job_id;
                const isInternal = originalNode.is_internal;
                nodes.update({
                    id: node.id,
                    color: {
                        background: isRoot ? '#2196F3' : (isInternal ? '#4CAF50' : '#FF9800'),
                        border: isRoot ? '#1976D2' : (isInternal ? '#388E3C' : '#F57C00')
                    }
                });
            }
        });
        return;
    }
    
    // Find matching nodes
    const matchingNodes = [];
    nodes.forEach(node => {
        const originalNode = currentGraphData.nodes.find(n => n.id === node.id);
        if (originalNode && originalNode.job_name.toLowerCase().includes(searchTerm.toLowerCase())) {
            matchingNodes.push(node.id);
            // Highlight matching node
            nodes.update({
                id: node.id,
                color: {
                    background: '#FFD700',
                    border: '#FFA500'
                }
            });
        } else {
            // Dim non-matching nodes
            const isRoot = node.id === currentGraphData.root_job_id;
            const isInternal = originalNode?.is_internal;
            nodes.update({
                id: node.id,
                color: {
                    background: isRoot ? '#2196F3' : (isInternal ? '#4CAF50' : '#FF9800'),
                    border: isRoot ? '#1976D2' : (isInternal ? '#388E3C' : '#F57C00')
                },
                opacity: 0.3
            });
        }
    });
    
    // Focus on first match
    if (matchingNodes.length > 0) {
        window.dependencyNetwork.focus(matchingNodes[0], {
            scale: 1.5,
            animation: true
        });
    }
}

/**
 * Reset graph view
 */
function resetGraphView() {
    if (window.dependencyNetwork) {
        window.dependencyNetwork.fit({
            animation: {
                duration: 500,
                easingFunction: 'easeInOutQuad'
            }
        });
    }
    
    // Reset search
    const searchInput = document.getElementById('graph-search');
    if (searchInput) {
        searchInput.value = '';
        searchAndHighlight('');
    }
}

/**
 * Fit graph to view
 */
function fitGraphView() {
    if (window.dependencyNetwork) {
        window.dependencyNetwork.fit({
            animation: {
                duration: 500,
                easingFunction: 'easeInOutQuad'
            }
        });
    }
}

/**
 * Export graph as image
 */
function exportGraphImage() {
    if (!window.dependencyNetwork) return;
    
    const canvas = window.dependencyNetwork.canvas.frame.canvas;
    const link = document.createElement('a');
    link.download = `dependency-graph-${Date.now()}.png`;
    link.href = canvas.toDataURL();
    link.click();
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    initializeGraphControls();
});
