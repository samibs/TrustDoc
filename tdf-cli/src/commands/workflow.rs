use std::fs;
use std::path::PathBuf;
use tdf_core::archive::ArchiveReader;
use tdf_core::error::TdfResult;
use tdf_core::multiparty::{SigningOrder, SigningWorkflow, SignerRequirement};

pub fn create_workflow(
    document: PathBuf,
    output: Option<PathBuf>,
    order: String,
    signers: String,
) -> TdfResult<()> {
    // Read document to get ID
    let (doc, _, _) = ArchiveReader::read(&document)?;

    // Parse signing order
    let signing_order = match order.as_str() {
        "unordered" => SigningOrder::Unordered,
        "ordered" => {
            // For ordered, use the signers list as the order
            let signer_list: Vec<String> = signers.split(',').map(|s| s.trim().to_string()).collect();
            SigningOrder::Ordered(signer_list.clone())
        }
        "simultaneous" => SigningOrder::Simultaneous,
        _ => {
            return Err(tdf_core::error::TdfError::InvalidDocument(
                "Order must be: unordered, ordered, or simultaneous".to_string(),
            ));
        }
    };

    // Parse signers
    let signer_ids: Vec<String> = if order == "ordered" {
        // Already parsed above
        match &signing_order {
            SigningOrder::Ordered(ids) => ids.clone(),
            _ => signers.split(',').map(|s| s.trim().to_string()).collect(),
        }
    } else {
        signers.split(',').map(|s| s.trim().to_string()).collect()
    };

    let required_signers: Vec<SignerRequirement> = signer_ids
        .iter()
        .map(|id| SignerRequirement {
            signer_id: id.clone(),
            signer_name: id.clone(), // Default to ID, can be customized
            role: None,
            required: true,
        })
        .collect();

    // Create workflow
    let workflow = SigningWorkflow::new(
        doc.manifest.document.id,
        signing_order,
        required_signers,
    );

    // Determine output path
    let output_path = output.unwrap_or_else(|| {
        document
            .with_extension("workflow.json")
            .file_name()
            .map(|n| PathBuf::from(n))
            .unwrap_or_else(|| PathBuf::from("workflow.json"))
    });

    // Save workflow
    let json = serde_json::to_string_pretty(&workflow)?;
    fs::write(&output_path, json)?;

    println!("Created signing workflow: {}", output_path.display());
    println!("  Document ID: {}", workflow.document_id);
    println!("  Order: {:?}", workflow.order);
    println!("  Required signers: {}", workflow.required_signers.len());
    println!("\nNext steps:");
    println!("  1. Share workflow with signers");
    println!("  2. Each signer signs the document");
    println!("  3. Use 'tdf workflow status' to check progress");

    Ok(())
}

pub fn show_workflow_status(workflow: PathBuf) -> TdfResult<()> {
    let json_str = fs::read_to_string(&workflow)?;
    let workflow: SigningWorkflow = serde_json::from_str(&json_str)?;

    println!("Signing Workflow Status");
    println!("======================");
    println!("Workflow ID: {}", workflow.id);
    println!("Document ID: {}", workflow.document_id);
    println!("Order: {:?}", workflow.order);
    println!("Status: {:?}", workflow.status);
    println!("Created: {}", workflow.created);
    if let Some(completed) = workflow.completed {
        println!("Completed: {}", completed);
    }

    println!("\nRequired Signers:");
    for (i, signer) in workflow.required_signers.iter().enumerate() {
        println!("  {}. {} ({})", i + 1, signer.signer_name, signer.signer_id);
        if let Some(ref role) = signer.role {
            println!("     Role: {}", role);
        }
    }

    match workflow.status {
        tdf_core::multiparty::WorkflowStatus::InProgress { signed_count, total } => {
            println!("\nProgress: {}/{} signatures", signed_count, total);
            if let Some(next) = workflow.get_next_signer() {
                println!("Next signer: {} ({})", next.signer_name, next.signer_id);
            }
        }
        tdf_core::multiparty::WorkflowStatus::Completed => {
            println!("\n✓ All signatures complete!");
        }
        tdf_core::multiparty::WorkflowStatus::Rejected { ref reason } => {
            println!("\n✗ Workflow rejected: {}", reason);
        }
        _ => {}
    }

    Ok(())
}

