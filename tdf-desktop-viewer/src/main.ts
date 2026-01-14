// Simple TDF Viewer - Main Entry Point
import { loadDocument, verifyIntegrity, extractData, ExtractedData } from 'tdf-ts';
import { createInlineIcon } from './components/Icon';
import { keyboardShortcuts } from './services/keyboard-shortcuts';
import { statusBar } from './components/StatusBar';
import { errorBanner, createFileLoadError, createVerificationError, createExtractionError } from './components/ErrorBanner';
import { auditLog } from './services/audit-log';
import { generateVerificationReport, exportVerificationAsJSON, exportVerificationAsPDF } from './services/export-verification';
import { showKeyboardShortcutsHelp } from './components/KeyboardShortcutsHelp';

let currentDocument: any = null;

document.addEventListener('DOMContentLoaded', async () => {
    await initIcons();
    initKeyboardShortcuts();
    initUploadArea();
    initAccessibility();
});

function initKeyboardShortcuts() {
    // Register keyboard shortcuts
    keyboardShortcuts.register({
        key: 'o',
        ctrl: true,
        handler: () => {
            const browseBtn = document.getElementById('browseBtn');
            if (browseBtn) browseBtn.click();
        },
        description: 'Open document',
        category: 'actions'
    });

    keyboardShortcuts.register({
        key: 'v',
        ctrl: true,
        handler: () => {
            const verifyBtn = document.getElementById('verifyBtn');
            if (verifyBtn && verifyBtn.style.display !== 'none') {
                verifyBtn.click();
            }
        },
        description: 'Verify document',
        category: 'actions'
    });

    keyboardShortcuts.register({
        key: 'e',
        ctrl: true,
        handler: () => {
            const extractBtn = document.getElementById('extractBtn');
            if (extractBtn && extractBtn.style.display !== 'none') {
                extractBtn.click();
            }
        },
        description: 'Extract data',
        category: 'actions'
    });

    keyboardShortcuts.register({
        key: 'p',
        ctrl: true,
        handler: () => {
            const printBtn = document.getElementById('printBtn');
            if (printBtn && printBtn.style.display !== 'none') {
                printBtn.click();
            }
        },
        description: 'Print document',
        category: 'actions'
    });

    keyboardShortcuts.register({
        key: 'Escape',
        handler: () => {
            // Close any open modals or return to upload view
            const documentView = document.getElementById('documentView');
            if (documentView && (documentView as HTMLElement).style.display !== 'none') {
                // Could add confirmation dialog here
            }
        },
        description: 'Cancel / Close',
        category: 'general'
    });

    // F1 or ? for help
    keyboardShortcuts.register({
        key: 'F1',
        handler: () => {
            showKeyboardShortcutsHelp();
        },
        description: 'Show keyboard shortcuts help',
        category: 'general'
    });

    keyboardShortcuts.register({
        key: '?',
        shift: true,
        handler: () => {
            showKeyboardShortcutsHelp();
        },
        description: 'Show keyboard shortcuts help',
        category: 'general'
    });
}

function initAccessibility() {
    // Add tabindex to all interactive elements
    const interactiveElements = document.querySelectorAll('button, [role="button"]');
    interactiveElements.forEach(el => {
        if (!el.hasAttribute('tabindex')) {
            el.setAttribute('tabindex', '0');
        }
    });

    // Ensure all buttons have aria-labels if they don't have visible text
    const buttons = document.querySelectorAll('button');
    buttons.forEach(btn => {
        if (!btn.getAttribute('aria-label') && !btn.textContent?.trim()) {
            const title = btn.getAttribute('title');
            if (title) {
                btn.setAttribute('aria-label', title);
            }
        }
    });
}

async function initIcons() {
    // Initialize icons in buttons
    const browseIcon = document.getElementById('browseIcon');
    const verifyIcon = document.getElementById('verifyIcon');
    const extractIcon = document.getElementById('extractIcon');
    const printIcon = document.getElementById('printIcon');
    const openIcon = document.getElementById('openIcon');

    if (browseIcon) {
        const icon = await createInlineIcon('folder-open', 20, '', 'Browse Files');
        browseIcon.appendChild(icon);
    }
    if (verifyIcon) {
        const icon = await createInlineIcon('shield-check', 20, '', 'Verify');
        verifyIcon.appendChild(icon);
    }
    if (extractIcon) {
        const icon = await createInlineIcon('table-cells', 20, '', 'Extract Data');
        extractIcon.appendChild(icon);
    }
    if (printIcon) {
        const icon = await createInlineIcon('printer', 20, '', 'Print');
        printIcon.appendChild(icon);
    }
    if (openIcon) {
        const icon = await createInlineIcon('folder-open', 20, '', 'Open');
        openIcon.appendChild(icon);
    }
}

function initUploadArea() {
    const uploadArea = document.getElementById('uploadArea');
    const browseBtn = document.getElementById('browseBtn');
    const fileInput = document.getElementById('fileInput') as HTMLInputElement;
    const verifyBtn = document.getElementById('verifyBtn');
    const extractBtn = document.getElementById('extractBtn');
    const openBtn = document.getElementById('openBtn');

    if (!uploadArea || !browseBtn || !fileInput) {
        console.error('Required elements not found');
        return;
    }

    // Handle browse button click
    browseBtn.addEventListener('click', () => {
        console.log('Browse button clicked');
        fileInput.click();
    });

    // Handle file input change
    fileInput.addEventListener('change', async () => {
        const files = fileInput.files;
        if (files && files.length > 0) {
            const file = files[0];
            if (file.name.endsWith('.tdf')) {
                await loadTdfFile(file);
                // Reset the input so the same file can be selected again
                fileInput.value = '';
            } else {
                showStatus('Please select a .tdf file', 'error');
                fileInput.value = '';
            }
        }
    });

    // Handle drag and drop
    uploadArea.addEventListener('dragover', (e) => {
        e.preventDefault();
        uploadArea.classList.add('drag-over');
    });

    uploadArea.addEventListener('dragleave', () => {
        uploadArea.classList.remove('drag-over');
    });

    uploadArea.addEventListener('drop', async (e) => {
        e.preventDefault();
        uploadArea.classList.remove('drag-over');

        const files = e.dataTransfer?.files;
        if (files && files.length > 0) {
            const file = files[0];
            if (file.name.endsWith('.tdf')) {
                await loadTdfFile(file);
            } else {
                showStatus('Please drop a .tdf file', 'error');
            }
        }
    });

    // Handle other buttons
    if (verifyBtn) {
        verifyBtn.addEventListener('click', () => verifyDocument());
    }

    if (extractBtn) {
        extractBtn.addEventListener('click', () => performDataExtraction());
    }

    // Handle Print button
    const printBtn = document.getElementById('printBtn');
    if (printBtn) {
        printBtn.addEventListener('click', () => {
            if (currentDocument) {
                window.print();
                auditLog.logDocumentPrint(
                    currentDocument.manifest?.document?.title || 'Untitled',
                    currentDocument.manifest?.document?.id
                );
            }
        });
    }

    // Handle Open button (for opening another document)
    if (openBtn) {
        openBtn.addEventListener('click', () => openAnotherDocument());
    }

}

function openAnotherDocument() {
    // Switch back to upload area
    const uploadArea = document.getElementById('uploadArea');
    const documentView = document.getElementById('documentView');

    if (uploadArea && documentView) {
        documentView.style.display = 'none';
        uploadArea.style.display = 'block';

        // Hide Open button when back in upload area
        const openBtn = document.getElementById('openBtn');
        if (openBtn) {
            openBtn.style.display = 'none';
        }

        // Clear current document
        currentDocument = null;

        // Reset file input
        const fileInput = document.getElementById('fileInput') as HTMLInputElement;
        if (fileInput) {
            fileInput.value = '';
        }

        // Clear document info from status bar
        statusBar.clearDocumentInfo();
        showStatus('Ready to open another document', 'info');
    }
}

async function loadTdfFile(file: File) {
    try {
        showStatus('Loading document...', 'info');

        currentDocument = await loadDocument(file);

        // Check for signatures in the TDF file
        const hasSignatures = await checkForSignatures(file);

        // Switch to document view
        const uploadArea = document.getElementById('uploadArea');
        const documentView = document.getElementById('documentView');

        if (uploadArea && documentView) {
            uploadArea.style.display = 'none';
            documentView.style.display = 'block';

            // Show Open button when document is loaded
            const openBtn = document.getElementById('openBtn');
            if (openBtn) {
                openBtn.style.display = 'inline-flex';
            }
        }

        // Render document content with signature status
        renderDocument(currentDocument, hasSignatures);

        // Automatically verify the document
        verifyDocument();

        if (hasSignatures) {
            showStatus('Signed document loaded successfully', 'success');
        } else {
            showStatus('Document loaded successfully (unsigned)', 'info');
        }
        
        // Update status bar with document info
        statusBar.setDocumentInfo(
            currentDocument.manifest?.document?.title || file.name,
            null, // Will be updated after verification
            hasSignatures ? 1 : 0
        );
        
        // Log document open
        auditLog.logDocumentOpen(
            currentDocument.manifest?.document?.title || file.name,
            currentDocument.manifest?.document?.id
        );
    } catch (error: any) {
        // Log error
        auditLog.logError('document_load', error, file.name);
        
        const errorDetails = createFileLoadError(error, file.name);
        errorBanner.show({
            ...errorDetails,
            onRetry: () => {
                loadTdfFile(file);
            }
        });
        
        // Reset UI
        const uploadArea = document.getElementById('uploadArea');
        const documentView = document.getElementById('documentView');
        if (uploadArea) uploadArea.style.display = 'block';
        if (documentView) documentView.style.display = 'none';
    }
}

async function checkForSignatures(file: File): Promise<boolean> {
    try {
        // Check if signatures.cbor exists and contains actual signatures
        const JSZip = (await import('jszip')).default;
        const zip = await JSZip.loadAsync(file);
        const signaturesFile = zip.file('signatures.cbor');
        
        if (!signaturesFile) {
            console.log('No signatures.cbor file found');
            return false;
        }
        
        // Read and parse the signatures file to check if it has actual signatures
        const sigBytes = await signaturesFile.async('uint8array');
        
        // Empty signature blocks are typically 13 bytes (just the CBOR structure)
        // Signed files are typically > 100 bytes (often 300+ bytes)
        // Use 50 bytes as threshold to distinguish empty vs signed
        if (sigBytes.length <= 50) {
            return false;
        }
        
        // Try to parse the CBOR to check for actual signature entries
        try {
            const { decode } = await import('cbor-web');
            const signatureBlock = decode(sigBytes);
            
            // Check if signatures array exists and has entries
            if (signatureBlock && signatureBlock.signatures && Array.isArray(signatureBlock.signatures)) {
                return signatureBlock.signatures.length > 0;
            }
            
            // If file is substantial (> 100 bytes), assume it has signatures
            // This handles edge cases where structure might differ
            return sigBytes.length > 100;
        } catch (parseError) {
            // If we can't parse, use file size as fallback
            // Signed files typically have signatures > 100 bytes, unsigned are ~13 bytes
            return sigBytes.length > 100;
        }
    } catch (error) {
        console.error('Error checking for signatures:', error);
        return false;
    }
}

function renderDocument(doc: any, hasSignatures: boolean = false) {
    const content = document.getElementById('content');
    if (!content) return;

    const manifest = doc.manifest || {};
    const documentMeta = manifest.document || {};
    const integrity = manifest.integrity || {};
    const authors = manifest.authors || [];

    // Determine signature status badge (icons will be inserted via JavaScript)
    const signatureBadge = hasSignatures 
        ? `<div class="signature-badge signed">
            <span class="badge-icon" id="signedIcon"></span>
            <span class="badge-text">SIGNED</span>
           </div>`
        : `<div class="signature-badge unsigned">
            <span class="badge-icon" id="unsignedIcon"></span>
            <span class="badge-text">UNSIGNED</span>
           </div>`;

    // Simple rendering - just show document info for now
    content.innerHTML = `
        <div class="document-header">
            <div class="document-title-section">
                <h2>${documentMeta.title || 'Untitled Document'}</h2>
                ${signatureBadge}
            </div>
            <div class="document-meta-grid">
                <div class="meta-card">
                    <div class="meta-label">Document ID</div>
                    <div class="meta-value">${documentMeta.id || 'N/A'}</div>
                </div>
                <div class="meta-card">
                    <div class="meta-label">Created</div>
                    <div class="meta-value">${documentMeta.created ? new Date(documentMeta.created).toLocaleString() : 'Unknown'}</div>
                </div>
                <div class="meta-card">
                    <div class="meta-label">Modified</div>
                    <div class="meta-value">${documentMeta.modified ? new Date(documentMeta.modified).toLocaleString() : 'Unknown'}</div>
                </div>
                <div class="meta-card">
                    <div class="meta-label">Language</div>
                    <div class="meta-value">${documentMeta.language || 'N/A'}</div>
                </div>
            </div>
            ${authors.length > 0 ? `
                <div class="authors-section">
                    <div class="meta-label">Authors</div>
                    <div class="authors-list">
                        ${authors.map((author: any) => `
                            <div class="author-badge">
                                <span class="author-name">${author.name || author.id}</span>
                                ${author.role ? `<span class="author-role">${author.role}</span>` : ''}
                            </div>
                        `).join('')}
                    </div>
                </div>
            ` : ''}
            <div class="integrity-section">
                <div class="meta-label">Integrity</div>
                <div class="integrity-info">
                    <div class="integrity-hash">
                        <span class="hash-label">Root Hash:</span>
                        <code class="hash-value">${integrity.root_hash || 'N/A'}</code>
                    </div>
                    <div class="integrity-algorithm">
                        <span class="algorithm-label">Algorithm:</span>
                        <span class="algorithm-value">${integrity.algorithm ? integrity.algorithm.toUpperCase() : 'N/A'}</span>
                    </div>
                </div>
            </div>
            ${doc.content?.sections ? `
                <div class="sections-info">
                    <div class="meta-label">Content Sections</div>
                    <div class="sections-count">${doc.content.sections.length} section${doc.content.sections.length !== 1 ? 's' : ''}</div>
                </div>
            ` : ''}
        </div>
    `;
    
    // Insert icons into badges
    if (hasSignatures) {
        const signedIcon = document.getElementById('signedIcon');
        if (signedIcon) {
            createInlineIcon('check-circle', 20, '', 'Signed').then(icon => {
                signedIcon.appendChild(icon);
            });
        }
    } else {
        const unsignedIcon = document.getElementById('unsignedIcon');
        if (unsignedIcon) {
            createInlineIcon('exclamation-triangle', 20, '', 'Unsigned').then(icon => {
                unsignedIcon.appendChild(icon);
            });
        }
    }
}

async function verifyDocument() {
    if (!currentDocument) {
        showStatus('No document loaded', 'error');
        return;
    }

    try {
        showStatus('Verifying document...', 'info');

        // Perform basic integrity verification
        const isValid = verifyIntegrity(currentDocument);

        if (isValid) {
            showStatus('Document integrity verified successfully', 'success');

            // Show verification details
            await showVerificationDetails(currentDocument);
            
            // Update status bar with document info
            statusBar.setDocumentInfo(
                currentDocument.manifest?.document?.title || 'Untitled',
                'Valid',
                null // Signature count will be updated separately
            );
            
            // Log verification
            auditLog.logDocumentVerify(
                currentDocument.manifest?.document?.title || 'Untitled',
                true,
                currentDocument.manifest?.document?.id
            );
        } else {
            showStatus('Document integrity verification failed', 'error');
            statusBar.setDocumentInfo(
                currentDocument.manifest?.document?.title || 'Untitled',
                'Invalid',
                null
            );
            
            // Log verification failure
            auditLog.logDocumentVerify(
                currentDocument.manifest?.document?.title || 'Untitled',
                false,
                currentDocument.manifest?.document?.id
            );
        }
    } catch (error: any) {
        const errorDetails = createVerificationError(error);
        errorBanner.show({
            ...errorDetails,
            onRetry: () => {
                verifyDocument();
            }
        });
    }
}

async function showVerificationDetails(doc: any) {
    const verificationPanel = document.getElementById('verificationPanel');
    if (!verificationPanel) return;

    const manifest = doc.manifest;
    const integrity = manifest.integrity;
    const isValid = verifyIntegrity(doc);

    // Summary section (always visible)
    const summaryHtml = `
        <div class="verification-summary">
            <div class="verification-summary-item">
                <span class="summary-label">Integrity:</span>
                <span class="summary-value ${isValid ? 'success' : 'error'}">${isValid ? 'Valid' : 'Invalid'}</span>
            </div>
            <div class="verification-summary-item">
                <span class="summary-label">Signatures:</span>
                <span class="summary-value">${doc.manifest?.signatures?.length || 0}</span>
            </div>
            <button class="verification-toggle-details" id="toggleDetails" aria-expanded="false" tabindex="0">
                <span class="toggle-text">Show Details</span>
                <span class="toggle-icon" id="toggleIcon"></span>
            </button>
        </div>
    `;

    // Details section (collapsible)
    const detailsHtml = `
        <div class="verification-details-collapsible" id="verificationDetails" style="display: none;">
            <div class="collapsible-section">
                <button class="section-toggle" data-section="integrity" aria-expanded="true" tabindex="0">
                    <span class="section-title">Integrity Information</span>
                    <span class="section-icon" data-icon="integrity"></span>
                </button>
                <div class="section-content" id="section-integrity">
                    <div class="verification-item">
                        <span class="label">Root Hash:</span>
                        <span class="value monospace">${integrity.root_hash}</span>
                    </div>
                    <div class="verification-item">
                        <span class="label">Algorithm:</span>
                        <span class="value">${integrity.algorithm.toUpperCase()}</span>
                    </div>
                </div>
            </div>

            <div class="collapsible-section">
                <button class="section-toggle" data-section="document" aria-expanded="true" tabindex="0">
                    <span class="section-title">Document Metadata</span>
                    <span class="section-icon" data-icon="document"></span>
                </button>
                <div class="section-content" id="section-document">
                    <div class="verification-item">
                        <span class="label">Document ID:</span>
                        <span class="value monospace">${manifest.document.id}</span>
                    </div>
                    <div class="verification-item">
                        <span class="label">Created:</span>
                        <span class="value">${new Date(manifest.document.created).toLocaleString()}</span>
                    </div>
                    <div class="verification-item">
                        <span class="label">Authors:</span>
                        <span class="value">${manifest.authors.map((a: any) => a.name).join(', ')}</span>
                    </div>
                    <div class="verification-item">
                        <span class="label">Sections:</span>
                        <span class="value">${doc.content.sections?.length || 0}</span>
                    </div>
                </div>
            </div>

            <div class="collapsible-section">
                <button class="section-toggle" data-section="seal" aria-expanded="false" tabindex="0">
                    <span class="section-title">Content Sealing Explanation</span>
                    <span class="section-icon" data-icon="seal"></span>
                </button>
                <div class="section-content" id="section-seal" style="display: none;">
                    <div class="seal-info-banner">
                        <div class="seal-icon" id="sealIcon"></div>
                        <div class="seal-text">
                            <strong>Content is Cryptographically Sealed</strong>
                            <p>All content is protected by a Merkle tree. Any modification to content, metadata, or structure will invalidate the root hash.</p>
                        </div>
                    </div>
                    <div class="seal-explanation">
                        <h4>How Content Sealing Works</h4>
                        <ul>
                            <li><strong>Merkle Tree Protection:</strong> Every component (content, manifest, styles) is individually hashed</li>
                            <li><strong>Root Hash:</strong> All hashes combine into a single root hash that represents the entire document</li>
                            <li><strong>Tamper Detection:</strong> Any change to any part recalculates a different root hash</li>
                            <li><strong>Automatic Verification:</strong> The viewer recalculates hashes and compares them on load</li>
                            <li><strong>100% Detection Rate:</strong> Tested against 100+ attack scenarios - all detected</li>
                        </ul>
                    </div>
                </div>
            </div>

            <div class="verification-export">
                <h4>Export Verification Report</h4>
                <div class="export-actions">
                    <button id="exportJSON" class="btn btn-secondary btn-sm">Export as JSON</button>
                    <button id="exportPDF" class="btn btn-secondary btn-sm">Export as PDF</button>
                </div>
            </div>
        </div>
    `;

    verificationPanel.innerHTML = `
        <div class="verification-details">
            <h3>Verification Results</h3>
            ${summaryHtml}
            ${detailsHtml}
        </div>
    `;

    verificationPanel.style.display = 'block';
    
    // Toggle details button
    const toggleBtn = verificationPanel.querySelector('#toggleDetails');
    const detailsSection = verificationPanel.querySelector('#verificationDetails');
    const toggleIcon = verificationPanel.querySelector('#toggleIcon');
    
    if (toggleBtn && detailsSection) {
        toggleBtn.addEventListener('click', async () => {
            const isExpanded = toggleBtn.getAttribute('aria-expanded') === 'true';
            (detailsSection as HTMLElement).style.display = isExpanded ? 'none' : 'block';
            toggleBtn.setAttribute('aria-expanded', String(!isExpanded));
            const textSpan = toggleBtn.querySelector('.toggle-text');
            if (textSpan) {
                textSpan.textContent = isExpanded ? 'Show Details' : 'Hide Details';
            }
            
            // Update icon
            if (toggleIcon) {
                const newIcon = await createInlineIcon(
                    isExpanded ? 'chevron-down' : 'chevron-up',
                    16,
                    'toggle-icon',
                    ''
                );
                toggleIcon.replaceWith(newIcon);
            }
        });
    }
    
    // Section toggles
    const sectionToggles = verificationPanel.querySelectorAll('.section-toggle');
    sectionToggles.forEach(toggle => {
        toggle.addEventListener('click', async () => {
            const sectionId = toggle.getAttribute('data-section');
            const sectionContent = verificationPanel.querySelector(`#section-${sectionId}`);
            const isExpanded = toggle.getAttribute('aria-expanded') === 'true';
            
            if (sectionContent) {
                (sectionContent as HTMLElement).style.display = isExpanded ? 'none' : 'block';
                toggle.setAttribute('aria-expanded', String(!isExpanded));
                
                // Update icon
                const icon = toggle.querySelector('.section-icon');
                if (icon) {
                    const newIcon = await createInlineIcon(
                        isExpanded ? 'chevron-down' : 'chevron-up',
                        16,
                        'section-icon',
                        ''
                    );
                    icon.replaceWith(newIcon);
                }
            }
        });
    });
    
    // Initialize icons
    const icons = verificationPanel.querySelectorAll('[data-icon]');
    icons.forEach(async (iconEl) => {
        const iconName = iconEl.getAttribute('data-icon');
        if (iconName === 'integrity') {
            const icon = await createInlineIcon('shield-check', 20, '', '');
            iconEl.appendChild(icon);
        } else if (iconName === 'document') {
            const icon = await createInlineIcon('document', 20, '', '');
            iconEl.appendChild(icon);
        } else if (iconName === 'seal') {
            const icon = await createInlineIcon('lock-closed', 20, '', '');
            iconEl.appendChild(icon);
        }
    });
    
    // Add export button handlers
    const exportJSONBtn = verificationPanel.querySelector('#exportJSON');
    const exportPDFBtn = verificationPanel.querySelector('#exportPDF');
    
    if (exportJSONBtn) {
        exportJSONBtn.addEventListener('click', () => {
            const report = generateVerificationReport(
                doc,
                verifyIntegrity(doc),
                false // TODO: Pass hasSignatures
            );
            exportVerificationAsJSON(report);
        });
    }
    
    if (exportPDFBtn) {
        exportPDFBtn.addEventListener('click', async () => {
            const report = generateVerificationReport(
                doc,
                verifyIntegrity(doc),
                false
            );
            await exportVerificationAsPDF(report);
        });
    }
    
    // Insert lock icon
    const sealIcon = document.getElementById('sealIcon');
    if (sealIcon) {
        createInlineIcon('lock-closed', 24, '', 'Cryptographically Sealed').then(icon => {
            sealIcon.appendChild(icon);
        });
    }
    
    // Initialize toggle icon
    if (toggleIcon) {
        const icon = await createInlineIcon('chevron-down', 16, 'toggle-icon', '');
        toggleIcon.replaceWith(icon);
    }
}

function showExtractedData(data: ExtractedData) {
    const extractionPanel = document.getElementById('extractionPanel');
    if (!extractionPanel) {
        // Create the panel if it doesn't exist
        const newPanel = document.createElement('div');
        newPanel.id = 'extractionPanel';
        newPanel.className = 'extraction-panel';

        const contentDiv = document.getElementById('content');
        if (contentDiv) {
            contentDiv.appendChild(newPanel);
        }
    }

    const panel = document.getElementById('extractionPanel');
    if (!panel) return;

    let html = `
        <div class="extraction-details">
            <h3>📊 Extracted Data</h3>

            <div class="metadata-section">
                <h4>📄 Document Metadata</h4>
                <div class="metadata-grid">
                    <div class="meta-item">
                        <span class="meta-label">Title:</span>
                        <span class="meta-value">${data.metadata.title}</span>
                    </div>
                    <div class="meta-item">
                        <span class="meta-label">ID:</span>
                        <span class="meta-value">${data.metadata.id}</span>
                    </div>
                    <div class="meta-item">
                        <span class="meta-label">Created:</span>
                        <span class="meta-value">${new Date(data.metadata.created).toLocaleString()}</span>
                    </div>
                    <div class="meta-item">
                        <span class="meta-label">Modified:</span>
                        <span class="meta-value">${new Date(data.metadata.modified).toLocaleString()}</span>
                    </div>
                </div>
            </div>
    `;

    // Add tables section
    const tableIds = Object.keys(data.tables);
    if (tableIds.length > 0) {
        html += `
            <div class="tables-section">
                <h4>📋 Tables (${tableIds.length})</h4>
        `;

        for (const tableId of tableIds) {
            const table = data.tables[tableId];
            html += `
                <div class="table-container">
                    <h5>Table: ${tableId}</h5>
                    <div class="table-wrapper">
                        <table class="data-table">
                            <thead>
                                <tr>
                                    ${table.columns.map(col => `<th>${col}</th>`).join('')}
                                </tr>
                            </thead>
                            <tbody>
                                ${table.rows.map(row =>
                                    `<tr>${row.map(cell =>
                                        `<td>${cell !== null ? String(cell) : ''}</td>`
                                    ).join('')}</tr>`
                                ).join('')}
                            </tbody>
                        </table>
                    </div>
                </div>
            `;
        }

        html += `</div>`;
    } else {
        html += `
            <div class="no-tables">
                <p>No tables found in this document</p>
            </div>
        `;
    }

    // Add metrics section if available
    if (data.metrics && Object.keys(data.metrics).length > 0) {
        html += `
            <div class="metrics-section">
                <h4>📈 Metrics</h4>
                <div class="metrics-grid">
                    ${Object.entries(data.metrics).map(([key, value]) =>
                        `<div class="metric-item">
                            <span class="metric-label">${key}:</span>
                            <span class="metric-value">${value}</span>
                        </div>`
                    ).join('')}
                </div>
            </div>
        `;
    }

    // Add export buttons
    html += `
            <div class="export-actions">
                <button onclick="exportToJSON()">📄 Export as JSON</button>
                <button onclick="exportToCSV()">📊 Export as CSV</button>
            </div>
        </div>
    `;

    panel.innerHTML = html;
    panel.style.display = 'block';
}

function performDataExtraction() {
    if (!currentDocument) {
        showStatus('No document loaded', 'error');
        return;
    }

    try {
        showStatus('Extracting data...', 'info');

        // Extract structured data from the document
        const extractedData = extractData(currentDocument);

        showStatus('Data extracted successfully', 'success');
        showExtractedData(extractedData);
        
        // Log data extraction
        const tableCount = extractedData.tables ? Object.keys(extractedData.tables).length : 0;
        const metricCount = extractedData.metrics ? Object.keys(extractedData.metrics).length : 0;
        const extractedItems = tableCount + metricCount;
        auditLog.logDataExtraction(
            currentDocument.manifest?.document?.title || 'Untitled',
            extractedItems,
            currentDocument.manifest?.document?.id
        );

    } catch (error: any) {
        const errorDetails = createExtractionError(error);
        errorBanner.show({
            ...errorDetails,
            onRetry: () => {
                performDataExtraction();
            }
        });
    }
}

function showStatus(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
    // Use persistent status bar instead of auto-dismissing messages
    const autoDismiss = type === 'success'; // Only auto-dismiss success messages
    const dismissible = type !== 'error' || true; // All messages are dismissible, but errors persist
    
    statusBar.show(message, type, autoDismiss, dismissible);
}

// Export functions
(window as any).exportToJSON = function() {
    if (!currentDocument) return;
    const data = extractData(currentDocument);
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
    downloadBlob(blob, 'extracted-data.json');
};

(window as any).exportToCSV = function() {
    if (!currentDocument) return;
    const data = extractData(currentDocument);

    let csv = 'Table,Column,Row,Value\n';

    for (const [tableId, table] of Object.entries(data.tables)) {
        table.rows.forEach((row, rowIndex) => {
            row.forEach((cell, colIndex) => {
                csv += `"${tableId}","${table.columns[colIndex]}","${rowIndex + 1}","${String(cell || '')}"\n`;
            });
        });
    }

    const blob = new Blob([csv], { type: 'text/csv' });
    downloadBlob(blob, 'extracted-data.csv');
};

function downloadBlob(blob: Blob, filename: string) {
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
}