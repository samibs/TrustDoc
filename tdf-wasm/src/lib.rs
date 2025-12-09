use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct VerificationResult {
    integrity_valid: bool,
    root_hash: String,
    signature_count: usize,
    errors: Vec<String>,
}

#[wasm_bindgen]
impl VerificationResult {
    #[wasm_bindgen(getter)]
    pub fn integrity_valid(&self) -> bool {
        self.integrity_valid
    }

    #[wasm_bindgen(getter)]
    pub fn root_hash(&self) -> String {
        self.root_hash.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn signature_count(&self) -> usize {
        self.signature_count
    }

    #[wasm_bindgen(getter)]
    pub fn errors(&self) -> js_sys::Array {
        self.errors
            .iter()
            .map(|e| JsValue::from_str(e))
            .collect::<js_sys::Array>()
    }
}

#[wasm_bindgen]
pub struct DocumentInfo {
    title: String,
    id: String,
    created: String,
    modified: String,
    author_count: usize,
    section_count: usize,
}

#[wasm_bindgen]
impl DocumentInfo {
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.title.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn created(&self) -> String {
        self.created.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn modified(&self) -> String {
        self.modified.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn author_count(&self) -> usize {
        self.author_count
    }

    #[wasm_bindgen(getter)]
    pub fn section_count(&self) -> usize {
        self.section_count
    }
}

#[wasm_bindgen]
pub fn verify_document(data: &[u8]) -> Result<VerificationResult, JsValue> {
    console_log!("Verifying document, size: {} bytes", data.len());

    // Use Cursor to read from memory instead of file system
    use std::io::Cursor;
    use zip::ZipArchive;

    let cursor = Cursor::new(data);
    let mut zip = ZipArchive::new(cursor)
        .map_err(|e| JsValue::from_str(&format!("Failed to open ZIP archive: {}", e)))?;

    // Read and verify components
    let mut components = std::collections::HashMap::new();

    // Read manifest
    let manifest_bytes = {
        let mut manifest_file = zip.by_name("manifest.cbor")
            .map_err(|_| JsValue::from_str("Missing manifest.cbor"))?;
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut manifest_file, &mut bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to read manifest: {}", e)))?;
        bytes
    };

    // Parse and prepare manifest for hashing (without root_hash)
    let mut manifest: tdf_core::document::Manifest = serde_cbor::from_slice(&manifest_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse manifest: {}", e)))?;
    manifest.integrity.root_hash = String::new();
    let manifest_bytes_for_hash = serde_cbor::to_vec(&manifest)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize manifest: {}", e)))?;

    // Read content
    let content_bytes = {
        let mut content_file = zip.by_name("content.cbor")
            .map_err(|_| JsValue::from_str("Missing content.cbor"))?;
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut content_file, &mut bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to read content: {}", e)))?;
        bytes
    };

    // Read styles
    let styles_bytes = {
        let mut styles_file = zip.by_name("styles.css")
            .map_err(|_| JsValue::from_str("Missing styles.css"))?;
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut styles_file, &mut bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to read styles: {}", e)))?;
        bytes
    };

    components.insert("manifest".to_string(), manifest_bytes_for_hash);
    components.insert("content".to_string(), content_bytes);
    components.insert("styles".to_string(), styles_bytes);

    // Read optional components
    if let Ok(mut layout_file) = zip.by_name("layout.cbor") {
        let mut layout_bytes = Vec::new();
        let _ = std::io::Read::read_to_end(&mut layout_file, &mut layout_bytes);
        if !layout_bytes.is_empty() {
            components.insert("layout".to_string(), layout_bytes);
        }
    }

    if let Ok(mut data_file) = zip.by_name("data.json") {
        let mut data_bytes = Vec::new();
        let _ = std::io::Read::read_to_end(&mut data_file, &mut data_bytes);
        if !data_bytes.is_empty() {
            components.insert("data".to_string(), data_bytes);
        }
    }

    // Read assets
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)
            .map_err(|e| JsValue::from_str(&format!("Failed to read file {}: {}", i, e)))?;
        let name = file.name().to_string();
        if name.starts_with("assets/") {
            let mut data = Vec::new();
            std::io::Read::read_to_end(&mut file, &mut data)
                .map_err(|e| JsValue::from_str(&format!("Failed to read asset {}: {}", name, e)))?;
            components.insert(format!("asset:{}", name), data);
        }
    }

    // Read Merkle tree
    let hashes_bytes = {
        let mut hashes_file = zip.by_name("hashes.bin")
            .map_err(|_| JsValue::from_str("Missing hashes.bin"))?;
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut hashes_file, &mut bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to read hashes: {}", e)))?;
        bytes
    };
    let merkle_tree = tdf_core::merkle::MerkleTree::from_binary(&hashes_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse Merkle tree: {}", e)))?;

    // Read signatures
    let signatures_bytes = {
        let mut signatures_file = zip.by_name("signatures.cbor")
            .map_err(|_| JsValue::from_str("Missing signatures.cbor"))?;
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut signatures_file, &mut bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to read signatures: {}", e)))?;
        bytes
    };
    let signature_block: tdf_core::signature::SignatureBlock = serde_cbor::from_slice(&signatures_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse signatures: {}", e)))?;

    // Verify Merkle tree
    let integrity_valid = merkle_tree.verify(&components)
        .map_err(|e| JsValue::from_str(&format!("Verification error: {}", e)))?;
    let root_hash = hex::encode(merkle_tree.root_hash());

    Ok(VerificationResult {
        integrity_valid,
        root_hash,
        signature_count: signature_block.signatures.len(),
        errors: if integrity_valid {
            vec![]
        } else {
            vec!["Integrity check failed - document may have been tampered with".to_string()]
        },
    })
}

#[wasm_bindgen]
pub fn get_document_info(data: &[u8]) -> Result<DocumentInfo, JsValue> {
    use std::io::Cursor;
    use zip::ZipArchive;

    let cursor = Cursor::new(data);
    let mut zip = ZipArchive::new(cursor)
        .map_err(|e| JsValue::from_str(&format!("Failed to open ZIP archive: {}", e)))?;

    // Read manifest
    let manifest_bytes = {
        let mut manifest_file = zip.by_name("manifest.cbor")
            .map_err(|_| JsValue::from_str("Missing manifest.cbor"))?;
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut manifest_file, &mut bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to read manifest: {}", e)))?;
        bytes
    };
    let manifest: tdf_core::document::Manifest = serde_cbor::from_slice(&manifest_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse manifest: {}", e)))?;

    // Read content
    let content_bytes = {
        let mut content_file = zip.by_name("content.cbor")
            .map_err(|_| JsValue::from_str("Missing content.cbor"))?;
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut content_file, &mut bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to read content: {}", e)))?;
        bytes
    };
    let content: tdf_core::content::DocumentContent = serde_cbor::from_slice(&content_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse content: {}", e)))?;

    Ok(DocumentInfo {
        title: manifest.document.title,
        id: manifest.document.id,
        created: manifest.document.created.to_rfc3339(),
        modified: manifest.document.modified.to_rfc3339(),
        author_count: manifest.authors.len(),
        section_count: content.sections.len(),
    })
}

#[wasm_bindgen]
pub fn compute_merkle_root(components: js_sys::Map) -> Result<String, JsValue> {
    use tdf_core::merkle::{MerkleTree, HashAlgorithm};
    use std::collections::HashMap;

    let mut map = HashMap::new();
    
    let entries = js_sys::Object::entries(&components);
    for i in 0..entries.length() {
        let entry = entries.get(i).dyn_into::<js_sys::Array>()?;
        let key = entry.get(0).as_string().ok_or_else(|| JsValue::from_str("Invalid key"))?;
        let value = entry.get(1).dyn_into::<js_sys::Uint8Array>()?;
        let bytes: Vec<u8> = value.to_vec();
        map.insert(key, bytes);
    }

    let mut tree = MerkleTree::new(HashAlgorithm::Sha256);
    let root = tree.compute_root(&map)
        .map_err(|e| JsValue::from_str(&format!("Merkle computation error: {}", e)))?;

    Ok(hex::encode(root))
}

