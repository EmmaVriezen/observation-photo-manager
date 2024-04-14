// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn save_image(source: &str, target: &str, x1: f32, y1: f32, x2: f32, y2: f32) -> String {
    use image::io::Reader as Reader;
    use std::path::Path;

    let retrieved_image_result = Reader::open(source);

    let retrieved_image = match retrieved_image_result {
        Ok(file) => file,
        Err(_) => return "Retrieving image failed.".to_string(),
    };
    
    let decoded_image_result = retrieved_image.decode();

    let decoded_image = match decoded_image_result {
        Ok(decoded) => decoded,
        Err(_) => return "Decoding image failed.".to_string(),
    };

    let height = decoded_image.height();
    let multiplier = height as f32 / 400.0;
    let x1_scaled = (x1 * multiplier) as u32;
    let x2_scaled = (x2 * multiplier) as u32;
    let y1_scaled = (y1 * multiplier) as u32;
    let y2_scaled = (y2 * multiplier) as u32;

    let cropped_image = decoded_image.crop_imm(x1_scaled, y1_scaled, x2_scaled, y2_scaled);

    let file_stem = Path::new(source).file_stem().unwrap().to_str().unwrap();
    let extension = Path::new(source).extension().unwrap().to_str().unwrap();

    let mut i = 0;
    let mut i_str = i.to_string();

    while Path::new(&format!("{target}\\{file_stem}_{i_str}.{extension}")).exists() {
        i = i + 1;
        i_str = i.to_string();
    }

    match cropped_image.save(format!("{target}\\{file_stem}_{i_str}.{extension}")) {
        Ok(_) => return "Successfully saved image!".to_string(),
        Err(_) => return "Saving image failed.".to_string(),
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
