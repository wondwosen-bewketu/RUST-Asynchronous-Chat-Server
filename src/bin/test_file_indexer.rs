//! Simple test to verify the file indexer is working
use rust_axum_project::modules::file_indexer::service::FileIndexerService;
use std::fs;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing file indexer components...");
    
    // Create a temporary directory for testing
    let test_dir = std::env::temp_dir().join("file_indexer_test");
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir)?;
    }
    fs::create_dir_all(&test_dir)?;
    
    // Create some test files
    let file1_path = test_dir.join("file1.txt");
    let file2_path = test_dir.join("file2.txt");
    let file3_path = test_dir.join("file3.txt");
    
    // Write identical content to file1 and file2 (they should be duplicates)
    let mut file1 = fs::File::create(&file1_path)?;
    file1.write_all(b"Hello, world!")?;
    
    let mut file2 = fs::File::create(&file2_path)?;
    file2.write_all(b"Hello, world!")?;
    
    // Write different content to file3
    let mut file3 = fs::File::create(&file3_path)?;
    file3.write_all(b"Goodbye, world!")?;
    
    // Test synchronous scanning
    println!("Testing synchronous scanning...");
    let result = FileIndexerService::scan_directory_sync(
        test_dir.to_str().unwrap(), 
        Some(2)
    )?;
    
    println!("✓ Scan completed in {}", result.time_taken);
    println!("✓ Total files processed: {}", result.total_files);
    println!("✓ Duplicates found: {}", result.duplicates.len());
    
    // Verify we found the expected duplicate
    assert_eq!(result.total_files, 3);
    assert_eq!(result.duplicates.len(), 1);
    
    // Clean up
    fs::remove_dir_all(&test_dir)?;
    
    println!("\nAll tests passed! The file indexer is working correctly.");
    println!("\nTo test the full functionality:");
    println!("1. Start the server: cargo run");
    println!("2. Register and login to get a JWT token");
    println!("3. Call the API: POST /api/files/scan with your token");
    println!("4. Or use the CLI: cargo run --bin file_indexer_cli /path/to/directory");
    
    Ok(())
}