#[macro_use]
mod string_templating;
mod copier;
mod path_remapper;

use self::{copier::CopierError, string_templating::Replacement};
use std::path::Path;

/// Copy a template from `src_path` to `dest_path` and apply the given replacements.
/// Replacements are created with the rep! macro.
/// For example `rep!("foo", "bar")` will replace all occurences of `{{foo}}` with `bar`.
/// This applies to file names (like `{{foo}}.rs`) and file contents.
/// ```no_run
/// let src = "./template";
/// let dst = "./dest";
/// let reps = [rep!("foo", "bar")];
/// copy_template(src, dst, &reps);
/// ```
pub fn copy_template<P: AsRef<Path>>(
    src_path: P,
    dest_path: P,
    replacements: &[Replacement],
) -> Result<(), CopierError> {
    copier::Copier::new(src_path, dest_path, replacements)?.copy()
}
