// Key Management UI Logic

import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import { showModal, closeModal, updateStatus } from './app';
import { renderKeyList } from './key-list';
import { renderKeyDetails } from './key-details';

export interface KeyInfo {
    id: string;
    name: string;
    algorithm: string;
    created: string;
    signer_id?: string;
    fingerprint?: string;
}

// selectedKeyId is managed by key-list.ts

export async function initKeys() {
    const newKeyBtn = document.getElementById('newKeyBtn');
    const importKeyBtn = document.getElementById('importKeyBtn');

    if (newKeyBtn) {
        newKeyBtn.addEventListener('click', () => showGenerateKeyDialog());
    }

    if (importKeyBtn) {
        importKeyBtn.addEventListener('click', () => showImportKeyDialog());
    }

    // Load and display keys
    await refreshKeyList();
}

async function refreshKeyList() {
    try {
        const keys = await invoke<KeyInfo[]>('list_keys');
        renderKeyList(keys, onKeySelect, onKeyDelete, onKeyExport);
        updateStatus(`Loaded ${keys.length} key(s)`, 'success');
    } catch (error) {
        updateStatus(`Error loading keys: ${error}`, 'error');
    }
}

function onKeySelect(keyId: string) {
    loadKeyDetails(keyId);
}

async function onKeyDelete(keyId: string) {
    if (!confirm('Are you sure you want to delete this key? This action cannot be undone.')) {
        return;
    }

    try {
        await invoke('delete_key', { keyId });
        updateStatus('Key deleted successfully', 'success');
        await refreshKeyList();
        const detailsContainer = document.getElementById('key-details-container');
        if (detailsContainer) {
            detailsContainer.style.display = 'none';
        }
    } catch (error) {
        updateStatus(`Error deleting key: ${error}`, 'error');
    }
}

async function onKeyExport(keyId: string) {
    try {
        const savePath = await save({
            filters: [{
                name: 'TDF Key',
                extensions: ['signing']
            }],
            defaultPath: `key-${keyId}.signing`,
        });

        if (savePath) {
            await invoke('export_key', { keyId, path: savePath });
            updateStatus('Key exported successfully', 'success');
        }
    } catch (error) {
        updateStatus(`Error exporting key: ${error}`, 'error');
    }
}

async function loadKeyDetails(keyId: string) {
    try {
        const details = await invoke<KeyInfo>('get_key_details', { keyId });
        renderKeyDetails(details);
        document.getElementById('key-details-container')!.style.display = 'block';
    } catch (error) {
        updateStatus(`Error loading key details: ${error}`, 'error');
    }
}

function showGenerateKeyDialog() {
    const content = document.createElement('div');
    content.innerHTML = `
        <div class="form-group">
            <label class="form-label">Key Name</label>
            <input type="text" id="keyName" class="form-input" placeholder="my-signing-key" required>
            <div class="form-help">A friendly name to identify this key</div>
        </div>
        <div class="form-group">
            <label class="form-label">Algorithm</label>
            <select id="keyAlgorithm" class="form-select">
                <option value="ed25519">Ed25519 (Recommended)</option>
                <option value="secp256k1">secp256k1 (Web3 Compatible)</option>
            </select>
            <div class="form-help">Ed25519 is faster and more secure. secp256k1 is compatible with Ethereum/Web3.</div>
        </div>
        <div class="form-group">
            <label class="form-label">Signer ID (Optional)</label>
            <input type="text" id="signerId" class="form-input" placeholder="did:web:example.com">
            <div class="form-help">Decentralized Identifier for this key</div>
        </div>
    `;

    const footer = document.createElement('div');
    footer.innerHTML = `
        <button class="btn btn-secondary" onclick="closeModal()">Cancel</button>
        <button class="btn btn-primary" id="generateKeyBtn">Generate Key</button>
    `;

    showModal('Generate New Key', content, footer);

    const generateBtn = document.getElementById('generateKeyBtn');
    const keyNameInput = document.getElementById('keyName') as HTMLInputElement;
    const algorithmSelect = document.getElementById('keyAlgorithm') as HTMLSelectElement;
    const signerIdInput = document.getElementById('signerId') as HTMLInputElement;

    if (generateBtn) {
        generateBtn.addEventListener('click', async () => {
            const name = keyNameInput?.value.trim();
            const algorithm = algorithmSelect?.value || 'ed25519';
            const signerId = signerIdInput?.value.trim() || undefined;

            if (!name) {
                alert('Please enter a key name');
                return;
            }

            try {
                updateStatus('Generating key...', 'info');
                await invoke('generate_key', { name, algorithm, signerId });
                updateStatus('Key generated successfully', 'success');
                closeModal();
                await refreshKeyList();
            } catch (error) {
                updateStatus(`Error generating key: ${error}`, 'error');
            }
        });
    }

    // Allow Enter key to submit
    if (keyNameInput) {
        keyNameInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter' && generateBtn) {
                generateBtn.click();
            }
        });
    }
}

async function showImportKeyDialog() {
    try {
        const selected = await open({
            filters: [{
                name: 'TDF Keys',
                extensions: ['signing', 'verifying']
            }],
            multiple: false,
        });

        if (!selected || typeof selected !== 'string') {
            return;
        }

        const content = document.createElement('div');
        content.innerHTML = `
            <div class="form-group">
                <label class="form-label">Key File</label>
                <input type="text" id="keyFilePath" class="form-input" value="${selected}" readonly>
            </div>
            <div class="form-group">
                <label class="form-label">Key Name</label>
                <input type="text" id="importKeyName" class="form-input" placeholder="imported-key" required>
                <div class="form-help">A friendly name for this imported key</div>
            </div>
            <div class="form-group">
                <label class="form-label">Password (Optional)</label>
                <input type="password" id="keyPassword" class="form-input" placeholder="Leave empty if not encrypted">
                <div class="form-help">If the key file is password-protected, enter the password here</div>
            </div>
        `;

        const footer = document.createElement('div');
        footer.innerHTML = `
            <button class="btn btn-secondary" onclick="closeModal()">Cancel</button>
            <button class="btn btn-primary" id="importKeyBtn">Import Key</button>
        `;

        showModal('Import Key', content, footer);

        const importBtn = document.getElementById('importKeyBtn');
        const keyNameInput = document.getElementById('importKeyName') as HTMLInputElement;
        const passwordInput = document.getElementById('keyPassword') as HTMLInputElement;

        if (importBtn) {
            importBtn.addEventListener('click', async () => {
                const name = keyNameInput?.value.trim();
                const password = passwordInput?.value || undefined;

                if (!name) {
                    alert('Please enter a key name');
                    return;
                }

                try {
                    updateStatus('Importing key...', 'info');
                    await invoke('import_key', { path: selected, name, password });
                    updateStatus('Key imported successfully', 'success');
                    closeModal();
                    await refreshKeyList();
                } catch (error) {
                    updateStatus(`Error importing key: ${error}`, 'error');
                }
            });
        }
    } catch (error) {
        updateStatus(`Error opening file dialog: ${error}`, 'error');
    }
}
