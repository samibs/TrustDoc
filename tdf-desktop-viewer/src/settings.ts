// Settings Panel

import { updateStatus } from './app';

export async function initSettings() {
    renderSettings();
}

function renderSettings() {
    const container = document.getElementById('settings-content');
    if (!container) return;

    container.innerHTML = `
        <div class="card">
            <div class="card-header">
                <h3 class="card-title">Security Settings</h3>
            </div>
            <div class="settings-content">
                <div class="form-group">
                    <label class="form-label">Default Security Tier</label>
                    <select id="securityTier" class="form-select">
                        <option value="micro">Micro (256 KB)</option>
                        <option value="standard" selected>Standard (5 MB)</option>
                        <option value="extended">Extended (50 MB)</option>
                    </select>
                    <div class="form-help">Maximum document size for verification</div>
                </div>
                
                <div class="form-group">
                    <label class="form-label">Key Storage Location</label>
                    <input type="text" id="keyStoragePath" class="form-input" value="~/.tdf/keys" readonly>
                    <div class="form-help">Keys are stored in your home directory</div>
                </div>
                
                <div class="form-group">
                    <label>
                        <input type="checkbox" id="rejectLegacyMerkle" checked>
                        Reject Legacy Merkle Trees
                    </label>
                    <div class="form-help">Reject documents using legacy Merkle tree format (v1)</div>
                </div>
                
                <div class="form-group">
                    <label>
                        <input type="checkbox" id="rejectLegacySignatures" checked>
                        Reject Legacy Signatures
                    </label>
                    <div class="form-help">Reject documents with legacy signature format (v1)</div>
                </div>
                
                <div class="form-group">
                    <label>
                        <input type="checkbox" id="requireRfc3161Timestamps">
                        Require RFC 3161 Timestamps
                    </label>
                    <div class="form-help">Require cryptographic timestamp proofs (optional)</div>
                </div>
            </div>
        </div>
        
        <div class="card">
            <div class="card-header">
                <h3 class="card-title">Appearance</h3>
            </div>
            <div class="settings-content">
                <div class="form-group">
                    <label class="form-label">Theme</label>
                    <select id="theme" class="form-select">
                        <option value="light" selected>Light</option>
                        <option value="dark">Dark</option>
                        <option value="auto">Auto (System)</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label class="form-label">Font Size</label>
                    <select id="fontSize" class="form-select">
                        <option value="small">Small</option>
                        <option value="medium" selected>Medium</option>
                        <option value="large">Large</option>
                    </select>
                </div>
            </div>
        </div>
        
        <div class="card">
            <div class="card-header">
                <h3 class="card-title">Advanced</h3>
            </div>
            <div class="settings-content">
                <div class="form-group">
                    <label>
                        <input type="checkbox" id="debugMode">
                        Debug Mode
                    </label>
                    <div class="form-help">Show detailed error messages and debug information</div>
                </div>
                
                <div class="form-group">
                    <label class="form-label">Log Level</label>
                    <select id="logLevel" class="form-select">
                        <option value="error">Error</option>
                        <option value="warn">Warning</option>
                        <option value="info" selected>Info</option>
                        <option value="debug">Debug</option>
                    </select>
                </div>
            </div>
        </div>
        
        <div class="card">
            <div class="card-header">
                <h3 class="card-title">About</h3>
            </div>
            <div class="settings-content">
                <div class="result-item">
                    <strong>Version:</strong>
                    <span>0.1.0</span>
                </div>
                <div class="result-item">
                    <strong>License:</strong>
                    <span>MIT OR Apache-2.0</span>
                </div>
                <div class="result-item">
                    <strong>Repository:</strong>
                    <span><a href="https://github.com/trustdoc/tdf" target="_blank">GitHub</a></span>
                </div>
            </div>
        </div>
    `;

    // Load saved settings
    loadSettings();
    
    // Save settings on change
    const inputs = container.querySelectorAll('input, select');
    inputs.forEach(input => {
        input.addEventListener('change', () => {
            saveSettings();
        });
    });
}

function loadSettings() {
    const saved = localStorage.getItem('tdf-settings');
    if (!saved) return;

    try {
        const settings = JSON.parse(saved);
        
        const securityTier = document.getElementById('securityTier') as HTMLSelectElement;
        if (securityTier && settings.securityTier) {
            securityTier.value = settings.securityTier;
        }
        
        const rejectLegacyMerkle = document.getElementById('rejectLegacyMerkle') as HTMLInputElement;
        if (rejectLegacyMerkle) {
            rejectLegacyMerkle.checked = settings.rejectLegacyMerkle !== false;
        }
        
        const rejectLegacySignatures = document.getElementById('rejectLegacySignatures') as HTMLInputElement;
        if (rejectLegacySignatures) {
            rejectLegacySignatures.checked = settings.rejectLegacySignatures !== false;
        }
        
        const requireRfc3161Timestamps = document.getElementById('requireRfc3161Timestamps') as HTMLInputElement;
        if (requireRfc3161Timestamps) {
            requireRfc3161Timestamps.checked = settings.requireRfc3161Timestamps === true;
        }
        
        const theme = document.getElementById('theme') as HTMLSelectElement;
        if (theme && settings.theme) {
            theme.value = settings.theme;
            applyTheme(settings.theme);
        }
        
        const fontSize = document.getElementById('fontSize') as HTMLSelectElement;
        if (fontSize && settings.fontSize) {
            fontSize.value = settings.fontSize;
            applyFontSize(settings.fontSize);
        }
        
        const debugMode = document.getElementById('debugMode') as HTMLInputElement;
        if (debugMode) {
            debugMode.checked = settings.debugMode === true;
        }
        
        const logLevel = document.getElementById('logLevel') as HTMLSelectElement;
        if (logLevel && settings.logLevel) {
            logLevel.value = settings.logLevel;
        }
    } catch (error) {
        console.error('Error loading settings:', error);
    }
}

function saveSettings() {
    const securityTier = (document.getElementById('securityTier') as HTMLSelectElement)?.value || 'standard';
    const rejectLegacyMerkle = (document.getElementById('rejectLegacyMerkle') as HTMLInputElement)?.checked ?? true;
    const rejectLegacySignatures = (document.getElementById('rejectLegacySignatures') as HTMLInputElement)?.checked ?? true;
    const requireRfc3161Timestamps = (document.getElementById('requireRfc3161Timestamps') as HTMLInputElement)?.checked ?? false;
    const theme = (document.getElementById('theme') as HTMLSelectElement)?.value || 'light';
    const fontSize = (document.getElementById('fontSize') as HTMLSelectElement)?.value || 'medium';
    const debugMode = (document.getElementById('debugMode') as HTMLInputElement)?.checked ?? false;
    const logLevel = (document.getElementById('logLevel') as HTMLSelectElement)?.value || 'info';

    const settings = {
        securityTier,
        rejectLegacyMerkle,
        rejectLegacySignatures,
        requireRfc3161Timestamps,
        theme,
        fontSize,
        debugMode,
        logLevel,
    };

    localStorage.setItem('tdf-settings', JSON.stringify(settings));
    applyTheme(theme);
    applyFontSize(fontSize);
    updateStatus('Settings saved', 'success');
}

function applyTheme(theme: string) {
    document.body.className = `theme-${theme}`;
}

function applyFontSize(size: string) {
    document.documentElement.style.fontSize = size === 'small' ? '14px' : size === 'large' ? '18px' : '16px';
}
