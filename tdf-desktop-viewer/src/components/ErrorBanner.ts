// Error Banner Component - Enterprise error handling
// Provides specific error messages, suggestions, and error codes

import { createInlineIcon } from './Icon';
import { statusBar } from './StatusBar';

export interface ErrorDetails {
    code: string;
    message: string;
    suggestion?: string;
    retryable?: boolean;
    onRetry?: () => void;
    technicalDetails?: string;
}

export class ErrorBanner {
    private container: HTMLElement | null = null;
    private currentError: ErrorDetails | null = null;

    constructor(containerId: string = 'errorBanner') {
        this.container = document.getElementById(containerId);
        if (!this.container) {
            // Create container if it doesn't exist
            this.container = document.createElement('div');
            this.container.id = containerId;
            this.container.className = 'error-banner-container';
            document.body.insertBefore(this.container, document.body.firstChild);
        }
    }

    async show(error: ErrorDetails): Promise<void> {
        this.currentError = error;
        
        if (!this.container) return;

        const banner = document.createElement('div');
        banner.className = 'error-banner';
        banner.setAttribute('role', 'alert');
        banner.setAttribute('aria-live', 'assertive');

        // Error icon
        const icon = await createInlineIcon('x-circle', 24, 'error-banner-icon', 'Error');
        banner.appendChild(icon);

        // Error content
        const content = document.createElement('div');
        content.className = 'error-banner-content';

        // Error message
        const message = document.createElement('div');
        message.className = 'error-banner-message';
        message.textContent = error.message;
        content.appendChild(message);

        // Error code
        const code = document.createElement('div');
        code.className = 'error-banner-code';
        code.textContent = `Error Code: ${error.code}`;
        content.appendChild(code);

        // Suggestion
        if (error.suggestion) {
            const suggestion = document.createElement('div');
            suggestion.className = 'error-banner-suggestion';
            suggestion.textContent = `💡 ${error.suggestion}`;
            content.appendChild(suggestion);
        }

        // Technical details (collapsible)
        if (error.technicalDetails) {
            const detailsToggle = document.createElement('button');
            detailsToggle.className = 'error-banner-details-toggle';
            detailsToggle.textContent = 'Show technical details';
            detailsToggle.setAttribute('aria-expanded', 'false');
            detailsToggle.setAttribute('tabindex', '0');

            const details = document.createElement('div');
            details.className = 'error-banner-details';
            details.style.display = 'none';
            details.textContent = error.technicalDetails;

            detailsToggle.addEventListener('click', () => {
                const isExpanded = detailsToggle.getAttribute('aria-expanded') === 'true';
                details.style.display = isExpanded ? 'none' : 'block';
                detailsToggle.setAttribute('aria-expanded', String(!isExpanded));
                detailsToggle.textContent = isExpanded ? 'Show technical details' : 'Hide technical details';
            });

            content.appendChild(detailsToggle);
            content.appendChild(details);
        }

        banner.appendChild(content);

        // Actions
        const actions = document.createElement('div');
        actions.className = 'error-banner-actions';

        // Retry button
        if (error.retryable && error.onRetry) {
            const retryButton = document.createElement('button');
            retryButton.className = 'btn btn-secondary btn-sm';
            retryButton.textContent = 'Retry';
            retryButton.setAttribute('tabindex', '0');
            retryButton.addEventListener('click', () => {
                this.dismiss();
                error.onRetry!();
            });
            actions.appendChild(retryButton);
        }

        // Dismiss button
        const dismissButton = document.createElement('button');
        dismissButton.className = 'error-banner-dismiss';
        dismissButton.setAttribute('aria-label', 'Dismiss error');
        dismissButton.setAttribute('tabindex', '0');
        
        const dismissIcon = await createInlineIcon('x-mark', 20, '', 'Dismiss');
        dismissButton.appendChild(dismissIcon);
        
        dismissButton.addEventListener('click', () => {
            this.dismiss();
        });

        actions.appendChild(dismissButton);
        banner.appendChild(actions);

        // Clear existing errors
        this.container.innerHTML = '';
        this.container.appendChild(banner);

        // Also show in status bar
        statusBar.show(error.message, 'error', false, true);

        // Log to console
        console.error(`[${error.code}] ${error.message}`, error.technicalDetails || '');
    }

    dismiss(): void {
        if (this.container) {
            this.container.innerHTML = '';
        }
        this.currentError = null;
    }

    getCurrentError(): ErrorDetails | null {
        return this.currentError;
    }
}

// Error code definitions
export const ErrorCodes = {
    FILE_LOAD_001: 'FILE_LOAD_001',
    FILE_LOAD_002: 'FILE_LOAD_002',
    FILE_LOAD_003: 'FILE_LOAD_003',
    VERIFY_001: 'VERIFY_001',
    EXTRACT_001: 'EXTRACT_001',
    SIGNATURE_001: 'SIGNATURE_001',
    UNKNOWN: 'UNKNOWN'
} as const;

// Error factory functions
export function createFileLoadError(error: Error, fileName: string): ErrorDetails {
    if (error.message.includes('not a valid TDF')) {
        return {
            code: ErrorCodes.FILE_LOAD_001,
            message: `Invalid TDF file format: ${fileName}`,
            suggestion: 'Please ensure the file is a valid .tdf document created by a TrustDoc-compatible application.',
            retryable: false,
            technicalDetails: error.message
        };
    }

    if (error.message.includes('corrupted') || error.message.includes('damaged')) {
        return {
            code: ErrorCodes.FILE_LOAD_002,
            message: `File appears to be corrupted: ${fileName}`,
            suggestion: 'The file may have been damaged during transfer. Try downloading or copying the file again.',
            retryable: true,
            technicalDetails: error.message
        };
    }

    if (error.message.includes('permission') || error.message.includes('access')) {
        return {
            code: ErrorCodes.FILE_LOAD_003,
            message: `Cannot access file: ${fileName}`,
            suggestion: 'Please check file permissions and ensure the file is not locked by another application.',
            retryable: true,
            technicalDetails: error.message
        };
    }

    return {
        code: ErrorCodes.FILE_LOAD_001,
        message: `Failed to load file: ${fileName}`,
        suggestion: 'Please try opening the file again. If the problem persists, contact support with the error code.',
        retryable: true,
        technicalDetails: error.message
    };
}

export function createVerificationError(error: Error): ErrorDetails {
    return {
        code: ErrorCodes.VERIFY_001,
        message: 'Document verification failed',
        suggestion: 'The document may have been tampered with or corrupted. Please verify the source of the document.',
        retryable: true,
        technicalDetails: error.message
    };
}

export function createExtractionError(error: Error): ErrorDetails {
    return {
        code: ErrorCodes.EXTRACT_001,
        message: 'Data extraction failed',
        suggestion: 'The document structure may be invalid. Please try verifying the document first.',
        retryable: true,
        technicalDetails: error.message
    };
}

// Singleton instance
export const errorBanner = new ErrorBanner('errorBanner');
