//! File indexer service for Axum integration
use crate::modules::file_indexer::scanner::{scan_directory, ScannerConfig, ScanResult};
use std::path::Path;
use tokio::task;

/// File Indexer Service
pub struct FileIndexerService;

impl FileIndexerService {
    /// Scan a directory for duplicate files asynchronously
    pub async fn scan_directory_async(
        dir_path: &str,
        num_threads: Option<usize>,
    ) -> Result<ScanResult, String> {
        let dir_path = dir_path.to_string();
        let config = ScannerConfig {
            num_threads: num_threads.unwrap_or(4),
            chunk_size: 10,
        };
        
        // Run the CPU-intensive scanning operation in a blocking task
        let result = task::spawn_blocking(move || {
            scan_directory(Path::new(&dir_path), config)
                .map_err(|e| e.to_string())
        }).await;
        
        match result {
            Ok(Ok(scan_result)) => Ok(scan_result),
            Ok(Err(e)) => Err(e),
            Err(e) => Err(format!("Task failed: {}", e)),
        }
    }
    
    /// Scan a directory for duplicate files synchronously (for CLI usage)
    pub fn scan_directory_sync(
        dir_path: &str,
        num_threads: Option<usize>,
    ) -> Result<ScanResult, String> {
        let config = ScannerConfig {
            num_threads: num_threads.unwrap_or(4),
            chunk_size: 10,
        };
        
        scan_directory(Path::new(dir_path), config)
            .map_err(|e| e.to_string())
    }
}