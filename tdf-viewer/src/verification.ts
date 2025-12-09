import { TdfDocument, loadDocument } from 'tdf-ts';

export interface VerificationResult {
  integrityValid: boolean;
  rootHash: string;
  signatureCount: number;
  errors: string[];
  warnings?: string[];
}

// WASM bindings will be loaded dynamically
let wasmModule: any = null;

async function loadWasm(): Promise<any> {
  if (wasmModule) return wasmModule;
  
  try {
    // Load WASM module from src/wasm directory
    // Use dynamic import - Vite will handle the WASM file properly
    const wasm = await import('./wasm/tdf_wasm.js');
    await wasm.default();
    wasmModule = wasm;
    console.log('✅ WASM module loaded successfully - full cryptographic verification enabled');
    return wasm;
  } catch (error) {
    // WASM not available - that's okay, viewer can work without it
    console.warn('⚠️ WASM module not available. Using basic validation only.', error);
    return null;
  }
}

export async function verifyDocument(file: File | Blob): Promise<VerificationResult> {
  const wasm = await loadWasm();
  
  if (wasm) {
    // Use WASM verification (full cryptographic verification)
    try {
      const arrayBuffer = await file.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);
      const result = wasm.verify_document(uint8Array);
      
      // VerificationResult has readonly properties (not methods) per TypeScript definitions
      // Access them directly as properties
      const integrityValid = result.integrity_valid;
      const rootHash = result.root_hash;
      const signatureCount = result.signature_count;
      const errors = Array.isArray(result.errors) 
        ? result.errors.map((e: any) => String(e))
        : [];
      
      return {
        integrityValid,
        rootHash: rootHash || '',
        signatureCount: signatureCount || 0,
        errors,
      };
    } catch (error: any) {
      console.error('WASM verification error:', error);
      return {
        integrityValid: false,
        rootHash: '',
        signatureCount: 0,
        errors: [`WASM verification error: ${error.message}`],
      };
    }
  }
  
  // Fallback: Basic document structure validation using TypeScript SDK
  try {
    const doc = await loadDocument(file);
    
    // Basic validation checks
    const errors: string[] = [];
    const warnings: string[] = [];
    
    // Check document structure
    if (!doc.manifest || !doc.manifest.document) {
      errors.push('Invalid document structure: missing manifest');
    }
    
    if (!doc.manifest.integrity || !doc.manifest.integrity.root_hash) {
      errors.push('Invalid document: missing integrity hash');
    }
    
    // Check content
    if (!doc.content || !doc.content.sections) {
      errors.push('Invalid document: missing content sections');
    }
    
    // Count signatures
    let signatureCount = 0;
    if (doc.signatures && doc.signatures.signatures) {
      signatureCount = doc.signatures.signatures.length;
    }
    
    // Warning about limited verification
    warnings.push('Limited verification (WASM not available). Document structure is valid, but cryptographic integrity not verified.');
    warnings.push('For full verification, build WASM: cd tdf-wasm && wasm-pack build --target web');
    
    return {
      integrityValid: errors.length === 0,
      rootHash: doc.manifest?.integrity?.root_hash || '',
      signatureCount,
      errors,
      warnings,
    };
  } catch (error: any) {
    return {
      integrityValid: false,
      rootHash: '',
      signatureCount: 0,
      errors: [`Document validation error: ${error.message}`],
      warnings: ['WASM verification not available. Only basic structure validation performed.'],
    };
  }
}

