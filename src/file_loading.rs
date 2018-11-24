use crate::{FieldProvider, GPUFieldProvider, State};

#[cfg(not(target_arch = "wasm32"))]
pub fn reload_file(state: &State) -> Result<(FieldProvider, GPUFieldProvider), String> {
    use std::{io::Read, fs::File};

    let path = state.file_path.as_ref().ok_or("No file path saved.".to_owned())?;
    let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    
    let mut content = Vec::new();
    file.read_to_end(&mut content).map_err(|e| format!("Failed to read file: {}", e))?;
    
    let field_provider = FieldProvider::new(&content).map_err(|e| format!("Failed to parse file: {}", e))?;
    let gpu_field_provider = GPUFieldProvider::new(&content).map_err(|e| format!("Failed to parse file: {}", e))?;
    Ok((field_provider, gpu_field_provider))
}

#[cfg(target_arch = "wasm32")]
pub fn reload_file(_state: &State) -> Result<(FieldProvider, GPUFieldProvider), String> {
    let content = js!(return getData();).into_string().ok_or("Failed to get data from JS.".to_owned())?;
    let pos = content.find(",").map(|i| i + 1).unwrap_or(0);
    
    let b64 = content.split_at(pos).1;
    let data = base64::decode(b64).map_err(|e| format!("Failed to decode base64 content: {}", e))?;
    
    let field_provider = FieldProvider::new(&data).map_err(|e| format!("Failed to parse data: {}", e))?;
    let gpu_field_provider = GPUFieldProvider::new(&data).map_err(|e| format!("Failed to parse data: {}", e))?;
    Ok((field_provider, gpu_field_provider))
}
