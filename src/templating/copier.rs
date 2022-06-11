use std::{fs, io, path::Path};

use thiserror::Error;
use walkdir::WalkDir;

use super::{
    path_remapper::PathRemapper,
    string_templating::{Replacement, TemplateError, TemplateProcessing},
};

#[derive(Error, Debug)]
pub enum CopierError {
    #[error(transparent)]
    Template(#[from] TemplateError),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),
}

// Helper struct to copy a template
pub struct Copier<'a> {
    walk: Option<walkdir::WalkDir>,
    remapper: PathRemapper,
    replacements: &'a [Replacement],
}

impl<'a> Copier<'a> {
    pub fn new<P: AsRef<Path>>(
        src_path: P,
        dest_path: P,
        replacements: &'a [Replacement],
    ) -> io::Result<Self> {
        Ok(Self {
            walk: Some(WalkDir::new(&src_path)),
            remapper: PathRemapper {
                src_path: src_path.as_ref().to_path_buf(),
                dest_path: dest_path.as_ref().to_path_buf(),
            },
            replacements,
        })
    }

    pub fn copy(mut self) -> Result<(), CopierError> {
        let walker = self.walk.take().expect("we should have a valid walker");
        for dirent in walker {
            let dirent = dirent?;
            if dirent.file_type().is_dir() {
                self.create_dir(dirent.path())?;
            } else if dirent.file_type().is_file() {
                self.create_file(dirent)?;
            }
        }

        Ok(())
    }

    fn create_dir(&self, path: &Path) -> Result<(), CopierError> {
        let new_path = self.remapper.remap(path);
        let new_path = new_path
            .to_str()
            .expect("did not get a valid dir path")
            .process_template(self.replacements)?;

        Ok(fs::create_dir(new_path)?)
    }

    fn create_file(&self, file: walkdir::DirEntry) -> Result<(), CopierError> {
        let content = fs::read_to_string(file.path())?;
        let new_content = &content.process_template(self.replacements)?;
        let new_path = self.remapper.remap(file.path());
        let new_path = new_path
            .to_str()
            .expect("did not get a valid file path")
            .process_template(self.replacements)?;

        fs::write(new_path, new_content)?;

        Ok(())
    }
}
