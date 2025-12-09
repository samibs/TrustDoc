import { loadDocument, TdfDocument } from 'tdf-ts';
import { renderDocument } from './renderer';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { readBinaryFile, writeTextFile } from '@tauri-apps/plugin-fs';

let currentDocument: TdfDocument | null = null;
let currentFilePath: string | null = null;

const uploadArea = document.getElementById('uploadArea')!;
const browseBtn = document.getElementById('browseBtn')!;
const documentView = document.getElementById('documentView')!;
const content = document.getElementById('content')!;
const status = document.getElementById('status')!;
const verifyBtn = document.getElementById('verifyBtn')!;
const extractBtn = document.getElementById('extractBtn')!;
const printBtn = document.getElementById('printBtn')!;
const openBtn = document.getElementById('openBtn')!;
const verificationPanel = document.getElementById('verificationPanel')!;

// Browse button
browseBtn.addEventListener('click', async () => {
    await openTdfFile();
});

// Open button
openBtn.addEventListener('click', async () => {
    await openTdfFile();
});

// Drag and drop
uploadArea.addEventListener('dragover', (e) => {
    e.preventDefault();
    uploadArea.classList.add('dragover');
});

uploadArea.addEventListener('dragleave', () => {
    uploadArea.classList.remove('dragover');
});

uploadArea.addEventListener('drop', async (e) => {
    e.preventDefault();
    uploadArea.classList.remove('dragover');
    
    const files = e.dataTransfer?.files;
    if (files && files.length > 0) {
        const file = files[0];
        if (file.name.endsWith('.tdf')) {
            await loadTdfFileFromBlob(file);
        } else {
            showStatus('Please drop a .tdf file', 'error');
        }
    }
});

// File menu handler (from Tauri)
window.addEventListener('tauri://menu', async (event: any) => {
    if (event.detail?.id === 'open-file') {
        await openTdfFile();
    }
});

async function openTdfFile() {
    try {
        const selected = await open({
            filters: [{
                name: 'TDF Documents',
                extensions: ['tdf']
            }],
            multiple: false,
        });

        if (selected && typeof selected === 'string') {
            currentFilePath = selected;
            await loadTdfFileFromPath(selected);
        }
    } catch (error) {
        showStatus(`Error opening file: ${error}`, 'error');
    }
}

async function loadTdfFileFromPath(filePath: string) {
    try {
        showStatus('Loading document...', 'info');
        
        // Read file using Tauri
        const fileData = await readBinaryFile(filePath);
        const blob = new Blob([fileData], { type: 'application/zip' });
        const file = new File([blob], filePath.split('/').pop() || 'document.tdf', { type: 'application/zip' });
        
        await loadTdfFileFromBlob(file);
    } catch (error) {
        showStatus(`Error loading document: ${error}`, 'error');
    }
}

async function loadTdfFileFromBlob(file: File | Blob) {
    try {
        showStatus('Loading document...', 'info');
        currentDocument = await loadDocument(file);
        
        uploadArea.style.display = 'none';
        documentView.style.display = 'block';
        
        renderDocument(currentDocument, content);
        showStatus('Document loaded successfully', 'success');
    } catch (error) {
        showStatus(`Error loading document: ${error}`, 'error');
    }
}

verifyBtn.addEventListener('click', async () => {
    if (!currentDocument || !currentFilePath) {
        showStatus('No document loaded', 'error');
        return;
    }
    
    showStatus('Verifying document...', 'info');
    
    try {
        // Use Tauri backend for verification (faster, native)
        const result = await invoke('verify_document', { filePath: currentFilePath }) as {
            integrity_valid: boolean;
            root_hash: string;
            signature_count: number;
            timestamp_warnings: string[];
        };
        
        verificationPanel.style.display = 'block';
        verificationPanel.innerHTML = `
            <div class="verification-result">
                <h3>🔍 Verification Results</h3>
                <div class="result-item">
                    <strong>Integrity:</strong> 
                    <span class="${result.integrity_valid ? 'valid' : 'invalid'}">
                        ${result.integrity_valid ? '✓ VALID' : '✗ INVALID'}
                    </span>
                </div>
                <div class="result-item">
                    <strong>Root Hash:</strong> 
                    <code>${result.root_hash}</code>
                </div>
                <div class="result-item">
                    <strong>Signatures:</strong> ${result.signature_count}
                </div>
                ${result.timestamp_warnings.length > 0 ? `
                    <div class="result-item warning">
                        <strong>Warnings:</strong>
                        <ul>
                            ${result.timestamp_warnings.map(w => `<li>${w}</li>`).join('')}
                        </ul>
                    </div>
                ` : ''}
            </div>
        `;
        
        if (result.integrity_valid) {
            showStatus('✓ Document verified successfully', 'success');
        } else {
            showStatus('✗ Document verification failed', 'error');
        }
    } catch (error) {
        showStatus(`Verification error: ${error}`, 'error');
    }
});

extractBtn.addEventListener('click', async () => {
    if (!currentDocument) return;
    
    try {
        // Extract structured data
        const data = {
            title: currentDocument.manifest.document.title,
            id: currentDocument.manifest.document.id,
            created: currentDocument.manifest.document.created,
            modified: currentDocument.manifest.document.modified,
            sections: currentDocument.content.sections.map(s => ({
                id: s.id,
                title: s.title,
                blocks: s.content.map(b => ({
                    type: b.type,
                    id: 'id' in b ? b.id : undefined,
                })),
            })),
        };
        
        // Use Tauri save dialog
        const { save } = await import('@tauri-apps/plugin-dialog');
        const savePath = await save({
            filters: [{
                name: 'JSON',
                extensions: ['json']
            }],
            defaultPath: `${currentDocument.manifest.document.id}.json`,
        });
        
        if (savePath) {
            await writeTextFile(savePath, JSON.stringify(data, null, 2));
            showStatus('Data extracted and saved', 'success');
        }
    } catch (error) {
        showStatus(`Error extracting data: ${error}`, 'error');
    }
});

printBtn.addEventListener('click', () => {
    window.print();
});

function showStatus(message: string, type: 'info' | 'success' | 'error') {
    status.textContent = message;
    status.className = `status ${type}`;
    setTimeout(() => {
        status.textContent = '';
        status.className = 'status';
    }, 5000);
}

// Handle file open from command line or file association
// Tauri will handle file associations automatically via tauri.conf.json

