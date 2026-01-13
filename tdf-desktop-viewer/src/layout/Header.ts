// Header Component - Persistent top header with document info and actions

import { createInlineIcon } from '../components/Icon';

export interface BreadcrumbItem {
    label: string;
    onClick?: () => void;
}

export class Header {
    private container: HTMLElement;
    private breadcrumbs: BreadcrumbItem[] = [];
    private documentName: string | null = null;

    constructor(containerId: string = 'appHeader') {
        const existing = document.getElementById(containerId);
        if (existing) {
            this.container = existing;
        } else {
            this.container = document.createElement('header');
            this.container.id = containerId;
            this.container.className = 'app-header';
            this.container.setAttribute('role', 'banner');
        }
    }

    async render(): Promise<void> {
        const appIcon = await createInlineIcon('document', 24, 'header-app-icon', 'TDF Viewer');
        
        this.container.innerHTML = `
            <div class="header-content">
                <div class="header-left">
                    ${appIcon.outerHTML}
                    <h1 class="header-title">TDF Desktop Viewer</h1>
                    ${this.renderBreadcrumbs()}
                </div>
                <div class="header-right" id="headerActions"></div>
            </div>
        `;
    }

    private renderBreadcrumbs(): string {
        if (this.breadcrumbs.length === 0) {
            return '';
        }

        const items = this.breadcrumbs.map((crumb, index) => {
            const isLast = index === this.breadcrumbs.length - 1;
            const separator = index > 0 ? '<span class="breadcrumb-separator">/</span>' : '';
            
            if (isLast || !crumb.onClick) {
                return `${separator}<span class="breadcrumb-item breadcrumb-current">${crumb.label}</span>`;
            } else {
                return `${separator}<button class="breadcrumb-item breadcrumb-link" 
                        onclick="window.headerInstance?.navigateToBreadcrumb(${index})" 
                        tabindex="0"
                        aria-label="Navigate to ${crumb.label}">${crumb.label}</button>`;
            }
        });

        return `<nav class="breadcrumbs" aria-label="Breadcrumb navigation">${items.join('')}</nav>`;
    }

    setBreadcrumbs(breadcrumbs: BreadcrumbItem[]): void {
        this.breadcrumbs = breadcrumbs;
        this.render();
    }

    setDocumentName(name: string | null): void {
        this.documentName = name;
        if (name) {
            this.setBreadcrumbs([
                { label: 'Home', onClick: () => this.navigateToHome() },
                { label: name }
            ]);
        } else {
            this.setBreadcrumbs([]);
        }
    }

    navigateToBreadcrumb(index: number): void {
        const crumb = this.breadcrumbs[index];
        if (crumb && crumb.onClick) {
            crumb.onClick();
        }
    }

    navigateToHome(): void {
        // This will be handled by the main app
        window.dispatchEvent(new CustomEvent('navigate-home'));
    }

    setActions(actions: HTMLElement[]): void {
        const actionsContainer = this.container.querySelector('#headerActions');
        if (actionsContainer) {
            actionsContainer.innerHTML = '';
            actions.forEach(action => {
                actionsContainer.appendChild(action);
            });
        }
    }

    getElement(): HTMLElement {
        return this.container;
    }
}

// Make it globally accessible for onclick handlers
(window as any).headerInstance = null;
