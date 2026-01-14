// Audit Logging Service
// Logs all user actions with timestamps for compliance and audit purposes

export interface AuditLogEntry {
    id: string;
    timestamp: string;
    action: string;
    details: Record<string, any>;
    documentId?: string;
    documentName?: string;
    userId?: string;
    sessionId: string;
}

class AuditLogService {
    private storageKey = 'tdf-viewer-audit-log';
    private maxEntries = 1000; // Keep last 1000 entries
    private sessionId: string;

    constructor() {
        this.sessionId = this.generateSessionId();
    }

    private generateSessionId(): string {
        return `session-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    }

    private generateEntryId(): string {
        return `entry-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    }

    log(action: string, details: Record<string, any> = {}): void {
        const entry: AuditLogEntry = {
            id: this.generateEntryId(),
            timestamp: new Date().toISOString(),
            action,
            details,
            sessionId: this.sessionId
        };

        const logs = this.getLogs();
        logs.push(entry);

        // Keep only last maxEntries
        if (logs.length > this.maxEntries) {
            logs.splice(0, logs.length - this.maxEntries);
        }

        this.saveLogs(logs);

        // Also log to console in development
        if ((import.meta as any).env?.DEV) {
            console.log('[Audit Log]', entry);
        }
    }

    logDocumentOpen(documentName: string, documentId?: string): void {
        this.log('document_open', {
            documentName,
            documentId
        });
    }

    logDocumentVerify(documentName: string, isValid: boolean, documentId?: string): void {
        this.log('document_verify', {
            documentName,
            documentId,
            isValid,
            verificationResult: isValid ? 'valid' : 'invalid'
        });
    }

    logDataExtraction(documentName: string, extractedItems: number, documentId?: string): void {
        this.log('data_extraction', {
            documentName,
            documentId,
            extractedItems
        });
    }

    logDocumentPrint(documentName: string, documentId?: string): void {
        this.log('document_print', {
            documentName,
            documentId
        });
    }

    logError(action: string, error: Error, documentName?: string, documentId?: string): void {
        this.log('error', {
            action,
            errorMessage: error.message,
            errorStack: error.stack,
            documentName,
            documentId
        });
    }

    getLogs(): AuditLogEntry[] {
        try {
            const stored = localStorage.getItem(this.storageKey);
            if (!stored) return [];
            return JSON.parse(stored);
        } catch (error) {
            console.error('Error reading audit logs:', error);
            return [];
        }
    }

    private saveLogs(logs: AuditLogEntry[]): void {
        try {
            localStorage.setItem(this.storageKey, JSON.stringify(logs));
        } catch (error) {
            console.error('Error saving audit logs:', error);
            // If storage is full, try to clear old entries
            if (error instanceof DOMException && error.code === 22) {
                const reducedLogs = logs.slice(-500); // Keep only last 500
                try {
                    localStorage.setItem(this.storageKey, JSON.stringify(reducedLogs));
                } catch (e) {
                    console.error('Failed to save reduced audit logs:', e);
                }
            }
        }
    }

    getLogsByAction(action: string): AuditLogEntry[] {
        return this.getLogs().filter(entry => entry.action === action);
    }

    getLogsByDocument(documentId: string): AuditLogEntry[] {
        return this.getLogs().filter(entry => entry.documentId === documentId);
    }

    getLogsBySession(sessionId: string): AuditLogEntry[] {
        return this.getLogs().filter(entry => entry.sessionId === sessionId);
    }

    getCurrentSessionLogs(): AuditLogEntry[] {
        return this.getLogsBySession(this.sessionId);
    }

    exportAsJSON(): string {
        return JSON.stringify(this.getLogs(), null, 2);
    }

    exportAsCSV(): string {
        const logs = this.getLogs();
        if (logs.length === 0) return '';

        // CSV Header
        const headers = ['ID', 'Timestamp', 'Action', 'Document Name', 'Document ID', 'Session ID', 'Details'];
        const rows = logs.map(entry => [
            entry.id,
            entry.timestamp,
            entry.action,
            entry.documentName || '',
            entry.documentId || '',
            entry.sessionId,
            JSON.stringify(entry.details)
        ]);

        const csvContent = [
            headers.join(','),
            ...rows.map(row => row.map(cell => `"${String(cell).replace(/"/g, '""')}"`).join(','))
        ].join('\n');

        return csvContent;
    }

    downloadJSON(filename: string = `audit-log-${new Date().toISOString().split('T')[0]}.json`): void {
        const json = this.exportAsJSON();
        const blob = new Blob([json], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = filename;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
    }

    downloadCSV(filename: string = `audit-log-${new Date().toISOString().split('T')[0]}.csv`): void {
        const csv = this.exportAsCSV();
        const blob = new Blob([csv], { type: 'text/csv' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = filename;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
    }

    clearLogs(): void {
        localStorage.removeItem(this.storageKey);
    }

    getStats(): {
        totalEntries: number;
        entriesByAction: Record<string, number>;
        entriesBySession: number;
        oldestEntry: string | null;
        newestEntry: string | null;
    } {
        const logs = this.getLogs();
        const entriesByAction: Record<string, number> = {};
        
        logs.forEach(entry => {
            entriesByAction[entry.action] = (entriesByAction[entry.action] || 0) + 1;
        });

        return {
            totalEntries: logs.length,
            entriesByAction,
            entriesBySession: new Set(logs.map(e => e.sessionId)).size,
            oldestEntry: logs.length > 0 ? logs[0].timestamp : null,
            newestEntry: logs.length > 0 ? logs[logs.length - 1].timestamp : null
        };
    }
}

// Export singleton instance
export const auditLog = new AuditLogService();
