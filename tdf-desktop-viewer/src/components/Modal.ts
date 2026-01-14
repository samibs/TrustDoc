// Modal Component - Enterprise-grade modal with focus trap and accessibility
// WCAG 2.1 AA compliant

import { keyboardShortcuts } from '../services/keyboard-shortcuts';
import { createInlineIcon } from './Icon';
import { Button } from './Button';

export interface ModalOptions {
    title: string;
    content: string | HTMLElement;
    footer?: string | HTMLElement;
    closeOnBackdropClick?: boolean;
    closeOnEscape?: boolean;
    showCloseButton?: boolean;
    ariaLabel?: string;
    ariaDescribedBy?: string;
    onClose?: () => void;
    size?: 'sm' | 'md' | 'lg' | 'xl';
}

export class Modal {
    private container: HTMLElement;
    private backdrop: HTMLElement;
    private dialog: HTMLElement;
    private options: Required<Omit<ModalOptions, 'onClose' | 'ariaLabel' | 'ariaDescribedBy'>> & Pick<ModalOptions, 'onClose' | 'ariaLabel' | 'ariaDescribedBy'>;
    private focusTrapCleanup: (() => void) | null = null;
    private previousActiveElement: HTMLElement | null = null;

    constructor(options: ModalOptions) {
        this.options = {
            title: options.title,
            content: options.content,
            footer: options.footer || undefined as any, // Will be checked before use
            closeOnBackdropClick: options.closeOnBackdropClick !== false,
            closeOnEscape: options.closeOnEscape !== false,
            showCloseButton: options.showCloseButton !== false,
            size: options.size || 'md',
            onClose: options.onClose,
            ariaLabel: options.ariaLabel,
            ariaDescribedBy: options.ariaDescribedBy
        };

        this.container = this.create();
        this.backdrop = this.createBackdrop();
        this.dialog = null as any; // Will be set in show()
        
        this.container.appendChild(this.backdrop);
    }

    private create(): HTMLElement {
        const container = document.createElement('div');
        container.className = 'modal-container';
        container.setAttribute('role', 'presentation');
        return container;
    }

    private createBackdrop(): HTMLElement {
        const backdrop = document.createElement('div');
        backdrop.className = 'modal-backdrop';
        backdrop.setAttribute('aria-hidden', 'true');
        
        if (this.options.closeOnBackdropClick) {
            backdrop.addEventListener('click', (e) => {
                if (e.target === backdrop) {
                    this.close();
                }
            });
        }
        
        return backdrop;
    }

    private async createDialog(): Promise<HTMLElement> {
        const dialog = document.createElement('div');
        dialog.className = `modal-dialog modal-${this.options.size}`;
        dialog.setAttribute('role', 'dialog');
        dialog.setAttribute('aria-modal', 'true');
        dialog.setAttribute('aria-labelledby', 'modal-title');
        
        if (this.options.ariaLabel) {
            dialog.setAttribute('aria-label', this.options.ariaLabel);
        }
        
        if (this.options.ariaDescribedBy) {
            dialog.setAttribute('aria-describedby', this.options.ariaDescribedBy);
        }

        // Header
        const header = document.createElement('div');
        header.className = 'modal-header';
        
        const title = document.createElement('h2');
        title.id = 'modal-title';
        title.className = 'modal-title';
        title.textContent = this.options.title;
        header.appendChild(title);

        if (this.options.showCloseButton) {
            const closeButton = document.createElement('button');
            closeButton.className = 'modal-close';
            closeButton.setAttribute('aria-label', 'Close dialog');
            closeButton.setAttribute('type', 'button');
            closeButton.setAttribute('tabindex', '0');
            
            const closeIcon = await createInlineIcon('x-mark', 20, '', 'Close');
            closeButton.appendChild(closeIcon);
            
            closeButton.addEventListener('click', () => this.close());
            header.appendChild(closeButton);
        }

        // Body
        const body = document.createElement('div');
        body.className = 'modal-body';
        body.id = this.options.ariaDescribedBy || 'modal-body';
        
        if (typeof this.options.content === 'string') {
            body.innerHTML = this.options.content;
        } else {
            body.appendChild(this.options.content);
        }

        // Footer
        let footer: HTMLElement | null = null;
        if (this.options.footer) {
            footer = document.createElement('div');
            footer.className = 'modal-footer';
            
            if (typeof this.options.footer === 'string') {
                footer.innerHTML = this.options.footer;
            } else {
                footer.appendChild(this.options.footer);
            }
        }

        dialog.appendChild(header);
        dialog.appendChild(body);
        if (footer) {
            dialog.appendChild(footer);
        }

        return dialog;
    }

    async show(): Promise<void> {
        // Create dialog if not already created
        if (!this.dialog) {
            this.dialog = await this.createDialog();
            this.container.appendChild(this.dialog);
        }

        // Store previous active element for restoration
        this.previousActiveElement = document.activeElement as HTMLElement;

        // Add to DOM
        document.body.appendChild(this.container);
        
        // Prevent body scroll
        document.body.style.overflow = 'hidden';

        // Set up focus trap
        this.focusTrapCleanup = keyboardShortcuts.trapFocus(this.dialog);

        // Register Escape key handler
        if (this.options.closeOnEscape) {
            keyboardShortcuts.register({
                key: 'Escape',
                handler: () => {
                    if (this.isOpen()) {
                        this.close();
                    }
                },
                description: 'Close modal',
                category: 'general'
            });
        }

        // Trigger animation
        requestAnimationFrame(() => {
            this.container.classList.add('modal-open');
        });
    }

    close(): void {
        if (!this.isOpen()) return;

        // Trigger close animation
        this.container.classList.remove('modal-open');
        this.container.classList.add('modal-closing');

        setTimeout(() => {
            // Cleanup
            if (this.focusTrapCleanup) {
                this.focusTrapCleanup();
                this.focusTrapCleanup = null;
            }

            // Remove from DOM
            if (this.container.parentNode) {
                this.container.parentNode.removeChild(this.container);
            }

            // Restore body scroll
            document.body.style.overflow = '';

            // Restore focus
            if (this.previousActiveElement) {
                this.previousActiveElement.focus();
            }

            // Call onClose callback
            if (this.options.onClose) {
                this.options.onClose();
            }
        }, 250); // Match CSS transition duration
    }

    isOpen(): boolean {
        return this.container.parentNode !== null && 
               this.container.classList.contains('modal-open');
    }

    getElement(): HTMLElement {
        return this.dialog;
    }

    // Static factory method
    static async create(options: ModalOptions): Promise<Modal> {
        const modal = new Modal(options);
        await modal.show();
        return modal;
    }
}

// Helper function for simple modals
export async function showModal(
    title: string,
    content: string | HTMLElement,
    options?: Partial<ModalOptions>
): Promise<Modal> {
    return Modal.create({
        title,
        content,
        ...options
    });
}

// Helper function for confirmation dialogs
export async function showConfirmModal(
    title: string,
    message: string,
    onConfirm: () => void,
    onCancel?: () => void
): Promise<Modal> {
    const confirmButton = await Button.create('Confirm', {
        variant: 'primary',
        onClick: () => {
            modal.close();
            onConfirm();
        }
    });

    const cancelButton = await Button.create('Cancel', {
        variant: 'secondary',
        onClick: () => {
            modal.close();
            if (onCancel) onCancel();
        }
    });

    const footer = document.createElement('div');
    footer.className = 'modal-footer-actions';
    footer.appendChild(cancelButton.getElement());
    footer.appendChild(confirmButton.getElement());

    const modal = new Modal({
        title,
        content: `<p>${message}</p>`,
        footer,
        closeOnBackdropClick: false
    });

    await modal.show();
    return modal;
}
