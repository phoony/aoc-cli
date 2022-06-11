use std::path::{Path, PathBuf};

pub struct PathRemapper {
    pub src_path: PathBuf,
    pub dest_path: PathBuf,
}

impl PathRemapper {
    pub fn remap(&self, path: &Path) -> std::path::PathBuf {
        // remove common prefix
        let stripped = path
            .strip_prefix(self.src_path.as_path())
            .expect("paths should have a common prefix");

        // build new destination path
        self.dest_path.join(stripped)
    }
}
