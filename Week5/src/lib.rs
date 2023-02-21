//Compare parallel and sequential versions of the program
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use walkdir::WalkDir;

pub fn walk(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }
    Ok(files)
}

// Create a checksum of each file and store in a HashMap if the checksum already exists, add the file to the vector of files with that checksum
pub fn checksum(files: Vec<String>) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let mut checksums = HashMap::new();
    for file in files {
        let checksum = md5::compute(std::fs::read(&file)?);
        let checksum = format!("{:x}", checksum);
        checksums
            .entry(checksum)
            .or_insert_with(Vec::new)
            .push(file);
    }
    Ok(checksums)
}

// Parallel version of checksum using rayon with a mutex to ensure
//that the HashMap is not accessed by multiple threads at the same time
pub fn checksum_par(files: Vec<String>) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let checksums = std::sync::Mutex::new(HashMap::new());
    files.par_iter().for_each(|file| {
        let checksum = md5::compute(std::fs::read(file).unwrap());
        let checksum = format!("{:x}", checksum);
        checksums
            .lock()
            .unwrap()
            .entry(checksum)
            .or_insert_with(Vec::new)
            .push(file.to_string());
    });
    Ok(checksums.into_inner().unwrap())
}

// Create a checksum of each file and store in a HashMap if the checksum already exists, add the file to the vector of files with that checksum
pub fn checksum_sha(files: Vec<String>) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let mut checksums = HashMap::new();
    for file in files {
        let input = Path::new(&file);
        let checksum = sha256::try_digest(input).unwrap();
        let checksum = format!("{}", checksum);
        checksums
            .entry(checksum)
            .or_insert_with(Vec::new)
            .push(file);
    }
    Ok(checksums)
}

// Parallel version of checksum using rayon with a mutex to ensure
//that the HashMap is not accessed by multiple threads at the same time
pub fn checksum_par_sha(
    files: Vec<String>,
) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let checksums = std::sync::Mutex::new(HashMap::new());
    files.par_iter().for_each(|file| {
        let input = Path::new(&file);
        let checksum = sha256::try_digest(input).unwrap();
        let checksum = format!("{}", checksum);
        checksums
            .lock()
            .unwrap()
            .entry(checksum)
            .or_insert_with(Vec::new)
            .push(file.to_string());
    });
    Ok(checksums.into_inner().unwrap())
}
