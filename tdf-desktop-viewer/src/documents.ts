// Document Operations UI

import { loadDocument, TdfDocument } from 'tdf-ts';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { renderDocument } from './renderer';
import { updateStatus } from './app';
import { renderVerificationPanel } from './verification-panel';

let currentDocument: TdfDocument | null = null;
let currentFilePath: string | null = null;

export async function initDocuments() {
    const openDocBtn = document.getElementById('openDocBtn');
    const browseFilesBtn = document.getElementById('browseFilesBtn');
    const createDocBtn = document.getElementById('createDocBtn');
    const verifyBtn = document.getElementById('verifyBtn');
    const signBtn = document.getElementById('signBtn');
    const documentList = document.getElementById('document-list');

    // Setup drag and drop
    if (documentList) {
        setupDragAndDrop(documentList);
    }

    // Connect both buttons to open document
    if (openDocBtn) {
        openDocBtn.addEventListener('click', () => openDocument());
    }

    if (browseFilesBtn) {
        browseFilesBtn.addEventListener('click', () => openDocument());
    }

    if (createDocBtn) {
        createDocBtn.addEventListener('click', () => showCreateDocumentWizard());
    }

    if (verifyBtn) {
        verifyBtn.addEventListener('click', () => verifyCurrentDocument());
    }

    if (signBtn) {
        signBtn.addEventListener('click', () => showSignDocumentDialog());
    }
}

function setupDragAndDrop(container: HTMLElement) {
    container.addEventListener('dragover', (e) => {
        e.preventDefault();
        container.classList.add('dragover');
    });

    container.addEventListener('dragleave', () => {
        container.classList.remove('dragover');
    });

    container.addEventListener('drop', async (e) => {
        e.preventDefault();
        container.classList.remove('dragover');
        
        const files = e.dataTransfer?.files;
        if (files && files.length > 0) {
            const file = files[0];
            if (file.name.endsWith('.tdf')) {
                await loadTdfFileFromBlob(file);
            } else {
                updateStatus('Please drop a .tdf file', 'error');
            }
        }
    });
}

async function openDocument() {
    try {
        updateStatus('Opening file dialog...', 'info');
        console.log('Opening file dialog...');
        
        const selected = await open({
            filters: [{
                name: 'TDF Documents',
                extensions: ['tdf']
            }],
            multiple: false,
            title: 'Select TDF Document',
        });

        console.log('File dialog result:', selected);

        if (selected) {
            // Handle both string (single file) and array (multiple files - though we set multiple: false)
            const filePath = typeof selected === 'string' ? selected : (Array.isArray(selected) ? selected[0] : null);
            if (filePath) {
                console.log('Selected file:', filePath);
                currentFilePath = filePath;
                await loadTdfFileFromPath(filePath);
            } else {
                console.log('No file path extracted from selection');
                updateStatus('No file selected', 'info');
            }
        } else {
            console.log('No file selected');
            updateStatus('No file selected', 'info');
        }
    } catch (error) {
        console.error('Error opening file dialog:', error);
        updateStatus(`Error opening file: ${error}`, 'error');
        // Show alert for debugging
        alert(`Error opening file dialog: ${error}`);
    }
}

async function loadTdfFileFromPath(filePath: string) {
    try {
        updateStatus('Loading document...', 'info');
        
        // Use Tauri invoke to read file as binary
        const fileData = await invoke<number[]>('read_file_binary', { path: filePath });
        const uint8Array = new Uint8Array(fileData);
        const blob = new Blob([uint8Array], { type: 'application/zip' });
        const file = new File([blob], filePath.split('/').pop() || 'document.tdf', { type: 'application/zip' });
        
        await loadTdfFileFromBlob(file);
    } catch (error) {
        updateStatus(`Error loading document: ${error}`, 'error');
    }
}

async function loadTdfFileFromBlob(file: File | Blob) {
    try {
        updateStatus('Loading document...', 'info');
        currentDocument = await loadDocument(file);
        
        const documentViewer = document.getElementById('document-viewer');
        const documentList = document.getElementById('document-list');
        const content = document.getElementById('document-content');

        if (documentViewer && documentList && content) {
            documentList.style.display = 'none';
            documentViewer.style.display = 'block';
            
            renderDocument(currentDocument, content);
            updateStatus('Document loaded successfully', 'success');
        }
    } catch (error) {
        updateStatus(`Error loading document: ${error}`, 'error');
    }
}

async function verifyCurrentDocument() {
    if (!currentDocument || !currentFilePath) {
        updateStatus('No document loaded', 'error');
        return;
    }
    
    updateStatus('Verifying document...', 'info');
    
    try {
        const result = await invoke('verify_document_enhanced', { filePath: currentFilePath }) as {
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
        };
        
        const verificationPanel = document.getElementById('verification-panel');
        if (verificationPanel) {
            verificationPanel.style.display = 'block';
            renderVerificationPanel(result, verificationPanel);
        }
        
        if (result.integrity_valid && result.signatures.every(s => s.valid)) {
            updateStatus('✓ Document verified successfully', 'success');
        } else {
            updateStatus('✗ Document verification failed', 'error');
        }
    } catch (error) {
        updateStatus(`Verification error: ${error}`, 'error');
    }
}

function showSignDocumentDialog() {
    // TODO: Implement sign document dialog
    updateStatus('Sign document feature coming soon', 'info');
}

function showCreateDocumentWizard() {
    // TODO: Implement document creation wizard
    updateStatus('Create document feature coming soon', 'info');
}
