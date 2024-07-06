// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod metadata;
mod helper_fns;
mod handle_with_pswd;
mod handle_without_pswd;
mod file_handling;


use metadata::read_data::read_metadata;
use handle_without_pswd::extract::extract_zip;
use app::password;
use file_handling::reading::config_read;
use file_handling::writing::config_write;
use tauri;
use helper_fns::fns::prior_check;
use handle_with_pswd::reading::read_zip_files_pswd;
use handle_without_pswd::read::read_zip_files;
use handle_with_pswd::extraction::extract_zip_pswd;


// Main entry point for the Tauri application
//Getting all the tauri command functions inside the invoke handler method
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![config_read,read_zip_files_pswd,read_metadata,read_zip_files,extract_zip,config_write,prior_check,extract_zip_pswd])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}









