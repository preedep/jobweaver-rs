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
    
    // Show professional loading state
    showSpinner('dependency-graph-container', 'Loading dependency graph...');
    
    try {
        const response = await fetch(`${API_BASE}/jobs/${jobId}/dependencies`, {
            headers: {
                'Authorization': `Bearer ${authToken}`
            }
        });
        
        const result = await response.json();
        
        if (result.success) {
            currentGraphData = result.data;
            
            // Small delay for smooth transition
            setTimeout(() => {
                renderDependencyGraph(currentGraphData, currentFilter, currentDepth, currentViewMode);
                updateDependencyStats(currentGraphData.stats);
                initializeGraphControls();
                
                // Add smooth appear animation
                smoothAppear(container);
            }, 150);
        } else {
            container.innerHTML = '<div class="content-placeholder"><div class="placeholder-text">Failed to load dependency graph</div></div>';
        }
    } catch (error) {
        console.error('Error loading dependency graph:', error);
        container.innerHTML = '<div class="content-placeholder"><div class="placeholder-text">Error loading dependency graph</div><div class="placeholder-subtext">Please try again</div></div>';
    }
}

/**
 * Apply scope filter to nodes and edges
 */
function applyScopeFilter(nodes, edges, filter, rootJobId) {
    let filteredNodes = nodes;
    let filteredEdges = edges;
    
    if (filter === 'internal') {
        filteredNodes = nodes.filter(n => n.is_internal);
        const nodeIds = new Set(filteredNodes.map(n => n.id));
        filteredEdges = edges.filter(e => 
            nodeIds.has(e.source_id) && nodeIds.has(e.target_id)
        );
    } else if (filter === 'external') {
        filteredNodes = nodes.filter(n => !n.is_internal || n.id === rootJobId);
        const nodeIds = new Set(filteredNodes.map(n => n.id));
        filteredEdges = edges.filter(e => {
            if (!nodeIds.has(e.source_id) || !nodeIds.has(e.target_id)) return false;
            const source = filteredNodes.find(n => n.id === e.source_id);
            const target = filteredNodes.find(n => n.id === e.target_id);
            return !source.is_internal || !target.is_internal;
        });
    }
    
    return { nodes: filteredNodes, edges: filteredEdges };
}

/**
 * Get node color configuration
 */
function getNodeColors(isRoot, isInternal) {
    if (isRoot) {
        return {
            background: '#2196F3',
            border: '#1976D2',
            highlight: { background: '#1976D2', border: '#0D47A1' }
        };
    }
    
    if (isInternal) {
        return {
            background: '#4CAF50',
            border: '#388E3C',
            highlight: { background: '#66BB6A', border: '#2E7D32' }
        };
    }
    
    return {
        background: '#FF9800',
        border: '#F57C00',
        highlight: { background: '#FFB74D', border: '#E65100' }
    };
}

/**
 * Create vis.js node configuration
 */
function createVisNode(node, rootJobId) {
    const isRoot = node.id === rootJobId;
    const isInternal = node.is_internal;
    
    return {
        id: node.id,
        label: truncateText(node.job_name, 25),
        title: `${node.job_name}\n${node.folder_name}\n${node.datacenter}`,
        color: getNodeColors(isRoot, isInternal),
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
        level: undefined
    };
}

/**
 * Create vis.js edge configuration
 */
function createVisEdge(edge, nodeMap) {
    const source = nodeMap.get(edge.source_id);
    const target = nodeMap.get(edge.target_id);
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
}

/**
 * Get layout configuration based on view mode
 */
function getLayoutConfig(viewMode) {
    if (viewMode === 'hierarchical') {
        return {
            hierarchical: {
                enabled: true,
                direction: 'UD',
                sortMethod: 'directed',
                nodeSpacing: 150,
                levelSeparation: 150,
                treeSpacing: 200
            }
        };
    }
    
    if (viewMode === 'radial') {
        return {
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
    }
    
    return { improvedLayout: true };
}

/**
 * Create vis.js network options
 */
function createNetworkOptions(viewMode) {
    return {
        layout: getLayoutConfig(viewMode),
        physics: { enabled: false },
        interaction: {
            hover: true,
            tooltipDelay: 100,
            zoomView: true,
            dragView: true
        },
        nodes: {
            shape: 'box',
            margin: 10,
            widthConstraint: { maximum: 200 }
        },
        edges: {
            smooth: {
                type: 'cubicBezier',
                forceDirection: 'vertical'
            }
        }
    };
}

/**
 * Setup network event handlers
 */
function setupNetworkEvents(network, filteredNodes) {
    network.on('click', function(params) {
        if (params.nodes.length > 0) {
            const nodeId = params.nodes[0];
            const node = filteredNodes.find(n => n.id === nodeId);
            if (node) {
                console.log('Clicked node:', node);
            }
        }
    });
}

/**
 * Fit network to container with animation
 */
function fitNetworkToContainer(network) {
    setTimeout(() => {
        network.fit({
            animation: {
                duration: 500,
                easingFunction: 'easeInOutQuad'
            }
        });
    }, 100);
}

/**
 * Render dependency graph using vis.js hierarchical directed graph
 */
function renderDependencyGraph(graphData, filter = 'all', maxDepth = 2, viewMode = 'hierarchical') {
    const container = document.getElementById('dependency-graph-container');
    if (!container) return;
    
    const depthFiltered = filterGraphByDepth(graphData, maxDepth);
    const { nodes: filteredNodes, edges: filteredEdges } = applyScopeFilter(
        depthFiltered.nodes,
        depthFiltered.edges,
        filter,
        graphData.root_job_id
    );
    
    if (filteredNodes.length === 0) {
        container.innerHTML = '<div style="display: flex; align-items: center; justify-content: center; height: 100%; color: #999;">No dependencies to display</div>';
        return;
    }
    
    container.innerHTML = '';
    
    const nodeMap = new Map(filteredNodes.map(n => [n.id, n]));
    const visNodes = filteredNodes.map(node => createVisNode(node, graphData.root_job_id));
    const visEdges = filteredEdges.map(edge => createVisEdge(edge, nodeMap));
    
    const data = {
        nodes: new vis.DataSet(visNodes),
        edges: new vis.DataSet(visEdges)
    };
    
    if (window.dependencyNetwork) {
        window.dependencyNetwork.destroy();
    }
    
    window.dependencyNetwork = new vis.Network(container, data, createNetworkOptions(viewMode));
    setupNetworkEvents(window.dependencyNetwork, filteredNodes);
    fitNetworkToContainer(window.dependencyNetwork);
}

/**
 * Update dependency statistics display
 */
function updateDependencyStats(stats) {
    document.getElementById('dep-total').textContent = stats.total_dependencies.toLocaleString();
    document.getElementById('dep-internal').textContent = stats.internal_dependencies.toLocaleString();
    document.getElementById('dep-external').textContent = stats.external_dependencies.toLocaleString();
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
 * Get original node color configuration
 */
function getOriginalNodeColor(nodeId, originalNode, rootJobId) {
    const isRoot = nodeId === rootJobId;
    const isInternal = originalNode.is_internal;
    
    return {
        background: isRoot ? '#2196F3' : (isInternal ? '#4CAF50' : '#FF9800'),
        border: isRoot ? '#1976D2' : (isInternal ? '#388E3C' : '#F57C00')
    };
}

/**
 * Get highlight color configuration
 */
function getHighlightColor() {
    return {
        background: '#FFD700',
        border: '#FFA500'
    };
}

/**
 * Reset node to original color
 */
function resetNodeColor(nodes, node, originalNode, rootJobId) {
    if (!originalNode) return;
    
    nodes.update({
        id: node.id,
        color: getOriginalNodeColor(node.id, originalNode, rootJobId),
        opacity: 1
    });
}

/**
 * Reset all nodes to original colors
 */
function resetAllNodeColors(nodes, graphData) {
    nodes.forEach(node => {
        const originalNode = graphData.nodes.find(n => n.id === node.id);
        if (originalNode) {
            resetNodeColor(nodes, node, originalNode, graphData.root_job_id);
        }
    });
}

/**
 * Check if node matches search term
 */
function nodeMatchesSearch(originalNode, searchTerm) {
    return originalNode && 
           originalNode.job_name.toLowerCase().includes(searchTerm.toLowerCase());
}

/**
 * Highlight matching node
 */
function highlightNode(nodes, nodeId) {
    nodes.update({
        id: nodeId,
        color: getHighlightColor(),
        opacity: 1
    });
}

/**
 * Dim non-matching node
 */
function dimNode(nodes, node, originalNode, rootJobId) {
    nodes.update({
        id: node.id,
        color: getOriginalNodeColor(node.id, originalNode, rootJobId),
        opacity: 0.3
    });
}

/**
 * Focus on first matching node
 */
function focusOnNode(network, nodeId) {
    network.focus(nodeId, {
        scale: 1.5,
        animation: true
    });
}

/**
 * Search and highlight nodes
 */
function searchAndHighlight(searchTerm) {
    if (!window.dependencyNetwork || !currentGraphData) return;
    
    const nodes = window.dependencyNetwork.body.data.nodes;
    
    if (!searchTerm) {
        resetAllNodeColors(nodes, currentGraphData);
        return;
    }
    
    const matchingNodes = [];
    const searchLower = searchTerm.toLowerCase();
    
    nodes.forEach(node => {
        const originalNode = currentGraphData.nodes.find(n => n.id === node.id);
        
        if (nodeMatchesSearch(originalNode, searchLower)) {
            matchingNodes.push(node.id);
            highlightNode(nodes, node.id);
        } else if (originalNode) {
            dimNode(nodes, node, originalNode, currentGraphData.root_job_id);
        }
    });
    
    if (matchingNodes.length > 0) {
        focusOnNode(window.dependencyNetwork, matchingNodes[0]);
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
