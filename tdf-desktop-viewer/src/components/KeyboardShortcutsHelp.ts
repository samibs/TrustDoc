// Keyboard Shortcuts Help Modal
// Displays all available keyboard shortcuts in an accessible modal

import { Modal } from './Modal';
import { keyboardShortcuts, formatShortcut } from '../services/keyboard-shortcuts';

export async function showKeyboardShortcutsHelp(): Promise<void> {
    const shortcuts = keyboardShortcuts.getAllShortcuts();
    
    const shortcutsByCategory = {
        navigation: shortcuts.filter(s => s.category === 'navigation'),
        actions: shortcuts.filter(s => s.category === 'actions'),
        general: shortcuts.filter(s => s.category === 'general')
    };

    const content = document.createElement('div');
    content.className = 'keyboard-shortcuts-help';

    // Navigation shortcuts
    if (shortcutsByCategory.navigation.length > 0) {
        const navSection = document.createElement('div');
        navSection.className = 'shortcuts-section';
        navSection.innerHTML = `
            <h3>Navigation</h3>
            <table class="shortcuts-table">
                <thead>
                    <tr>
                        <th>Shortcut</th>
                        <th>Action</th>
                    </tr>
                </thead>
                <tbody>
                    ${shortcutsByCategory.navigation.map(s => `
                        <tr>
                            <td><kbd>${formatShortcut(s)}</kbd></td>
                            <td>${s.description}</td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
        content.appendChild(navSection);
    }

    // Action shortcuts
    if (shortcutsByCategory.actions.length > 0) {
        const actionsSection = document.createElement('div');
        actionsSection.className = 'shortcuts-section';
        actionsSection.innerHTML = `
            <h3>Actions</h3>
            <table class="shortcuts-table">
                <thead>
                    <tr>
                        <th>Shortcut</th>
                        <th>Action</th>
                    </tr>
                </thead>
                <tbody>
                    ${shortcutsByCategory.actions.map(s => `
                        <tr>
                            <td><kbd>${formatShortcut(s)}</kbd></td>
                            <td>${s.description}</td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
        content.appendChild(actionsSection);
    }

    // General shortcuts
    if (shortcutsByCategory.general.length > 0) {
        const generalSection = document.createElement('div');
        generalSection.className = 'shortcuts-section';
        generalSection.innerHTML = `
            <h3>General</h3>
            <table class="shortcuts-table">
                <thead>
                    <tr>
                        <th>Shortcut</th>
                        <th>Action</th>
                    </tr>
                </thead>
                <tbody>
                    ${shortcutsByCategory.general.map(s => `
                        <tr>
                            <td><kbd>${formatShortcut(s)}</kbd></td>
                            <td>${s.description}</td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
        content.appendChild(generalSection);
    }

    // Add help text
    const helpText = document.createElement('div');
    helpText.className = 'shortcuts-help-text';
    helpText.innerHTML = `
        <p><strong>Tip:</strong> You can use Tab to navigate between interactive elements, and Enter or Space to activate buttons.</p>
    `;
    content.appendChild(helpText);

    await Modal.create({
        title: 'Keyboard Shortcuts',
        content,
        size: 'lg',
        ariaLabel: 'Keyboard shortcuts help'
    });
}
