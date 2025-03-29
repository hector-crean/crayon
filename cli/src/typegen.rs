use std::path::Path;
use ts_rs::{ExportError, TS};
use crayon_app::{event::{CrayonInEvent, CrayonOutEvent},};

pub fn generate_typescript_bindings<P: AsRef<Path> + Clone>(output_path: P) -> Result<(), ExportError> {

    CrayonInEvent::export_all_to(output_path.clone())?;
    CrayonOutEvent::export_all_to(output_path.clone())?;

    

    Ok(())
}