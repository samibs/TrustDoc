// Main Application Controller
// Handles navigation, UI state, and coordination between components

import { initKeys } from './keys';
import { initDocuments } from './documents';
import { initSettings } from './settings';

let currentSection: string = 'keys';

// Initialize application
document.addEventListener('DOMContentLoaded', () => {
    initNavigation();
    initKeyboardShortcuts();
    initKeys();
    initDocuments();
    initSettings();
    showSection('keys');
});

// Navigation
function initNavigation() {
    const navItems = document.querySelectorAll('.nav-item');
    navItems.forEach(item => {
        item.addEventListener('click', () => {
            const section = item.getAttribute('data-section');
            if (section) {
                showSection(section);
            }
        });
    });
}

function showSection(section: string) {
    // Update navigation
    document.querySelectorAll('.nav-item').forEach(item => {
        if (item.getAttribute('data-section') === section) {
            item.classList.add('active');
        } else {
            item.classList.remove('active');
        }
    });

    // Update content sections
    document.querySelectorAll('.content-section').forEach(sec => {
        if (sec.id === `${section}-section`) {
            sec.classList.add('active');
        } else {
            sec.classList.remove('active');
        }
    });

    // Update toolbar buttons visibility
    updateToolbarVisibility(section);

    currentSection = section;
}

function updateToolbarVisibility(section: string) {
    const newKeyBtn = document.getElementById('newKeyBtn') as HTMLElement;
    const importKeyBtn = document.getElementById('importKeyBtn') as HTMLElement;
    const openDocBtn = document.getElementById('openDocBtn') as HTMLElement;
    const createDocBtn = document.getElementById('createDocBtn') as HTMLElement;
    const verifyBtn = document.getElementById('verifyBtn') as HTMLElement;
    const signBtn = document.getElementById('signBtn') as HTMLElement;

    // Hide all buttons first
    [newKeyBtn, importKeyBtn, openDocBtn, createDocBtn, verifyBtn, signBtn].forEach(btn => {
        if (btn) btn.style.display = 'none';
    });

    // Show relevant buttons based on section
    switch (section) {
        case 'keys':
            if (newKeyBtn) newKeyBtn.style.display = 'inline-flex';
            if (importKeyBtn) importKeyBtn.style.display = 'inline-flex';
            break;
        case 'documents':
            if (openDocBtn) openDocBtn.style.display = 'inline-flex';
            if (createDocBtn) createDocBtn.style.display = 'inline-flex';
            if (verifyBtn) verifyBtn.style.display = 'inline-flex';
            if (signBtn) signBtn.style.display = 'inline-flex';
            break;
        case 'settings':
            // No specific toolbar buttons for settings
            break;
    }
}

// Keyboard shortcuts
function initKeyboardShortcuts() {
    document.addEventListener('keydown', (e) => {
        // Ctrl/Cmd + key combinations
        if (e.ctrlKey || e.metaKey) {
            switch (e.key) {
                case 'n':
                    e.preventDefault();
                    if (currentSection === 'keys') {
                        const btn = document.getElementById('newKeyBtn');
                        if (btn) btn.click();
                    }
                    break;
                case 'o':
                    e.preventDefault();
                    if (currentSection === 'documents') {
                        const btn = document.getElementById('openDocBtn');
                        if (btn) btn.click();
                    }
                    break;
                case 'v':
                    e.preventDefault();
                    if (currentSection === 'documents') {
                        const btn = document.getElementById('verifyBtn');
                        if (btn) btn.click();
                    }
                    break;
            }
        }

        // Number keys for navigation
        if (!e.ctrlKey && !e.metaKey && !e.altKey) {
            switch (e.key) {
                case '1':
                    e.preventDefault();
                    showSection('keys');
                    break;
                case '2':
                    e.preventDefault();
                    showSection('documents');
                    break;
                case '3':
                    e.preventDefault();
                    showSection('settings');
                    break;
            }
        }
    });
}

// Status bar updates
export function updateStatus(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
    const statusBar = document.getElementById('statusBar');
    const statusText = document.getElementById('statusText');
    if (statusBar && statusText) {
        statusText.textContent = message;
        statusBar.className = `status-bar ${type}`;
        
        // Reset after 5 seconds for info messages
        if (type === 'info') {
            setTimeout(() => {
                statusText.textContent = 'Ready';
                statusBar.className = 'status-bar';
            }, 5000);
        }
    }
}

// Modal utilities
export function showModal(title: string, content: HTMLElement, footer?: HTMLElement) {
    const overlay = document.getElementById('modal-overlay');
    const modal = document.getElementById('modal');
    
    if (!overlay || !modal) return;

    modal.innerHTML = `
        <div class="modal-header">
            <h3 class="modal-title">${title}</h3>
            <button class="modal-close" onclick="closeModal()">×</button>
        </div>
        <div class="modal-body"></div>
        <div class="modal-footer"></div>
    `;

    const modalBody = modal.querySelector('.modal-body');
    const modalFooter = modal.querySelector('.modal-footer');
    
    if (modalBody) {
        modalBody.appendChild(content);
    }
    
    if (modalFooter && footer) {
        modalFooter.appendChild(footer);
    }

    overlay.style.display = 'flex';
    
    // Close on overlay click
    overlay.addEventListener('click', (e) => {
        if (e.target === overlay) {
            closeModal();
        }
    });
}

export function closeModal() {
    const overlay = document.getElementById('modal-overlay');
    if (overlay) {
        overlay.style.display = 'none';
    }
}

// Make closeModal available globally
(window as any).closeModal = closeModal;

// Export current section for other modules
export function getCurrentSection(): string {
    return currentSection;
}
