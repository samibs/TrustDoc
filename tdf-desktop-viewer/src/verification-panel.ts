// Enhanced Verification Panel Component

export interface VerificationResult {
    integrity_valid: boolean;
    root_hash: string;
    signature_count: number;
    signatures: Array<{
        signer_id: string;
        signer_name: string;
        algorithm: string;
        timestamp: string;
        valid: boolean;
    }>;
    timestamp_warnings: string[];
}

export function renderVerificationPanel(result: VerificationResult, container: HTMLElement) {
    const integrityStatus = result.integrity_valid ? 'valid' : 'invalid';
    const integrityIcon = result.integrity_valid ? '✓' : '✗';
    const integrityText = result.integrity_valid ? 'VALID' : 'INVALID';

    container.innerHTML = `
        <div class="verification-result">
            <h3>🔍 Verification Results</h3>
            
            <div class="result-item">
                <strong>Document Integrity:</strong>
                <span class="status-indicator ${integrityStatus}">
                    ${integrityIcon} ${integrityText}
                </span>
            </div>
            
            <div class="result-item">
                <strong>Root Hash:</strong>
                <code>${escapeHtml(result.root_hash)}</code>
            </div>
            
            <div class="result-item">
                <strong>Signatures:</strong>
                <span>${result.signature_count}</span>
            </div>
            
            ${result.signatures.length > 0 ? `
                <div class="signature-list">
                    <h4>Signature Details</h4>
                    ${result.signatures.map((sig, index) => `
                        <div class="signature-item">
                            <div class="signature-header">
                                <span class="status-indicator ${sig.valid ? 'valid' : 'invalid'}">
                                    ${sig.valid ? '✓' : '✗'} Signature ${index + 1}
                                </span>
                            </div>
                            <div class="signature-details">
                                <div><strong>Signer:</strong> ${escapeHtml(sig.signer_name || sig.signer_id)}</div>
                                <div><strong>Signer ID:</strong> <code>${escapeHtml(sig.signer_id)}</code></div>
                                <div><strong>Algorithm:</strong> ${escapeHtml(sig.algorithm.toUpperCase())}</div>
                                <div><strong>Timestamp:</strong> ${formatDate(sig.timestamp)}</div>
                            </div>
                        </div>
                    `).join('')}
                </div>
            ` : `
                <div class="result-item warning">
                    <strong>⚠️ No signatures found</strong>
                    <p>This document has not been signed.</p>
                </div>
            `}
            
            ${result.timestamp_warnings.length > 0 ? `
                <div class="result-item warning">
                    <strong>⚠️ Timestamp Warnings:</strong>
                    <ul>
                        ${result.timestamp_warnings.map(w => `<li>${escapeHtml(w)}</li>`).join('')}
                    </ul>
                </div>
            ` : ''}
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
        return date.toLocaleDateString() + ' ' + date.toLocaleTimeString([], { 
            hour: '2-digit', 
            minute: '2-digit',
            second: '2-digit'
        });
    } catch {
        return dateString;
    }
}
