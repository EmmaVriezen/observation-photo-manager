// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn save_image(source: &str, target: &str, x1: u32, y1: u32, x2: u32, y2: u32) -> String {
    use image::io::Reader as Reader;

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

    let cropped_image = decoded_image.crop_imm(x1, y1, x2, y2);

    match cropped_image.save(format!("{target}\\copy.jpg")) {
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
