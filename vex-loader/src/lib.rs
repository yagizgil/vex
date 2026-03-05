use vex_core::smap;
use std::fs;
use std::path::Path;

/// Loader is responsible for reading files from the disk and adding them to the SourceMap.
pub struct Loader;

impl Loader {
    /// Read a file from the given path and register it in the global SourceMap.
    /// Returns the unique ID (usize) of the file if successful.
    pub fn load_file<P: AsRef<Path>>(path: P) -> std::io::Result<usize> {
        let path_ref = path.as_ref();
        
        // Read file content as a string
        let content = fs::read_to_string(path_ref)?;
        let path_str = path_ref.to_string_lossy().to_string();
        
        // Register the file in SourceMap to track it during compilation
        let file_id = {
            let mut map = smap!(write);
            map.add_file(path_str, content)
        };
        
        Ok(file_id)
    }

    /// Retrieve a file from the SourceMap by its ID.
    pub fn get_file(file_id: usize) -> vex_core::source::SourceFile {
        smap!().get_file(file_id)
            .cloned()
            .expect("Loader: File not found in SourceMap")
    }

    /// A shortcut to just get the string content of a file.
    pub fn get_content(file_id: usize) -> String {
        Self::get_file(file_id).content
    }
}
