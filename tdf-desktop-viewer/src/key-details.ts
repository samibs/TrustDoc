// Key Details Panel Component

import { KeyInfo } from './keys';

export function renderKeyDetails(key: KeyInfo) {
    const container = document.getElementById('key-details-container');
    if (!container) return;

    container.innerHTML = `
        <div class="card">
            <div class="card-header">
                <h3 class="card-title">Key Details</h3>
            </div>
            <div class="key-details-content">
                <div class="result-item">
                    <strong>Name:</strong>
                    <span>${escapeHtml(key.name)}</span>
                </div>
                <div class="result-item">
                    <strong>Algorithm:</strong>
                    <span class="status-indicator info">${escapeHtml(key.algorithm.toUpperCase())}</span>
                </div>
                <div class="result-item">
                    <strong>Created:</strong>
                    <span>${formatDate(key.created)}</span>
                </div>
                ${key.signer_id ? `
                    <div class="result-item">
                        <strong>Signer ID:</strong>
                        <code>${escapeHtml(key.signer_id)}</code>
                    </div>
                ` : ''}
                ${key.fingerprint ? `
                    <div class="result-item">
                        <strong>Fingerprint:</strong>
                        <code>${escapeHtml(key.fingerprint)}</code>
                    </div>
                ` : ''}
                <div class="result-item">
                    <strong>Key ID:</strong>
                    <code>${escapeHtml(key.id)}</code>
                </div>
            </div>
        </div>
    `;
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
