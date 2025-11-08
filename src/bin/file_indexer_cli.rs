//! CLI tool for multithreaded file indexing
use rust_axum_project::modules::file_indexer::service::FileIndexerService;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_path> [num_threads]", args[0]);
        eprintln!("Example: {} /home/uploads 4", args[0]);
        std::process::exit(1);
    }
    
    let dir_path = &args[1];
    let num_threads = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(4);
    
    println!("Scanning directory: {}", dir_path);
    println!("Using {} threads", num_threads);
    
    match FileIndexerService::scan_directory_sync(dir_path, Some(num_threads)) {
        Ok(result) => {
            println!("\nScan completed in {}", result.time_taken);
            println!("Total files processed: {}", result.total_files);
            
            if result.duplicates.is_empty() {
                println!("\nNo duplicate files found.");
            } else {
                println!("\nDuplicate files found:");
                for (hash, paths) in &result.duplicates {
                    println!(" - Hash {}: {} duplicates", &hash[..12], paths.len());
                    for path in paths {
                        println!("   → {}", path.display());
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error scanning directory: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}