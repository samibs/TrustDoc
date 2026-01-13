mod commands;
mod utils;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
enum WorkflowCommand {
    /// Create a new signing workflow
    Create {
        /// TDF document file
        document: PathBuf,
        /// Output workflow JSON file
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Signing order: unordered, ordered, or simultaneous
        #[arg(long, default_value = "unordered")]
        order: String,
        /// Required signer IDs (comma-separated)
        #[arg(long)]
        signers: String,
    },
    /// Show workflow status
    Status {
        /// Workflow JSON file
        workflow: PathBuf,
    },
}

#[derive(Parser)]
#[command(name = "tdf")]
#[command(about = "TDF (TrustDoc Financial) format tool")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new TDF document from JSON input
    Create {
        /// Input JSON file
        input: PathBuf,
        /// Output TDF file
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Signer ID (DID format)
        #[arg(long)]
        signer_id: Option<String>,
        /// Signer name
        #[arg(long)]
        signer_name: Option<String>,
        /// Path to signing key file (Ed25519 private key)
        #[arg(long)]
        key: Option<PathBuf>,
        /// Use manual timestamp (local system time)
        #[arg(long)]
        timestamp_manual: bool,
    },
    /// Verify integrity and signatures of a TDF document
    Verify {
        /// TDF file to verify
        document: PathBuf,
        /// Path to verifying key file (Ed25519 public key)
        #[arg(short, long)]
        key: Option<PathBuf>,
        /// Security tier: micro, standard, extended, permissive
        #[arg(long, default_value = "standard")]
        security_tier: String,
        /// Path to revocation list file (CBOR)
        #[arg(long)]
        revocation_list: Option<PathBuf>,
        /// Path to trusted signers whitelist file (JSON)
        #[arg(long)]
        trusted_signers: Option<PathBuf>,
        /// Allow unsigned documents (skip signature requirement)
        #[arg(long)]
        allow_unsigned: bool,
        /// Lenient mode: allow warnings without failing (default is strict)
        #[arg(long)]
        lenient: bool,
        /// Enforce whitelist: fail if signer not in whitelist
        #[arg(long)]
        enforce_whitelist: bool,
        /// Skip revocation checking
        #[arg(long)]
        skip_revocation: bool,
    },
    /// Extract structured data from a TDF document
    Extract {
        /// TDF file to extract from
        document: PathBuf,
        /// Output JSON file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Show metadata and signature information
    Info {
        /// TDF file
        document: PathBuf,
    },
    /// Export TDF document to PDF
    Export {
        /// TDF file to export
        document: PathBuf,
        /// Output PDF file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Multi-party signing workflow
    Workflow {
        /// Create a new signing workflow
        #[command(subcommand)]
        workflow_cmd: Option<WorkflowCommand>,
    },
    /// Generate a new Ed25519 keypair for signing
    Keygen {
        /// Output directory for keys (default: current directory)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Key name prefix (default: "tdf-key")
        #[arg(short, long, default_value = "tdf-key")]
        name: String,
        /// Use secp256k1 algorithm (Web3 compatible) instead of Ed25519
        #[arg(long)]
        secp256k1: bool,
    },
    /// Import file(s) and convert to TDF (supports CSV, XLSX, DOCX, PPTX, TXT, MD, PDF)
    Import {
        /// Input file or folder containing files to convert
        input: PathBuf,
        /// Output TDF file or folder
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Signer ID (DID format)
        #[arg(long)]
        signer_id: Option<String>,
        /// Signer name
        #[arg(long)]
        signer_name: Option<String>,
        /// Path to signing key file (Ed25519 private key)
        #[arg(long)]
        key: Option<PathBuf>,
        /// Batch mode: convert all supported files in folder
        #[arg(long)]
        batch: bool,
    },
    /// Revoke a signing key
    Revoke {
        /// Signer ID (DID) to revoke
        #[arg(long)]
        key_id: String,
        /// Reason for revocation (key-compromise, ca-compromise, superseded, etc.)
        #[arg(long, default_value = "key-compromise")]
        reason: String,
        /// Authority issuing revocation
        #[arg(long)]
        authority: Option<String>,
        /// Output revocation list file (CBOR format)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Check revocation status of keys in a TDF document
    CheckRevocation {
        /// TDF file to check
        document: PathBuf,
        /// External revocation list file (optional)
        #[arg(short, long)]
        revocation_list: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Create {
            input,
            output,
            signer_id,
            signer_name,
            key,
            timestamp_manual,
        } => commands::create::create_document(input, output, signer_id, signer_name, key, timestamp_manual),
        Commands::Verify {
            document,
            key,
            security_tier,
            revocation_list,
            trusted_signers,
            allow_unsigned,
            lenient,
            enforce_whitelist,
            skip_revocation,
        } => {
            commands::verify::verify_document(
                document,
                key,
                security_tier,
                revocation_list,
                trusted_signers,
                allow_unsigned,
                lenient,
                enforce_whitelist,
                skip_revocation,
            )
        }
        Commands::Extract { document, output } => commands::extract::extract_data(document, output),
        Commands::Info { document } => commands::info::show_info(document),
        Commands::Export { document, output } => commands::export::export_to_pdf(document, output),
        Commands::Workflow { workflow_cmd } => {
            match workflow_cmd {
                Some(WorkflowCommand::Create { document, output, order, signers }) => {
                    commands::workflow::create_workflow(document, output, order, signers)
                }
                Some(WorkflowCommand::Status { workflow }) => {
                    commands::workflow::show_workflow_status(workflow)
                }
                None => {
                    eprintln!("Workflow command required. Use 'tdf workflow --help' for options.");
                    Ok(())
                }
            }
        },
        Commands::Keygen { output, name, secp256k1 } => {
            if secp256k1 {
                commands::keygen::generate_keypair_secp256k1(output, name)
            } else {
                commands::keygen::generate_keypair_ed25519(output, name)
            }
        },
        Commands::Import { input, output, signer_id, signer_name, key, batch } => {
            if batch || input.is_dir() {
                commands::import::import_batch_from_folder(input, output, signer_id, signer_name, key)
            } else {
                commands::import::import_from_file(input, output, signer_id, signer_name, key)
            }
        },
        Commands::Revoke { key_id, reason, authority, output } => {
            commands::revoke::revoke_key(key_id, reason, authority, output)
        },
        Commands::CheckRevocation { document, revocation_list } => {
            commands::revoke::check_revocation(document, revocation_list)
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

