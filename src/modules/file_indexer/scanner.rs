//! File scanner implementation for multithreaded file indexing
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

/// Represents a file hash and its paths
pub type FileHash = String;
pub type FilePath = PathBuf;
pub type FileIndex = HashMap<FileHash, Vec<FilePath>>;

/// Scan result containing duplicates and statistics
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub duplicates: FileIndex,
    pub total_files: usize,
    pub time_taken: String,
}

/// Configuration for the file scanner
#[derive(Debug, Clone)]
pub struct ScannerConfig {
    pub num_threads: usize,
    pub chunk_size: usize,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            num_threads: 4,
            chunk_size: 10,
        }
    }
}

/// Scan a directory for duplicate files using multithreading
pub fn scan_directory(dir_path: &Path, config: ScannerConfig) -> Result<ScanResult, String> {
    let start_time = Instant::now();
    
    // Collect all files recursively
    let files = collect_files(dir_path).map_err(|e| e.to_string())?;
    let total_files = files.len();
    
    if files.is_empty() {
        return Ok(ScanResult {
            duplicates: HashMap::new(),
            total_files: 0,
            time_taken: "0ms".to_string(),
        });
    }
    
    // Split files into chunks for threading
    let chunks: Vec<Vec<FilePath>> = files
        .chunks((files.len() + config.num_threads - 1) / config.num_threads)
        .map(|chunk| chunk.to_vec())
        .collect();
    
    // Shared index protected by mutex
    let index = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];
    
    // Spawn worker threads
    for chunk in chunks {
        let index = Arc::clone(&index);
        let handle = thread::spawn(move || {
            process_file_chunk(chunk, index);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Extract duplicates from the index
    let index = Arc::try_unwrap(index).unwrap().into_inner().unwrap();
    let duplicates = find_duplicates(index);
    
    let duration = start_time.elapsed();
    let time_taken = if duration.as_millis() < 1000 {
        format!("{}ms", duration.as_millis())
    } else {
        format!("{:.1}s", duration.as_secs_f32())
    };
    
    Ok(ScanResult {
        duplicates,
        total_files,
        time_taken,
    })
}

/// Collect all files recursively from a directory
fn collect_files(dir_path: &Path) -> Result<Vec<FilePath>, std::io::Error> {
    let mut files = Vec::new();
    
    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                files.push(path);
            } else if path.is_dir() {
                files.extend(collect_files(&path)?);
            }
        }
    }
    
    Ok(files)
}

/// Process a chunk of files in a worker thread
fn process_file_chunk(chunk: Vec<FilePath>, index: Arc<Mutex<FileIndex>>) {
    for file_path in chunk {
        if let Ok(hash) = calculate_file_hash(&file_path) {
            let mut index = index.lock().unwrap();
            index.entry(hash).or_insert_with(Vec::new).push(file_path);
        }
    }
}

/// Calculate SHA256 hash of a file
fn calculate_file_hash(file_path: &Path) -> Result<FileHash, std::io::Error> {
    let mut file = fs::File::open(file_path)?;
    let mut hasher = Sha256::new();
    
    // Read file in chunks to handle large files efficiently
    let mut buffer = [0; 8192];
    loop {
        let bytes_read = std::io::Read::read(&mut file, &mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Find duplicate files from the index
fn find_duplicates(index: FileIndex) -> FileIndex {
    index
        .into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .collect()
}