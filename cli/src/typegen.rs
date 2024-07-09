use crayon_types::events::DrayonEvent;
use specta::{
    ts::{ExportConfig, ExportError},
    TypeCollection,
};
use std::{fs::File, io::Write, path::Path};

// Import types and modules to generate types from

pub fn generate_typescript_bindings<P: AsRef<Path>>(output_path: P) -> Result<(), ExportError> {
    let code = TypeCollection::default()
        .register::<DrayonEvent>()
        .export_ts(&ExportConfig::default())?;

    let mut file = File::create(output_path)?;

    file.write(code.as_bytes())?;

    Ok(())
}
