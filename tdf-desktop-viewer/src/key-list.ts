// Key List Component

import { KeyInfo } from './keys';

export function renderKeyList(
    keys: KeyInfo[],
    onSelect: (keyId: string) => void,
    onDelete: (keyId: string) => void,
    onExport: (keyId: string) => void
) {
    const container = document.getElementById('key-list-container');
    if (!container) return;

    if (keys.length === 0) {
        container.innerHTML = `
            <div class="empty-state">
                <p>🔑 No keys found</p>
                <p class="hint">Generate a new key or import an existing one to get started</p>
            </div>
        `;
        return;
    }

    const table = document.createElement('div');
    table.className = 'table-container';
    table.innerHTML = `
        <table>
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Algorithm</th>
                    <th>Created</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                ${keys.map(key => `
                    <tr class="key-row" data-key-id="${key.id}">
                        <td>
                            <div class="key-name">${escapeHtml(key.name)}</div>
                            ${key.signer_id ? `<div class="key-meta">${escapeHtml(key.signer_id)}</div>` : ''}
                        </td>
                        <td>
                            <span class="status-indicator info">${escapeHtml(key.algorithm.toUpperCase())}</span>
                        </td>
                        <td>${formatDate(key.created)}</td>
                        <td>
                            <div class="key-actions">
                                <button class="btn btn-sm btn-primary" onclick="selectKey('${key.id}')" title="View Details">View</button>
                                <button class="btn btn-sm btn-secondary" onclick="exportKey('${key.id}')" title="Export Key">Export</button>
                                <button class="btn btn-sm btn-danger" onclick="deleteKey('${key.id}')" title="Delete Key">Delete</button>
                            </div>
                        </td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    `;

    container.innerHTML = '';
    container.appendChild(table);

    // Make functions available globally for onclick handlers
    (window as any).selectKey = (keyId: string) => {
        // Remove previous selection
        document.querySelectorAll('.key-row').forEach(row => {
            row.classList.remove('selected');
        });
        // Add selection to clicked row
        const row = document.querySelector(`[data-key-id="${keyId}"]`);
        if (row) {
            row.classList.add('selected');
        }
        onSelect(keyId);
    };

    (window as any).exportKey = (keyId: string) => {
        onExport(keyId);
    };

    (window as any).deleteKey = (keyId: string) => {
        onDelete(keyId);
    };
}

function escapeHtml(text: string): string {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function formatDate(dateString: string): string {
    try {
        const date = new Date(dateString);
        return date.toLocaleDateString() + ' ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    } catch {
        return dateString;
    }
}
