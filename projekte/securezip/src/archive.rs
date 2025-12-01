use crate::error::SecureZipError;
use std::path::PathBuf;
use tar::Builder;

// [David]: Implement archive creation logic.
// Function signature: `create_archive(files: Vec<PathBuf>) -> Result<Vec<u8>>`
// Use `tar::Builder`.
// Iterate over files and add them to the archive.
pub fn create_archive(files: Vec<PathBuf>) -> Result<Vec<u8>, SecureZipError> {
    let mut archive_data = Vec::new();
    {
        let mut builder = Builder::new(&mut archive_data);

        for file in files {
            if file.is_file() {
                // Use only the file name for the archive entry
                let file_name = file
                    .file_name()
                    .ok_or_else(|| SecureZipError::Archive("Invalid file name".to_string()))?;
                builder
                    .append_path_with_name(&file, file_name)
                    .map_err(|e| SecureZipError::Archive(e.to_string()))?;
            } else if file.is_dir() {
                builder
                    .append_dir_all(file.file_name().unwrap_or(file.as_ref()), &file)
                    .map_err(|e| SecureZipError::Archive(e.to_string()))?;
            }
        }

        builder
            .finish()
            .map_err(|e| SecureZipError::Archive(e.to_string()))?;
    }

    Ok(archive_data)
}

// [David]: Implement archive extraction logic.
// Function signature: `extract_archive(data: &[u8], dest: PathBuf) -> Result<()>`
// Use `tar::Archive`.
// Unpack the archive to the destination directory.
pub fn extract_archive(data: &[u8], dest: PathBuf) -> Result<(), SecureZipError> {
    let mut archive = tar::Archive::new(data);
    archive
        .unpack(&dest)
        .map_err(|e| SecureZipError::Archive(e.to_string()))?;
    Ok(())
}
