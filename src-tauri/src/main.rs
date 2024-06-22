// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![extract_zip])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn extract_zip(path:String) -> Result<String, String>{
  let result = format!("Extracted files from {}", path);
  println!("{:?}",path);
  Ok(result)
}
