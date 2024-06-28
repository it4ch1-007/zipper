// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead,Write};
use std::{fs};
use serde::Serialize;
use zip::result::ZipError;
use zip::{ZipArchive};
use std::path::{Path, PathBuf};
use zip::read::ZipFile;
use tauri;



#[derive(Serialize)]
struct ZipFileMetadata<>{
  entries: usize,
  comment: String,
  data_size: u64,
  is_empty: bool,
}

struct password{
  pswd:String,
}

fn fn_without_pswd(i: usize, archive: &mut ZipArchive<File>) -> Result<ZipFile, ZipError> {
  archive.by_index(i)
}


fn fn_pswd(i: usize, archive: &mut ZipArchive<File>,myPassword:password) -> Result<ZipFile, ZipError> {
  match archive.by_index_decrypt(i, myPassword.pswd.as_bytes()) {
      Ok(zip_file) => Ok(zip_file),
      Err(e) => {
          Err(e)
      }
  }
}
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![config_read,read_zip_files_pswd,read_metadata,read_zip_files,extract_zip,config_write,prior_check,extract_zip_pswd])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn extract_zip_pswd(zipPath:String,pswd:String){
  
  let zipname = std::path::Path::new(&*zipPath); //getting the dereference to get the value of the string but passing it as a reference to the actual function paramter.
  let file = File::open(&zipname).unwrap();
  let mut archive = zip::ZipArchive::new(file).unwrap(); //making a new instance of the zip file
  let mut pass=pswd;

  for i in 0..archive.len(){
    let myPassword = password{
      pswd:pass.clone(),
    };
    let mut new_archive = zip::ZipArchive::new(File::open(&zipname).unwrap()).unwrap();
      let file = match archive.by_index_decrypt(i,pass.as_bytes()){
          Ok(mut file) => fn_without_pswd(i,&mut new_archive),
          Err(err)=> 
         
              {
                
                  fn_pswd(i,&mut new_archive,myPassword)

              },
              
     
      }.unwrap();
    let substring = zipPath.rsplitn(2, '\\').nth(1).expect("msg");
  let mut outpath = PathBuf::new();
  let sec_str = file.enclosed_name().expect("msg");
  outpath.push(substring);
  outpath.push(sec_str);

  if(*file.name()).ends_with('/'){
    fs::create_dir_all(&outpath).unwrap();//passing a reference as parameter
  }
  else{

    if let Some(p) = outpath.parent(){
        if !p.exists(){
           fs::create_dir_all(&p).unwrap();
        }
    }
  let mut outFile = fs::File::create(&outpath).unwrap();
    } 

  }
}

#[tauri::command]
fn config_write(zipPath:String){
  let path = "../src/utils/config.txt";
  let mut config_file = path;
  let mut vec_none: Vec<String> = vec![];

  if let Ok(lines) = read_lines(config_file){
    let mut vec_lines = Vec::new();
        for line in lines {
            if let Ok(utf16_line) = line {
                let utf8_line = utf16_line.encode_utf16() 
                    .filter(|&ch| ch != 0) 
                    .collect::<Vec<u16>>(); 
                let utf8_line = String::from_utf16_lossy(&utf8_line); 
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
            writeln!(output,"{}",line);
        }
    }

}

#[tauri::command]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn fn_ret_false()->bool{
  false
}

#[tauri::command]
fn extract_zip(zippath:String,pswd:String){
  
  let zipname = std::path::Path::new(&*zippath); //getting the dereference to get the value of the string but passing it as a reference to the actual function paramter.
  let file = File::open(&zipname).unwrap();
  let mut archive = zip::ZipArchive::new(file).unwrap(); //making a new instance of the zip file
  let mut password = "hello".to_string();
  let mut pass = pswd;
  for i in 0..archive.len(){
    let mut new_archive = zip::ZipArchive::new(File::open(&zipname).unwrap()).unwrap();
    let myPassword = password{
      pswd:pass.clone(),
    };
      let file = match archive.by_index(i){
          Ok(mut file) => fn_without_pswd(i,&mut new_archive),
          Err(err)=> 
              {    fn_pswd(i,&mut new_archive,myPassword)},
        }.unwrap();
  let mut substring = zippath.rsplitn(2, '\\').nth(1).expect("msg");
  let mut outpath = PathBuf::new();
  let sec_str = file.enclosed_name().expect("msg");
  outpath.push(substring);
  outpath.push(sec_str);
  if(*file.name()).ends_with('/'){
    fs::create_dir_all(&outpath).unwrap();//passing a reference as parameter
  }
  else{
    if let Some(p) = outpath.parent(){
        if !p.exists(){
           fs::create_dir_all(&p).unwrap();
        }
    }
  let mut outFile = fs::File::create(&outpath).unwrap();
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
              {return fn_ret_false()},
            }
          }
          return true;
}
  

#[tauri::command]
fn read_zip_files_pswd(zippath:String,pswd:String) -> Vec<PathBuf>{

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
     
      let zip_metadata = ZipFileMetadata{
        entries: num_entries,
        comment:comment,
        data_size: prepended_data_size,
        is_empty: is_empty,
      };

      serde_json::to_string(&zip_metadata).unwrap()
}

#[tauri::command]
fn read_zip_files(zippath:String,pswd:String) -> Vec<PathBuf>{
  let zipname = std::path::Path::new(&*zippath);
  let mut return_vec: Vec<PathBuf> = vec![];
  let file = File::open(&zipname).unwrap();
  let mut archive = zip::ZipArchive::new(file).unwrap();
  let mut pass = pswd;
  for i in 0..archive.len(){
    let myPassword = password{
      pswd:pass.clone(),
    };
    let mut new_archive = zip::ZipArchive::new(File::open(&zipname).unwrap()).unwrap();
      let file = match archive.by_index(i){
          Ok(_) => fn_without_pswd(i,&mut new_archive),
          Err(_)=> 
              
              {

                  fn_pswd(i,&mut new_archive,myPassword)

              },
              
     
      }.unwrap();
      let outpath = match file.enclosed_name(){ //This resolves a security issue as here it checks whether the path is trying to get out of the directory or not

        Some(path) => path.to_owned(), //borrowing the instance of the filepath
        None => continue,
      };
      return_vec.push(outpath);
}
  return_vec
}

#[tauri::command]
fn config_read() -> Vec<String>{
  let path = "../src/utils/config.txt";
  let mut config_file = path;
  let mut vec_none: Vec<String> = vec![];

  if let Ok(lines) = read_lines(config_file){
    let mut vec_lines = Vec::new();

        for line in lines {
            if let Ok(utf16_line) = line {
                let utf8_line = utf16_line.encode_utf16() // Convert to UTF-16 encoded bytes
                    .filter(|&ch| ch != 0) // Filter out '\0' characters
                    .collect::<Vec<u16>>(); // Collect into a vector of u16

                let utf8_line = String::from_utf16_lossy(&utf8_line); // Convert UTF-16 to UTF-8 string
               
                vec_lines.push(utf8_line);
                
            }
        }
        if vec_lines.len()>=5 {
        vec_lines[0..5].to_vec()
        }
        else{
          vec_lines
        }

  }
  else{
    vec_none.push("Error reading the config file".to_string());

    vec_none
  }
  
}

