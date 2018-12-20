use crate::particles::VectorField;
use crate::{FieldProvider, GPUFieldProvider, State};
#[cfg(target_arch = "wasm32")]
use std::path::PathBuf;
#[cfg(target_arch = "wasm32")]
use stdweb::*;

pub enum FileResult {
    OptionsFile(reparser::Options),
    VectorField((FieldProvider, GPUFieldProvider)),
}

pub fn reload_file(state: &State) -> Result<FileResult, String> {
    let ext = get_ext(state)?;
    let data = get_data(state)?;
    handle_file_ext(&ext, &data, state)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_data(state: &State) -> Result<Vec<u8>, String> {
    use std::{fs::File, io::Read};

    let path = state
        .file_path
        .as_ref()
        .ok_or_else(|| "No file path saved.".to_owned())?;
    let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;

    let mut content = Vec::new();
    file.read_to_end(&mut content)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    Ok(content)
}

#[cfg(target_arch = "wasm32")]
pub fn get_data(_state: &State) -> Result<Vec<u8>, String> {
    // Get file extension
    let content = js!(return getData();)
        .into_string()
        .ok_or_else(|| "Failed to get data from JS.".to_owned())?;

    let pos = content.find(",").map(|i| i + 1).unwrap_or(0);
    let b64 = content.split_at(pos).1;
    base64::decode(b64).map_err(|e| format!("Failed to decode base64 content: {}", e))
}

#[cfg(not(target_arch = "wasm32"))]
fn get_ext(state: &State) -> Result<String, String> {
    state
        .file_path
        .as_ref()
        .ok_or_else(|| "No file path saved.".to_owned())?
        .extension()
        .ok_or_else(|| "No file extension.".to_owned())
        .map(|s| s.to_string_lossy().into_owned())
}

#[cfg(target_arch = "wasm32")]
fn get_ext(_state: &State) -> Result<String, String> {
    let path_str = js!(return getPath();)
        .into_string()
        .ok_or_else(|| "Failed to get path from JS.".to_owned())?;
    let mut path = PathBuf::new();
    path.push(path_str);

    path.extension()
        .ok_or_else(|| "No file extension.".to_owned())
        .map(|s| s.to_string_lossy().into_owned())
}

fn handle_file_ext(file_ext: &str, data: &[u8], state: &State) -> Result<FileResult, String> {
    match file_ext {
        "bincode" => {
            let vector_field =
                bincode::deserialize(data).map_err(|e| format!("Failed to parse data: {}", e))?;
            create_providers(vector_field)
        }
        "nhdr" => {
            let string_rep =
                std::str::from_utf8(data).map_err(|e| format!("Parse error: {}", e))?;
            Ok(FileResult::OptionsFile(
                reparser::Options::from_header_file(string_rep.lines()),
            ))
        }
        "raw" => {
            let options = state
                .options_file
                .as_ref()
                .ok_or_else(|| "No options file loaded.".to_owned())?;
            let data = reparser::load_data_bytes_from_opt(&options, data)?;
            let vector_field =
                bincode::deserialize(&data).map_err(|e| format!("Failed to parse data: {}", e))?;
            create_providers(vector_field)
        }
        _ => Err("Unknown file extension".to_owned()),
    }
}

fn create_providers(vectorfield: VectorField) -> Result<FileResult, String> {
    let gpu_field_provider = GPUFieldProvider::new(&vectorfield);
    let field_provider = FieldProvider::new(vectorfield);
    Ok(FileResult::VectorField((
        field_provider,
        gpu_field_provider,
    )))
}
