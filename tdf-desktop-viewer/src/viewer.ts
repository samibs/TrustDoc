import { loadDocument, TdfDocument } from 'tdf-ts';
import { renderDocument } from './renderer';
import { renderDiagram } from './diagram';

let currentDocument: TdfDocument | null = null;

const uploadArea = document.getElementById('uploadArea')!;
const fileInput = document.getElementById('fileInput') as HTMLInputElement;
const browseBtn = document.getElementById('browseBtn')!;
const documentView = document.getElementById('documentView')!;
const content = document.getElementById('content')!;
const status = document.getElementById('status')!;
const verifyBtn = document.getElementById('verifyBtn')!;
const extractBtn = document.getElementById('extractBtn')!;
const printBtn = document.getElementById('printBtn')!;

browseBtn.addEventListener('click', () => fileInput.click());

fileInput.addEventListener('change', async (e) => {
    const file = (e.target as HTMLInputElement).files?.[0];
    if (file) {
        await loadTdfFile(file);
    }
});

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
    
    const file = e.dataTransfer?.files[0];
    if (file && file.name.endsWith('.tdf')) {
        await loadTdfFile(file);
    } else {
        showStatus('Please drop a .tdf file', 'error');
    }
});

async function loadTdfFile(file: File) {
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
    if (!currentDocument) return;
    
    showStatus('Verifying document...', 'info');
    
    try {
        // Re-read file for verification
        const fileInput = document.getElementById('fileInput') as HTMLInputElement;
        if (fileInput.files && fileInput.files[0]) {
            const { verifyDocument } = await import('./verification');
            const result = await verifyDocument(fileInput.files[0]);
            
            if (result.integrityValid) {
                let message = `✓ Document structure valid. Root hash: ${result.rootHash}`;
                if (result.signatureCount > 0) {
                    message += ` (${result.signatureCount} signature${result.signatureCount > 1 ? 's' : ''})`;
                }
                if (result.warnings && result.warnings.length > 0) {
                    message += `\n⚠️ ${result.warnings[0]}`;
                }
                showStatus(message, 'success');
            } else {
                let message = `✗ Verification failed: ${result.errors.join(', ')}`;
                if (result.warnings && result.warnings.length > 0) {
                    message += `\n⚠️ ${result.warnings[0]}`;
                }
                showStatus(message, 'error');
            }
        } else {
            showStatus('No file available for verification', 'error');
        }
    } catch (error) {
        showStatus(`Verification error: ${error}`, 'error');
    }
});

extractBtn.addEventListener('click', () => {
    if (!currentDocument) return;
    
    // Extract structured data
    const data = {
        title: currentDocument.manifest.document.title,
        id: currentDocument.manifest.document.id,
        sections: currentDocument.content.sections.map(s => ({
            id: s.id,
            title: s.title,
            blockCount: s.content.length,
        })),
    };
    
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${currentDocument.manifest.document.id}.json`;
    a.click();
    URL.revokeObjectURL(url);
    
    showStatus('Data extracted and downloaded', 'success');
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

