use std::collections::HashMap;

use std::sync::{RwLock};
use once_cell::sync::Lazy;

pub static GLOBAL_SOURCEMAP: Lazy<RwLock<SourceMap>> = Lazy::new(|| {
    RwLock::new(SourceMap::new())
});

#[macro_export]
macro_rules! smap {
    () => {
        $crate::source::GLOBAL_SOURCEMAP.read().expect("SourceMap lock poisoned")
    };
    (write) => {
        $crate::source::GLOBAL_SOURCEMAP.write().expect("SourceMap lock poisoned")
    };
}

/// Represents a single source file.
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub id: usize,          // Matches the file_id in the Span
    pub path: String,       // File location (e.g., "src/math/utils.vx")
    pub content: String,    // The full Vex code inside the file
}

/// A registry that keeps all project files in memory.
#[derive(Debug, Clone, Default)]
pub struct SourceMap {
    files: HashMap<usize, SourceFile>,
    path_to_id: HashMap<String, usize>,
    next_id: usize,
}
impl SourceMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new file to the system and gives it an ID.
    pub fn add_file(&mut self, path: String, content: String) -> usize {
        if let Some(&id) = self.path_to_id.get(&path) {
            return id;
        }

        let id = self.next_id;
        let file = SourceFile {
            id,
            path: path.clone(),
            content,
        };
        
        self.files.insert(id, file);
        self.path_to_id.insert(path, id);
        self.next_id += 1;
        
        id
    }

    /// Gets a file by its ID (useful for error messages).
    pub fn get_file(&self, id: usize) -> Option<&SourceFile> {
        self.files.get(&id)
    }
}