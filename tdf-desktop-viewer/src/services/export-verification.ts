// Export Verification Report Service
// Exports verification results as JSON and PDF for audit purposes

export interface VerificationReport {
    timestamp: string;
    document: {
        id: string;
        title: string;
        created?: string;
        modified?: string;
    };
    verification: {
        integrity: {
            valid: boolean;
            rootHash: string;
            algorithm: string;
        };
        signatures: {
            present: boolean;
            count: number;
        };
    };
    metadata: {
        authors: Array<{ name: string; role?: string }>;
        sections: number;
        language?: string;
    };
}

export function generateVerificationReport(
    document: any,
    isValid: boolean,
    hasSignatures: boolean
): VerificationReport {
    const manifest = document.manifest || {};
    const documentMeta = manifest.document || {};
    const integrity = manifest.integrity || {};
    const authors = manifest.authors || [];

    return {
        timestamp: new Date().toISOString(),
        document: {
            id: documentMeta.id || 'N/A',
            title: documentMeta.title || 'Untitled',
            created: documentMeta.created,
            modified: documentMeta.modified
        },
        verification: {
            integrity: {
                valid: isValid,
                rootHash: integrity.root_hash || 'N/A',
                algorithm: integrity.algorithm || 'N/A'
            },
            signatures: {
                present: hasSignatures,
                count: hasSignatures ? 1 : 0
            }
        },
        metadata: {
            authors: authors.map((a: any) => ({
                name: a.name || a.id,
                role: a.role
            })),
            sections: document.content?.sections?.length || 0,
            language: documentMeta.language
        }
    };
}

export function exportVerificationAsJSON(report: VerificationReport, filename?: string): void {
    const json = JSON.stringify(report, null, 2);
    const blob = new Blob([json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename || `verification-report-${new Date().toISOString().split('T')[0]}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
}

export async function exportVerificationAsPDF(report: VerificationReport, filename?: string): Promise<void> {
    // For PDF generation, we'll create an HTML document and use browser print
    // In a production environment, you might want to use a library like jsPDF or pdfkit
    
    const html = `
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Verification Report</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            padding: 40px;
            max-width: 800px;
            margin: 0 auto;
        }
        h1 {
            color: #0066cc;
            border-bottom: 2px solid #0066cc;
            padding-bottom: 10px;
        }
        h2 {
            color: #333;
            margin-top: 30px;
        }
        .section {
            margin: 20px 0;
            padding: 15px;
            background: #f8f9fa;
            border-radius: 5px;
        }
        .field {
            margin: 10px 0;
        }
        .label {
            font-weight: bold;
            color: #666;
        }
        .value {
            color: #333;
        }
        .status {
            display: inline-block;
            padding: 5px 15px;
            border-radius: 20px;
            font-weight: bold;
            margin: 5px 0;
        }
        .status.valid {
            background: #d4edda;
            color: #155724;
        }
        .status.invalid {
            background: #f8d7da;
            color: #721c24;
        }
        .footer {
            margin-top: 40px;
            padding-top: 20px;
            border-top: 1px solid #ddd;
            font-size: 12px;
            color: #666;
            text-align: center;
        }
    </style>
</head>
<body>
    <h1>Document Verification Report</h1>
    
    <div class="section">
        <h2>Document Information</h2>
        <div class="field">
            <span class="label">Title:</span>
            <span class="value">${report.document.title}</span>
        </div>
        <div class="field">
            <span class="label">Document ID:</span>
            <span class="value">${report.document.id}</span>
        </div>
        ${report.document.created ? `
        <div class="field">
            <span class="label">Created:</span>
            <span class="value">${new Date(report.document.created).toLocaleString()}</span>
        </div>
        ` : ''}
        ${report.document.modified ? `
        <div class="field">
            <span class="label">Modified:</span>
            <span class="value">${new Date(report.document.modified).toLocaleString()}</span>
        </div>
        ` : ''}
    </div>

    <div class="section">
        <h2>Verification Results</h2>
        <div class="field">
            <span class="label">Integrity Status:</span>
            <span class="status ${report.verification.integrity.valid ? 'valid' : 'invalid'}">
                ${report.verification.integrity.valid ? 'VALID' : 'INVALID'}
            </span>
        </div>
        <div class="field">
            <span class="label">Root Hash:</span>
            <span class="value" style="font-family: monospace; font-size: 12px;">${report.verification.integrity.rootHash}</span>
        </div>
        <div class="field">
            <span class="label">Algorithm:</span>
            <span class="value">${report.verification.integrity.algorithm}</span>
        </div>
        <div class="field">
            <span class="label">Signatures:</span>
            <span class="status ${report.verification.signatures.present ? 'valid' : 'invalid'}">
                ${report.verification.signatures.present ? 'PRESENT' : 'NOT PRESENT'}
            </span>
            <span class="value"> (${report.verification.signatures.count} signature(s))</span>
        </div>
    </div>

    <div class="section">
        <h2>Metadata</h2>
        <div class="field">
            <span class="label">Authors:</span>
            <span class="value">${report.metadata.authors.map(a => a.name).join(', ') || 'N/A'}</span>
        </div>
        <div class="field">
            <span class="label">Content Sections:</span>
            <span class="value">${report.metadata.sections}</span>
        </div>
        ${report.metadata.language ? `
        <div class="field">
            <span class="label">Language:</span>
            <span class="value">${report.metadata.language}</span>
        </div>
        ` : ''}
    </div>

    <div class="footer">
        <p>Report generated: ${new Date(report.timestamp).toLocaleString()}</p>
        <p>TDF Desktop Viewer - TrustDoc Financial Document Viewer</p>
    </div>
</body>
</html>
    `;

    // Open in new window and trigger print
    const printWindow = window.open('', '_blank');
    if (printWindow) {
        printWindow.document.write(html);
        printWindow.document.close();
        
        // Wait for content to load, then print
        printWindow.onload = () => {
            setTimeout(() => {
                printWindow.print();
            }, 250);
        };
    }
}
