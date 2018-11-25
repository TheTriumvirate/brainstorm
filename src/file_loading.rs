use crate::{FieldProvider, GPUFieldProvider, State};
use crate::particles::VectorField;
#[cfg(target_arch = "wasm32")]
use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
pub fn reload_file(state: &State) -> Result<(FieldProvider, GPUFieldProvider), String> {
    use std::{io::Read, fs::File};

    let path = state.file_path.as_ref().ok_or_else(|| "No file path saved.".to_owned())?;
    let ext = path.extension().ok_or_else(|| "No file extension.".to_owned())?;
    let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    
    let mut content = Vec::new();
    file.read_to_end(&mut content).map_err(|e| format!("Failed to read file: {}", e))?;

    create_providers(&ext.to_string_lossy().into_owned(), &content)
}

#[cfg(target_arch = "wasm32")]
pub fn reload_file(_state: &State) -> Result<(FieldProvider, GPUFieldProvider), String> {
    // Get file extension
    let path_str = js!(return getPath();)
        .into_string()
        .ok_or_else(|| "Failed to get path from JS.".to_owned())?;
    let mut path = PathBuf::new();
    path.push(path_str);
    let ext = path.extension().ok_or_else(|| "No file extension.".to_owned())?;

    let content = js!(return getData();)
        .into_string()
        .ok_or_else(|| "Failed to get data from JS.".to_owned())?;
    
    let pos = content.find(",").map(|i| i + 1).unwrap_or(0);
    let b64 = content.split_at(pos).1;
    let data = base64::decode(b64).map_err(|e| format!("Failed to decode base64 content: {}", e))?;
    
    create_providers(&ext.to_string_lossy().into_owned(), &data)
}

fn create_providers(file_ext: &str, data: &[u8]) -> Result<(FieldProvider, GPUFieldProvider), String> {
    let vectorfield: VectorField = match file_ext {
        "bincode" => {
            bincode::deserialize(data)
                .map_err(|e| format!("Failed to parse data: {}", e))
        }
        _ => Err("Unknown file extension".to_owned())
    }?;
    
    let gpu_field_provider = GPUFieldProvider::new(&vectorfield);
    let field_provider = FieldProvider::new(vectorfield);
    
    Ok((field_provider, gpu_field_provider))
}
