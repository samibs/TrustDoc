// Status Bar Component - Persistent status display
// Replaces auto-dismissing status messages with enterprise-grade persistent status bar

import { createInlineIcon } from './Icon';

export type StatusType = 'info' | 'success' | 'error' | 'warning' | 'ready';

interface StatusMessage {
    id: string;
    message: string;
    type: StatusType;
    timestamp: Date;
    dismissible: boolean;
}

class StatusBar {
    private container: HTMLElement | null = null;
    private messages: Map<string, StatusMessage> = new Map();
    private currentDocument: { name: string; integrity: string; signatures: number } | null = null;
    private autoDismissTimers: Map<string, NodeJS.Timeout> = new Map();

    constructor(containerId: string = 'statusBar') {
        this.container = document.getElementById(containerId);
        if (!this.container) {
            console.error('Status bar container not found');
        }
    }

    show(message: string, type: StatusType = 'info', autoDismiss: boolean = false, dismissible: boolean = true): string {
        const id = `status-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
        const statusMessage: StatusMessage = {
            id,
            message,
            type,
            timestamp: new Date(),
            dismissible
        };

        this.messages.set(id, statusMessage);
        this.render();

        // Auto-dismiss for success messages only (10 seconds)
        if (autoDismiss && type === 'success') {
            const timer = setTimeout(() => {
                this.dismiss(id);
            }, 10000);
            this.autoDismissTimers.set(id, timer);
        }

        return id;
    }

    dismiss(id: string): void {
        this.messages.delete(id);
        const timer = this.autoDismissTimers.get(id);
        if (timer) {
            clearTimeout(timer);
            this.autoDismissTimers.delete(id);
        }
        this.render();
    }

    dismissAll(): void {
        this.messages.clear();
        this.autoDismissTimers.forEach(timer => clearTimeout(timer));
        this.autoDismissTimers.clear();
        this.render();
    }

    setDocumentInfo(name: string, integrity: 'Valid' | 'Invalid' | null, signatures: number | null): void {
        this.currentDocument = {
            name,
            integrity: integrity || 'Unknown',
            signatures: signatures || 0
        };
        this.render();
    }

    clearDocumentInfo(): void {
        this.currentDocument = null;
        this.render();
    }

    private async render(): Promise<void> {
        if (!this.container) return;

        const parts: string[] = [];

        // Status messages
        if (this.messages.size > 0) {
            const messageArray = Array.from(this.messages.values());
            const latestMessage = messageArray[messageArray.length - 1];
            parts.push(`<span class="status-message status-${latestMessage.type}">${this.escapeHtml(latestMessage.message)}</span>`);
            
            // Dismiss button for latest message
            if (latestMessage.dismissible) {
                const dismissIcon = await createInlineIcon('x-mark', 16, 'status-dismiss-icon', 'Dismiss');
                parts.push(`<button class="status-dismiss" onclick="window.statusBarInstance.dismiss('${latestMessage.id}')" aria-label="Dismiss message">${dismissIcon.outerHTML}</button>`);
            }
        } else {
            parts.push('<span class="status-message status-ready">Ready</span>');
        }

        // Document info (when document is loaded)
        if (this.currentDocument) {
            parts.push('|');
            parts.push(`<span class="status-doc-info">Document: <strong>${this.escapeHtml(this.currentDocument.name)}</strong></span>`);
            if (this.currentDocument.integrity !== 'Unknown') {
                const integrityClass = this.currentDocument.integrity === 'Valid' ? 'status-success' : 'status-error';
                parts.push(`<span class="status-integrity ${integrityClass}">Integrity: ${this.currentDocument.integrity}</span>`);
            }
            if (this.currentDocument.signatures > 0) {
                parts.push(`<span class="status-signatures">Signatures: ${this.currentDocument.signatures}</span>`);
            }
        }

        this.container.innerHTML = parts.join(' ');
    }

    private escapeHtml(text: string): string {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
}

// Create singleton instance
export const statusBar = new StatusBar('statusBar');

// Make it globally accessible for onclick handlers
(window as any).statusBarInstance = statusBar;

// Export for use in other modules
export default statusBar;
