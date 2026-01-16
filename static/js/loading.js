/**
 * Professional Loading States Manager
 * Enterprise-grade UX for data loading
 */

class LoadingManager {
    constructor() {
        this.activeLoaders = new Set();
        this.createGlobalOverlay();
    }

    /**
     * Create global loading overlay
     */
    createGlobalOverlay() {
        if (document.getElementById('global-loading-overlay')) return;
        
        const overlay = document.createElement('div');
        overlay.id = 'global-loading-overlay';
        overlay.className = 'loading-overlay-modern';
        overlay.innerHTML = `
            <div class="loading-content">
                <div class="loading-icon">
                    <div class="spinner spinner-lg"></div>
                </div>
                <div class="loading-title">Loading...</div>
                <div class="loading-message">Please wait while we fetch your data</div>
            </div>
        `;
        document.body.appendChild(overlay);
    }

    /**
     * Show global loading overlay
     */
    showGlobalLoading(title = 'Loading...', message = 'Please wait while we fetch your data') {
        const overlay = document.getElementById('global-loading-overlay');
        if (overlay) {
            overlay.querySelector('.loading-title').textContent = title;
            overlay.querySelector('.loading-message').textContent = message;
            overlay.classList.add('active');
        }
    }

    /**
     * Hide global loading overlay
     */
    hideGlobalLoading() {
        const overlay = document.getElementById('global-loading-overlay');
        if (overlay) {
            overlay.classList.remove('active');
        }
    }

    /**
     * Show skeleton loading for a container
     */
    showSkeletonTable(containerId, rows = 5) {
        const container = document.getElementById(containerId);
        if (!container) return;

        let html = '<div class="skeleton-table">';
        for (let i = 0; i < rows; i++) {
            html += `
                <div class="skeleton-table-row">
                    <div class="skeleton skeleton-table-cell"></div>
                    <div class="skeleton skeleton-table-cell"></div>
                    <div class="skeleton skeleton-table-cell"></div>
                    <div class="skeleton skeleton-table-cell"></div>
                    <div class="skeleton skeleton-table-cell"></div>
                </div>
            `;
        }
        html += '</div>';
        container.innerHTML = html;
    }

    /**
     * Show skeleton loading for stat cards
     */
    showSkeletonCards(containerId, count = 4) {
        const container = document.getElementById(containerId);
        if (!container) return;

        let html = '';
        for (let i = 0; i < count; i++) {
            html += `
                <div class="skeleton-stat-card">
                    <div class="skeleton skeleton-stat-value"></div>
                    <div class="skeleton skeleton-stat-label"></div>
                </div>
            `;
        }
        container.innerHTML = html;
    }

    /**
     * Show loading spinner in container
     */
    showSpinner(containerId, message = 'Loading...') {
        const container = document.getElementById(containerId);
        if (!container) return;

        container.innerHTML = `
            <div class="spinner-container">
                <div class="spinner"></div>
                <div class="spinner-text">${message}</div>
            </div>
        `;
    }

    /**
     * Show loading with dots animation
     */
    showLoadingDots(containerId, message = 'Loading') {
        const container = document.getElementById(containerId);
        if (!container) return;

        container.innerHTML = `
            <div class="spinner-container">
                <div class="spinner-text">
                    ${message}
                    <span class="loading-dots">
                        <span></span>
                        <span></span>
                        <span></span>
                    </span>
                </div>
            </div>
        `;
    }

    /**
     * Add smooth fade-in animation to element
     */
    fadeIn(element) {
        if (element) {
            element.classList.add('fade-in');
        }
    }

    /**
     * Add smooth appear animation to element
     */
    smoothAppear(element) {
        if (element) {
            element.classList.add('smooth-appear');
        }
    }

    /**
     * Wrap async function with loading state
     */
    async withLoading(asyncFn, options = {}) {
        const {
            showGlobal = false,
            title = 'Loading...',
            message = 'Please wait...',
            onStart = null,
            onComplete = null
        } = options;

        try {
            if (showGlobal) {
                this.showGlobalLoading(title, message);
            }
            if (onStart) onStart();

            const result = await asyncFn();

            if (onComplete) onComplete(result);
            return result;
        } finally {
            if (showGlobal) {
                // Small delay for smooth transition
                setTimeout(() => this.hideGlobalLoading(), 200);
            }
        }
    }

    /**
     * Show progress bar
     */
    showProgress(containerId, progress = 0) {
        const container = document.getElementById(containerId);
        if (!container) return;

        if (!container.querySelector('.progress-bar-container')) {
            container.innerHTML = `
                <div class="progress-bar-container">
                    <div class="progress-bar" style="width: ${progress}%"></div>
                </div>
            `;
        } else {
            const bar = container.querySelector('.progress-bar');
            if (bar) {
                bar.style.width = `${progress}%`;
            }
        }
    }

    /**
     * Show indeterminate progress
     */
    showIndeterminateProgress(containerId) {
        const container = document.getElementById(containerId);
        if (!container) return;

        container.innerHTML = `
            <div class="progress-bar-container">
                <div class="progress-bar progress-bar-indeterminate"></div>
            </div>
        `;
    }
}

// Global instance
const loadingManager = new LoadingManager();

// Helper functions for easy access
function showLoading(title, message) {
    loadingManager.showGlobalLoading(title, message);
}

function hideLoading() {
    loadingManager.hideGlobalLoading();
}

function showSkeletonTable(containerId, rows = 5) {
    loadingManager.showSkeletonTable(containerId, rows);
}

function showSkeletonCards(containerId, count = 4) {
    loadingManager.showSkeletonCards(containerId, count);
}

function showSpinner(containerId, message = 'Loading...') {
    loadingManager.showSpinner(containerId, message);
}

function fadeIn(element) {
    loadingManager.fadeIn(element);
}

function smoothAppear(element) {
    loadingManager.smoothAppear(element);
}

async function withLoading(asyncFn, options = {}) {
    return loadingManager.withLoading(asyncFn, options);
}
