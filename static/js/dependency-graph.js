/**
 * Dependency Graph Visualization
 * Simple SVG-based dependency graph renderer
 */

let currentGraphData = null;
let currentFilter = 'all';

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
            renderDependencyGraph(currentGraphData, currentFilter);
            updateDependencyStats(currentGraphData.stats);
        } else {
            container.innerHTML = '<div class="error">Failed to load dependency graph</div>';
        }
    } catch (error) {
        console.error('Error loading dependency graph:', error);
        container.innerHTML = '<div class="error">Error loading dependency graph</div>';
    }
}

/**
 * Render dependency graph using simple SVG
 */
function renderDependencyGraph(graphData, filter = 'all') {
    const container = document.getElementById('dependency-graph-container');
    if (!container) return;
    
    // Filter nodes and edges based on filter
    let nodes = graphData.nodes;
    let edges = graphData.edges;
    
    if (filter === 'internal') {
        nodes = nodes.filter(n => n.is_internal);
        edges = edges.filter(e => {
            const source = nodes.find(n => n.id === e.source_id);
            const target = nodes.find(n => n.id === e.target_id);
            return source && target;
        });
    } else if (filter === 'external') {
        nodes = nodes.filter(n => !n.is_internal || n.id === graphData.root_job_id);
        edges = edges.filter(e => {
            const source = nodes.find(n => n.id === e.source_id);
            const target = nodes.find(n => n.id === e.target_id);
            return source && target && (!source.is_internal || !target.is_internal);
        });
    }
    
    if (nodes.length === 0) {
        container.innerHTML = '<div style="display: flex; align-items: center; justify-content: center; height: 100%; color: #999;">No dependencies to display</div>';
        return;
    }
    
    // Simple force-directed layout
    const width = container.clientWidth;
    const height = 400;
    const centerX = width / 2;
    const centerY = height / 2;
    
    // Position nodes in a circular layout
    const rootNode = nodes.find(n => n.id === graphData.root_job_id);
    const otherNodes = nodes.filter(n => n.id !== graphData.root_job_id);
    
    // Root in center
    rootNode.x = centerX;
    rootNode.y = centerY;
    
    // Others in circle around root
    const radius = Math.min(width, height) * 0.35;
    otherNodes.forEach((node, i) => {
        const angle = (2 * Math.PI * i) / otherNodes.length;
        node.x = centerX + radius * Math.cos(angle);
        node.y = centerY + radius * Math.sin(angle);
    });
    
    // Create SVG
    let svg = `<svg width="${width}" height="${height}" style="background: white;">`;
    
    // Draw edges first (so they appear behind nodes)
    edges.forEach(edge => {
        const source = nodes.find(n => n.id === edge.source_id);
        const target = nodes.find(n => n.id === edge.target_id);
        if (source && target) {
            const color = (source.is_internal && target.is_internal) ? '#4CAF50' : '#FF9800';
            svg += `<line x1="${source.x}" y1="${source.y}" x2="${target.x}" y2="${target.y}" 
                    stroke="${color}" stroke-width="2" opacity="0.6" marker-end="url(#arrowhead-${color.replace('#', '')})"/>`;
        }
    });
    
    // Define arrow markers
    svg += `<defs>
        <marker id="arrowhead-4CAF50" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto">
            <polygon points="0 0, 10 3, 0 6" fill="#4CAF50" />
        </marker>
        <marker id="arrowhead-FF9800" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto">
            <polygon points="0 0, 10 3, 0 6" fill="#FF9800" />
        </marker>
    </defs>`;
    
    // Draw nodes
    nodes.forEach(node => {
        const isRoot = node.id === graphData.root_job_id;
        const color = isRoot ? '#2196F3' : (node.is_internal ? '#4CAF50' : '#FF9800');
        const radius = isRoot ? 8 : 6;
        
        // Node circle
        svg += `<circle cx="${node.x}" cy="${node.y}" r="${radius}" fill="${color}" stroke="white" stroke-width="2">
            <title>${node.job_name}\n${node.folder_name}\n${node.datacenter}</title>
        </circle>`;
        
        // Node label
        const labelY = node.y + (isRoot ? -15 : 20);
        svg += `<text x="${node.x}" y="${labelY}" text-anchor="middle" font-size="11" fill="#333" font-weight="${isRoot ? 'bold' : 'normal'}">
            ${truncateText(node.job_name, 20)}
        </text>`;
    });
    
    // Add legend inside SVG
    const legendY = height - 25;
    svg += `<g>
        <circle cx="20" cy="${legendY}" r="5" fill="#2196F3"/>
        <text x="30" y="${legendY + 4}" font-size="11" fill="#666">Root Job</text>
        
        <circle cx="100" cy="${legendY}" r="5" fill="#4CAF50"/>
        <text x="110" y="${legendY + 4}" font-size="11" fill="#666">Internal</text>
        
        <circle cx="170" cy="${legendY}" r="5" fill="#FF9800"/>
        <text x="180" y="${legendY + 4}" font-size="11" fill="#666">External</text>
    </g>`;
    
    svg += '</svg>';
    
    container.innerHTML = svg;
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
 * Initialize dependency graph filter buttons
 */
function initializeDependencyGraphFilters() {
    const buttons = document.querySelectorAll('.dependency-filter-buttons button');
    buttons.forEach(button => {
        button.addEventListener('click', () => {
            // Update active state
            buttons.forEach(b => b.classList.remove('active'));
            button.classList.add('active');
            
            // Update filter and re-render
            currentFilter = button.dataset.filter;
            if (currentGraphData) {
                renderDependencyGraph(currentGraphData, currentFilter);
            }
        });
    });
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    initializeDependencyGraphFilters();
});
