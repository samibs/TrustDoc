// Sidebar Component - Persistent left sidebar with navigation and document info

import { createInlineIcon } from '../components/Icon';

export interface SidebarItem {
    id: string;
    label: string;
    icon?: string;
    onClick: () => void;
    active?: boolean;
}

export class Sidebar {
    private container: HTMLElement;
    private isCollapsed: boolean = false;
    private items: SidebarItem[] = [];

    constructor(containerId: string = 'sidebar') {
        const existing = document.getElementById(containerId);
        if (existing) {
            this.container = existing;
        } else {
            this.container = document.createElement('aside');
            this.container.id = containerId;
            this.container.className = 'sidebar';
            this.container.setAttribute('role', 'complementary');
            this.container.setAttribute('aria-label', 'Navigation sidebar');
        }
    }

    async render(): Promise<void> {
        const collapseButton = await createInlineIcon('chevron-down', 20, 'sidebar-collapse-icon', 'Collapse sidebar');
        
        this.container.innerHTML = `
            <div class="sidebar-header">
                <h2 class="sidebar-title">TDF Viewer</h2>
                <button class="sidebar-collapse" id="sidebarCollapse" aria-label="Collapse sidebar" tabindex="0">
                    ${collapseButton.outerHTML}
                </button>
            </div>
            <nav class="sidebar-nav" id="sidebarNav" role="navigation" aria-label="Main navigation">
                ${this.items.map(item => this.renderItem(item)).join('')}
            </nav>
            <div class="sidebar-footer" id="sidebarFooter"></div>
        `;

        // Load saved state
        const savedState = localStorage.getItem('sidebar-collapsed');
        if (savedState === 'true') {
            this.collapse();
        }

        // Add collapse handler
        const collapseBtn = this.container.querySelector('#sidebarCollapse');
        if (collapseBtn) {
            collapseBtn.addEventListener('click', () => {
                this.toggle();
            });
        }
    }

    private renderItem(item: SidebarItem): string {
        const iconHtml = item.icon ? `<span class="sidebar-item-icon" data-icon="${item.icon}"></span>` : '';
        const activeClass = item.active ? 'sidebar-item-active' : '';
        
        return `
            <div class="sidebar-item ${activeClass}" 
                 data-item-id="${item.id}" 
                 role="button"
                 tabindex="0"
                 aria-label="${item.label}">
                ${iconHtml}
                <span class="sidebar-item-label">${item.label}</span>
            </div>
        `;
    }

    addItem(item: SidebarItem): void {
        this.items.push(item);
    }

    setItems(items: SidebarItem[]): void {
        this.items = items;
    }

    setActiveItem(itemId: string): void {
        this.items.forEach(item => {
            item.active = item.id === itemId;
        });
        this.render();
    }

    async toggle(): Promise<void> {
        this.isCollapsed = !this.isCollapsed;
        this.container.classList.toggle('sidebar-collapsed', this.isCollapsed);
        localStorage.setItem('sidebar-collapsed', String(this.isCollapsed));
        
        // Update icon
        const icon = this.container.querySelector('.sidebar-collapse-icon');
        if (icon) {
            const newIcon = await createInlineIcon(
                this.isCollapsed ? 'chevron-up' : 'chevron-down',
                20,
                'sidebar-collapse-icon',
                this.isCollapsed ? 'Expand sidebar' : 'Collapse sidebar'
            );
            icon.replaceWith(newIcon);
        }
    }

    collapse(): void {
        if (!this.isCollapsed) {
            this.toggle();
        }
    }

    expand(): void {
        if (this.isCollapsed) {
            this.toggle();
        }
    }

    setFooter(content: HTMLElement | string): void {
        const footer = this.container.querySelector('#sidebarFooter');
        if (footer) {
            if (typeof content === 'string') {
                footer.innerHTML = content;
            } else {
                footer.innerHTML = '';
                footer.appendChild(content);
            }
        }
    }

    getElement(): HTMLElement {
        return this.container;
    }
}
