// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use core::arch;
use std::fs::{read, File, OpenOptions};
use std::io::{self, BufRead,Write};
use std::{fs, result};
use std::iter::zip;
use serde::Serialize;
use zip::result::ZipError;
use zip::{CompressionMethod, ZipArchive};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use serde_json::Serializer;
use zip::read::ZipFile;
use tauri;
use std::str;



#[derive(Serialize)]
struct ZipFileMetadata<>{
  entries: usize,
  comment: String,
  data_size: u64,
  is_empty: bool,
  // total_size: u128,
}
struct file_writer{
  vec_lines:Vec<String>,
}
struct var{
  flag:bool,
}
fn fn_without_pswd(i:usize,archive:&mut ZipArchive<File>) -> zip::read::ZipFile{

  // println!("Without pswd called");
  archive.by_index(i.try_into().unwrap()).unwrap()

  
}

#[tauri::command]
fn fn_prompt(){
  // let app:tauri::AppHandle;
  // app.emit_all("prompt_password", {}).unwrap();

}
fn fn_pswd(i:usize,archive:&mut ZipArchive<File>) -> zip::read::ZipFile{
  let mut pswd = String::new();
  println!("Wanted fn called::");

  // println!("The zip is password encrypted\nPlease Enter the Password: ");
  io::stdin().read_line(&mut pswd).expect("Error reading the user input");
  archive.by_index_decrypt(i,pswd.as_bytes()).unwrap()
}
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![config_read,read_zip_files_pswd,read_metadata,read_zip_files,extract_zip,error_printer,config_write,prior_check])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
#[tauri::command]
fn config_write(zipPath:String){
  println!("config_write called");
  let path = "../src/utils/config.txt";
  let mut config_file = path;
  let mut vec_none: Vec<String> = vec![];
  // let reader = BufReader::new(config_file);
  if let Ok(lines) = read_lines(config_file){
    // 
    let mut vec_lines = Vec::new();

        for line in lines {
            if let Ok(utf16_line) = line {
                let utf8_line = utf16_line.encode_utf16() // Convert to UTF-16 encoded bytes
                    .filter(|&ch| ch != 0) // Filter out '\0' characters
                    .collect::<Vec<u16>>(); // Collect into a vector of u16

                let utf8_line = String::from_utf16_lossy(&utf8_line); // Convert UTF-16 to UTF-8 string
                // println!("{:?}",utf8_line);
                vec_lines.push(utf8_line);
                
            }
        }
          vec_lines.insert(0,zipPath);
          vec_lines.pop();
         
          let mut output = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(config_file).unwrap();
        
        for line in &vec_lines{

            // println!("{}",line);
            // output.write(line.as_bytes()).expect("Error");
            writeln!(output,"{}",line);
        }
    }

}

// //     // let reader = BufReader::new(config_file);
// //     if let Ok(lines) = read_lines(config_file){
// //         let mut vec_lines: Vec<String> = lines.flatten().collect(); //converts the iterator into a vector of strings.
// //         vec_lines.insert(0, s.to_string());
// //         if vec_lines[0]==s{
// //             for line in &vec_lines{

// //                 println!("{}",line);
// //                 // output.write(line.as_bytes()).expect("Error");
// //                 // writeln!(output,"{}",line);
// //             }
// //         }
// //         else{
// //         if vec_lines.len()>5{
// //             vec_lines.pop();
// //         }
// //         let mut output = OpenOptions::new()
// //             .write(true)
// //             .truncate(true)
// //             .open(config_file).unwrap();
        
// //         for line in &vec_lines{

// //             println!("{}",line);
// //             // output.write(line.as_bytes()).expect("Error");
// //             // writeln!(output,"{}",line);
// //         }
// //     }
// //     }

#[tauri::command]
fn error_printer(){
  print!("Error occured");
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn fn_ret_false()->bool{
  println!("false fn called");
  false
}
fn fn_error(){
  println!("called");
}
#[tauri::command]
fn extract_zip(zippath:String){

  let zipname = std::path::Path::new(&*zippath); //getting the dereference to get the value of the string but passing it as a reference to the actual function paramter.
  let file = File::open(&zipname).unwrap();
  let mut archive = zip::ZipArchive::new(file).unwrap(); //making a new instance of the zip file
  let mut password = "hello".to_string();

  // let mut decrypted_file = ZipFile::new();
  for i in 0..archive.len(){
    let mut new_archive = zip::ZipArchive::new(File::open(&zipname).unwrap()).unwrap();
      let file = match archive.by_index(i){
          Ok(mut file) => fn_without_pswd(i,&mut new_archive),
          Err(err)=> 
              // ZipError::Io(_) => {
              //     eprintln!("IO error in opening the zip!! {:?}",err);
              // }
              // ZipError::FileNotFound => {
              //     eprint!("File not found!! {:?}",err);
              // }
              {
                
                  fn_pswd(i,&mut new_archive)
              //    archive.by_index_decrypt(i, &password.as_bytes()).expect("Failed!!!").unwrap()
              },
              
     
      };
    let substring = zippath.rsplitn(2, '/').nth(1).expect("msg");
  let mut outpath = PathBuf::new();
  let sec_str = file.enclosed_name().expect("msg");
  outpath.push(substring);
  outpath.push(sec_str);
  println!("{:?}",outpath);
  //  match file.enclosed_name(){ //This resolves a security issue as here it checks whether the path is trying to get out of the directory or not

  //     Some(path) => path.to_owned(), //borrowing the instance of the filepath
  //     None => continue,
  //   };
    
  

//now check for the folders inside the zip
  if(*file.name()).ends_with('/'){
  //if the file is not a folder
    // println!("File {} extracted to {:?}",i,outpath); //for displaying the path buffers we can also use outpath.display()
    fs::create_dir_all(&outpath).unwrap();//passing a reference as parameter

  }
  else{
  //if the file is a folder
    // println!("File {} extracted to {:?} of {} bytes",i,outpath,file.size());
    if let Some(p) = outpath.parent(){
        if !p.exists(){
           fs::create_dir_all(&p).unwrap();
        }
    }
  let mut outFile = fs::File::create(&outpath).unwrap();
  // fs::copy(&mut file,&outpath).unwrap(); //now copy the file into the zip to the file we just created in the new custom directory.

    } 

  }
}

#[tauri::command]
fn prior_check(zippath:String) -> bool{
  let zipname = std::path::Path::new(&*zippath);
  let mut return_vec: Vec<PathBuf> = vec![];
  let file = File::open(&zipname).unwrap();
  let mut archive = zip::ZipArchive::new(file).unwrap();
  for i in 0..archive.len(){
    let mut new_archive = zip::ZipArchive::new(File::open(&zipname).unwrap()).unwrap();
      let result =  archive.by_index(i);
      match result{
          Ok(file) => {continue;},
          Err(err)=> 
              // ZipError::Io(_) => {
              //     eprintln!("IO error in opening the zip!! {:?}",err);
              // }
              // ZipError::FileNotFound => {
              //     eprint!("File not found!! {:?}",err);
              // }
              {
                // let app_handle = tauri::Manager::app.handle(&self);
                  // fn_error();
                  return fn_ret_false()
              //    archive.by_index_decrypt(i, &password.as_bytes()).expect("Failed!!!").unwrap()
              },
            }
          }
          println!("true");
          return true;
}
  

#[tauri::command]
fn read_zip_files_pswd(zippath:String,pswd:String) -> Vec<PathBuf>{
  println!("read zip files with pswd called");
  println!("{:?}",pswd.as_bytes());
  let zipname = std::path::Path::new(&*zippath);
  let mut return_vec: Vec<PathBuf> = vec![];
  let file = File::open(&zipname).unwrap();
  let mut archive = zip::ZipArchive::new(file).unwrap();
  for i in 0..archive.len(){
    let mut new_archive = zip::ZipArchive::new(File::open(&zipname).unwrap()).unwrap();
      let file =  archive.by_index_decrypt(i,pswd.as_bytes()).unwrap();
      let outpath = match file.enclosed_name(){ //This resolves a security issue as here it checks whether the path is trying to get out of the directory or not

        Some(path) => path.to_owned(), //borrowing the instance of the filepath
        None => continue,
      };
      return_vec.push(outpath);
}
  return_vec
}

#[tauri::command]
fn read_metadata(archive: String) -> String{
      let file = File::open(archive).unwrap();
      let mut zip_archive = ZipArchive::new(file).unwrap();
      let num_entries = zip_archive.len();
      let comment = std::str::from_utf8(zip_archive.comment()).unwrap().to_string();
      let prepended_data_size = zip_archive.offset();
      let is_empty = zip_archive.is_empty();
      // let total_files_size = zip_archive.decompressed_size().unwrap();
      // for i in 0..num_entries{
      // let aes_key = zip_archive.get_aes_verification_key_and_salt(i);
      // }
      let zip_metadata = ZipFileMetadata{
        entries: num_entries,
        comment:comment,
        data_size: prepended_data_size,
        is_empty: is_empty,
        // total_size: total_files_size,

      };

      serde_json::to_string(&zip_metadata).unwrap()
}

#[tauri::command]
fn read_zip_files(zippath:String) -> Vec<PathBuf>{
  println!("read zip files called");
  let zipname = std::path::Path::new(&*zippath);
  let mut return_vec: Vec<PathBuf> = vec![];
  let file = File::open(&zipname).unwrap();
  let mut archive = zip::ZipArchive::new(file).unwrap();
  for i in 0..archive.len(){
    let mut new_archive = zip::ZipArchive::new(File::open(&zipname).unwrap()).unwrap();
      let file = match archive.by_index(i){
          Ok(mut file) => fn_without_pswd(i,&mut new_archive),
          Err(err)=> 
              // ZipError::Io(_) => {
              //     eprintln!("IO error in opening the zip!! {:?}",err);
              // }
              // ZipError::FileNotFound => {
              //     eprint!("File not found!! {:?}",err);
              // }
              {
                // let app_handle = tauri::Manager::app.handle(&self);
                  fn_pswd(i,&mut new_archive)
              //    archive.by_index_decrypt(i, &password.as_bytes()).expect("Failed!!!").unwrap()
              },
              
     
      };
      let outpath = match file.enclosed_name(){ //This resolves a security issue as here it checks whether the path is trying to get out of the directory or not

        Some(path) => path.to_owned(), //borrowing the instance of the filepath
        None => continue,
      };
      return_vec.push(outpath);
}
  return_vec
}
#[tauri::command]
fn test_button()->String{
  "Button is working....".to_string()
}
// #[tauri::command]
// // fn config_reader(){
// //     let path = "config.txt";
// //     let mut config_file = path;
// //     let mut s = "dormammooo!!!";
// //     // let reader = BufReader::new(config_file);
// //     if let Ok(lines) = read_lines(config_file){
// //         let mut vec_lines: Vec<String> = lines.flatten().collect(); //converts the iterator into a vector of strings.
// //         vec_lines.insert(0, s.to_string());
// //         if vec_lines[0]==s{
// //             for line in &vec_lines{

// //                 println!("{}",line);
// //                 // output.write(line.as_bytes()).expect("Error");
// //                 // writeln!(output,"{}",line);
// //             }
// //         }
// //         else{
// //         if vec_lines.len()>5{
// //             vec_lines.pop();
// //         }
// //         let mut output = OpenOptions::new()
// //             .write(true)
// //             .truncate(true)
// //             .open(config_file).unwrap();
        
// //         for line in &vec_lines{

// //             println!("{}",line);
// //             // output.write(line.as_bytes()).expect("Error");
// //             // writeln!(output,"{}",line);
// //         }
// //     }
// //     }
    
// // }
fn decode_utf16le(data: &str) -> io::Result<String> {
  // Convert UTF-16LE encoded string to UTF-8
  let utf16_bytes: Vec<u16> = data.encode_utf16().collect();
  let utf8_bytes = String::from_utf16_lossy(&utf16_bytes);
  Ok(utf8_bytes.into())
}

#[tauri::command]
fn config_read() -> Vec<String>{
  // let runtime_path = "C:\\Users\\akshi\\Downloads\\config.txt";
  // let runtime_file = Path::new(runtime_path);
  // let config_path = "C:\\Users\\akshi\\Downloads\\config.txt";
  // if runtime_file.exists() {
  //   if let Ok(metadata) = fs::metadata(runtime_file) {
  //     if metadata.len() > 0 {
  //         // Delete the existing config.txt if it exists
  //         let config_file = Path::new(config_path);
  //         if config_file.exists() {
  //             fs::remove_file(config_file);
  //         }
  //         fs::rename(runtime_file, config_file);
  //       }
        
  // let args: Vec<String> = std::env::args().collect();
  // println!("{:?}",args[0]);

  let path = "../src/utils/config.txt";
  let mut config_file = path;
  let mut vec_none: Vec<String> = vec![];
  // let reader = BufReader::new(config_file);
  if let Ok(lines) = read_lines(config_file){
    // 
    let mut vec_lines = Vec::new();

        for line in lines {
            if let Ok(utf16_line) = line {
                let utf8_line = utf16_line.encode_utf16() // Convert to UTF-16 encoded bytes
                    .filter(|&ch| ch != 0) // Filter out '\0' characters
                    .collect::<Vec<u16>>(); // Collect into a vector of u16

                let utf8_line = String::from_utf16_lossy(&utf8_line); // Convert UTF-16 to UTF-8 string
                // println!("{:?}",utf8_line);
                vec_lines.push(utf8_line);
                
            }
        }
        if vec_lines.len()>=5 {
        vec_lines[0..5].to_vec()
        }
        else{
          vec_lines
        }
        //just return the first 5 elements
  }
  else{
    vec_none.push("Error reading the config file".to_string());
    // println!("{:?}",vec_none);
    vec_none
  }
  
}

