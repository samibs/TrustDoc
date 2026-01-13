// Keyboard Shortcuts Service
// Provides centralized keyboard navigation and shortcuts

export interface KeyboardShortcut {
    key: string;
    ctrl?: boolean;
    shift?: boolean;
    alt?: boolean;
    meta?: boolean;
    handler: (e: KeyboardEvent) => void;
    description: string;
    category: 'navigation' | 'actions' | 'general';
}

class KeyboardShortcutsService {
    private shortcuts: Map<string, KeyboardShortcut> = new Map();
    private focusableSelectors = 'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])';

    constructor() {
        document.addEventListener('keydown', this.handleKeyDown.bind(this));
    }

    register(shortcut: KeyboardShortcut): void {
        const key = this.getShortcutKey(shortcut);
        this.shortcuts.set(key, shortcut);
    }

    unregister(key: string, modifiers?: { ctrl?: boolean; shift?: boolean; alt?: boolean; meta?: boolean }): void {
        const shortcutKey = this.getShortcutKeyFromString(key, modifiers);
        this.shortcuts.delete(shortcutKey);
    }

    private getShortcutKey(shortcut: KeyboardShortcut): string {
        const parts: string[] = [];
        if (shortcut.ctrl) parts.push('ctrl');
        if (shortcut.shift) parts.push('shift');
        if (shortcut.alt) parts.push('alt');
        if (shortcut.meta) parts.push('meta');
        parts.push(shortcut.key.toLowerCase());
        return parts.join('+');
    }

    private getShortcutKeyFromString(key: string, modifiers?: { ctrl?: boolean; shift?: boolean; alt?: boolean; meta?: boolean }): string {
        const parts: string[] = [];
        if (modifiers?.ctrl) parts.push('ctrl');
        if (modifiers?.shift) parts.push('shift');
        if (modifiers?.alt) parts.push('alt');
        if (modifiers?.meta) parts.push('meta');
        parts.push(key.toLowerCase());
        return parts.join('+');
    }

    private handleKeyDown(e: KeyboardEvent): void {
        // Don't intercept if user is typing in an input
        if (e.target instanceof HTMLInputElement || 
            e.target instanceof HTMLTextAreaElement ||
            (e.target instanceof HTMLElement && e.target.isContentEditable)) {
            return;
        }

        const key = this.getShortcutKey({
            key: e.key,
            ctrl: e.ctrlKey,
            shift: e.shiftKey,
            alt: e.altKey,
            meta: e.metaKey,
            handler: () => {},
            description: '',
            category: 'general'
        });

        const shortcut = this.shortcuts.get(key);
        if (shortcut) {
            e.preventDefault();
            e.stopPropagation();
            shortcut.handler(e);
        }
    }

    // Focus management
    getFocusableElements(container: HTMLElement = document.body): HTMLElement[] {
        const elements = Array.from(container.querySelectorAll<HTMLElement>(this.focusableSelectors));
        return elements.filter(el => {
            // Filter out hidden and disabled elements
            return !el.hasAttribute('disabled') &&
                   !el.hasAttribute('hidden') &&
                   el.offsetParent !== null &&
                   (el.tabIndex >= 0 || el.tabIndex === undefined);
        });
    }

    trapFocus(container: HTMLElement): () => void {
        const focusableElements = this.getFocusableElements(container);
        if (focusableElements.length === 0) return () => {};

        const firstElement = focusableElements[0];
        const lastElement = focusableElements[focusableElements.length - 1];

        const handleTab = (e: KeyboardEvent) => {
            if (e.key !== 'Tab') return;

            if (e.shiftKey) {
                // Shift + Tab
                if (document.activeElement === firstElement) {
                    e.preventDefault();
                    lastElement.focus();
                }
            } else {
                // Tab
                if (document.activeElement === lastElement) {
                    e.preventDefault();
                    firstElement.focus();
                }
            }
        };

        container.addEventListener('keydown', handleTab);
        firstElement.focus();

        // Return cleanup function
        return () => {
            container.removeEventListener('keydown', handleTab);
        };
    }

    // Get all registered shortcuts for help display
    getAllShortcuts(): KeyboardShortcut[] {
        return Array.from(this.shortcuts.values());
    }

    getShortcutsByCategory(category: 'navigation' | 'actions' | 'general'): KeyboardShortcut[] {
        return this.getAllShortcuts().filter(s => s.category === category);
    }
}

// Export singleton instance
export const keyboardShortcuts = new KeyboardShortcutsService();

// Helper function to format shortcut for display
export function formatShortcut(shortcut: KeyboardShortcut): string {
    const parts: string[] = [];
    if (shortcut.ctrl) parts.push('Ctrl');
    if (shortcut.shift) parts.push('Shift');
    if (shortcut.alt) parts.push('Alt');
    if (shortcut.meta) parts.push('Cmd');
    
    let key = shortcut.key;
    if (key === ' ') key = 'Space';
    if (key.length === 1) key = key.toUpperCase();
    
    parts.push(key);
    return parts.join(' + ');
}
