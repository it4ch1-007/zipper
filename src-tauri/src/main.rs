// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//Adding Rust cargo packages
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead,Write};
use std::{fs};
use serde::Serialize;
use zip::result::ZipError;
use zip::{ZipArchive};
use std::path::{Path, PathBuf};
use zip::read::ZipFile;
use tauri;


//Struct to represent metadata of the zip
#[derive(Serialize)]
struct ZipFileMetadata<>{
  entries: usize,
  comment: String,
  data_size: u64,
  is_empty: bool,
}

//This is to make password to be a global variable
struct password{
  pswd:String,
}

//Called when the file reading does not require any password
fn fn_without_pswd(i: usize, archive: &mut ZipArchive<File>) -> Result<ZipFile, ZipError> {
  archive.by_index(i)
}

//Called when password is required to decrypt the file
fn fn_pswd(i: usize, archive: &mut ZipArchive<File>,myPassword:password) -> Result<ZipFile, ZipError> {
  match archive.by_index_decrypt(i, myPassword.pswd.as_bytes()) {
      Ok(zip_file) => Ok(zip_file),
      Err(e) => {
          Err(e)
      }
  }
}

// Main entry point for the Tauri application
//Getting all the tauri command functions inside the invoke handler method
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![config_read,read_zip_files_pswd,read_metadata,read_zip_files,extract_zip,config_write,prior_check,extract_zip_pswd])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

//Helper function for function read_zip_files
#[tauri::command]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Helper function that always returns false
//Another helper function for error handling
fn fn_ret_false()->bool{
  false
}

//To extract the zip when password is required
#[tauri::command]
fn extract_zip_pswd(zipPath:String,pswd:String){
  
  let zipname = std::path::Path::new(&*zipPath); //getting the dereference to get the value of the string but passing it as a reference to the actual function paramter.
 
  let file = File::open(&zipname).unwrap();
 
  let mut archive = zip::ZipArchive::new(file).unwrap(); //making a new instance of the zip file
  
  let mut pass=pswd;

  for i in 0..archive.len(){
   
    let myPassword = password{
      pswd:pass.clone(), 
      //cloning the new variable to move its value inside the loop
      };

    let mut new_archive = zip::ZipArchive::new(File::open(&zipname).unwrap()).unwrap();
    
    let file = match archive.by_index_decrypt(i,pass.as_bytes()){
       
        Ok(mut file) => fn_without_pswd(i,&mut new_archive),
      
        Err(err)=> 
            {
                fn_pswd(i,&mut new_archive,myPassword)
            },
   
    }.unwrap(); //unwrapping to obtain a ZipFile instead of an option.

  let substring = zipPath.rsplitn(2, '\\').nth(1).expect("Error");//getting the parent directory of Zip given

  let mut outpath = PathBuf::new();

  let sec_str = file.enclosed_name().expect("Error");

  outpath.push(substring);

  outpath.push(sec_str);//outpath for the resulting path of the new file copied or decrypted from the zip extracted

  if(*file.name()).ends_with('/'){
   
    fs::create_dir_all(&outpath).unwrap();//passing a reference as parameter
    //to create a new directory for the files extracted
  }

  else
  {
    if let Some(p) = outpath.parent(){
     
        if !p.exists(){ //if the parent directory exists already then now new directory will be made
          //without this every file will make its owbn parent directory giving a lot of duplicate directories.
           fs::create_dir_all(&p).unwrap();
        }
    }
  let mut outFile = fs::File::create(&outpath).unwrap();
  } 

  }
}

//Extracts the files without the need of password
#[tauri::command]
fn extract_zip(zippath:String,pswd:String){
  
  let zipname = std::path::Path::new(&*zippath); //getting the dereference to get the value of the string but passing it as a reference to the actual function paramter.
  
  let file = File::open(&zipname).unwrap();
  
  let mut archive = zip::ZipArchive::new(file).unwrap(); //making a new instance of the zip file
  
  let mut password = "hello".to_string(); //to ensure that the empty password does not crash the app
  
  let mut pass = pswd;
  
  for i in 0..archive.len(){
   
    let mut new_archive = zip::ZipArchive::new(File::open(&zipname).unwrap()).unwrap(); //making a new instance of the zip file
    
    let myPassword = password{
      pswd:pass.clone(),//cloning the new variable to move its value inside the loop
    };
    
    let file = match archive.by_index(i){
        
        Ok(mut file) => fn_without_pswd(i,&mut new_archive),
       
        Err(err)=> 
           
            {    fn_pswd(i,&mut new_archive,myPassword)},
    
      }.unwrap();//unwrapping to obtain a ZipFile instead of an option.
  
  let mut substring = zippath.rsplitn(2, '\\').nth(1).expect("Error");//getting the parent directory of Zip given
 
  let mut outpath = PathBuf::new();
 
  let sec_str = file.enclosed_name().expect("Error");
 
  outpath.push(substring);
 
  outpath.push(sec_str);//outpath for the resulting path of the new file copied or decrypted from the zip extracted
 
  if(*file.name()).ends_with('/'){
 
    fs::create_dir_all(&outpath).unwrap();//passing a reference as parameter
 
  }
 
  else{
 
   if let Some(p) = outpath.parent(){
  
        if !p.exists(){
          //if the parent directory exists already then now new directory will be made
          //without this every file will make its owbn parent directory giving a lot of duplicate directories.
 
           fs::create_dir_all(&p).unwrap();
           //to create a new directory for the files extracted
 
        }
    }
  
  let mut outFile = fs::File::create(&outpath).unwrap();
    } 
  }
}

//To check if the zipfile requires password to decrypt it 
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
         
              {return fn_ret_false()},//Returning false in a fashinable way :P
          
            }
        
          }
         
          return true; //the zipfile requires password and is encrypted
}
  
//To read the files inside the zip archive encrypted with the password
#[tauri::command]
fn read_zip_files_pswd(zippath:String,pswd:String) -> Vec<PathBuf>{

  let zipname = std::path::Path::new(&*zippath);
  
  let mut return_vec: Vec<PathBuf> = vec![];
 
  let file = File::open(&zipname).unwrap();
 
  let mut archive = zip::ZipArchive::new(file).unwrap();
  
  for i in 0..archive.len(){
   
    let mut new_archive = zip::ZipArchive::new(File::open(&zipname).unwrap()).unwrap();
     
      let file =  archive.by_index_decrypt(i,pswd.as_bytes()).unwrap(); //to decrypt the files inside the zip one by one using the password from the prompt
   
      let outpath = match file.enclosed_name(){ //This resolves a security issue as here it checks whether the path is trying to get out of the directory or not

        Some(path) => path.to_owned(), //borrowing the instance of the filepath
       
        None => continue,
     
      };
     
      return_vec.push(outpath);
}
 
  return_vec //Returning tjhe vector containing the zip files name

}

//Function to read the metadata of the zip file central directory
#[tauri::command]
fn read_metadata(archive: String) -> String{
     
      let file = File::open(archive).unwrap();
     
      let mut zip_archive = ZipArchive::new(file).unwrap(); //making the file as a type of ZipArchive
      
      let num_entries = zip_archive.len(); //number of entries inside the zip
      
      let comment = std::str::from_utf8(zip_archive.comment()).unwrap().to_string(); //the comment stored inside the zip central directory
     
      let prepended_data_size = zip_archive.offset(); //The prepended data inside the bytes representation of the zip file
    
      let is_empty = zip_archive.is_empty();//If the zip archive is empty or not
     
     //Declaring an instance of the type ZipFileMetadata to store the details of the zip.
      let zip_metadata = ZipFileMetadata{
      
        entries: num_entries,
       
        comment:comment,
      
        data_size: prepended_data_size,
       
        is_empty: is_empty,
    
      };

      serde_json::to_string(&zip_metadata).unwrap() //return the metadata as a json string
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

//Function to read the configuration file to represent what files are viewed recently
#[tauri::command]
fn config_read() -> Vec<String>{
  let path = "../src/utils/config.txt";//Path of the configuiration file
  
  let mut config_file = path;
 
  let mut vec_none: Vec<String> = vec![]; //to return a none vector if there is any error in reading the file

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
        if vec_lines.len()>=5 { //Only represent the last 5 zip files that were extracted using Zipper.
       
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

//Function writing the recent extracted files
#[tauri::command]
fn config_write(zipPath:String){
  let path = "../src/utils/config.txt";//this is the configuration file storing the recent extracted files 
  //IF ANYONE WANTS TO RUN THE BUILD VERSION THEN THE CONFIG.TXT FILE MUST BE PLACED WHERE ITS PATH IS CONFIGURED FOR THAT IS INSIDE THE SAM DIRECTORY AS THE EXECUTABLE FILE.

  let mut config_file = path;
 
  let mut vec_none: Vec<String> = vec![];

  if let Ok(lines) = read_lines(config_file){
   
    let mut vec_lines = Vec::new();
        for line in lines {
           
            if let Ok(utf16_line) = line {
               
                let utf8_line = utf16_line.encode_utf16() 
                    .filter(|&ch| ch != 0) 
                    .collect::<Vec<u16>>(); 
               
                let utf8_line = String::from_utf16_lossy(&utf8_line); //Converting hex representation back to Ascii String
                vec_lines.push(utf8_line);
            }
        }
          vec_lines.insert(0,zipPath);
          vec_lines.pop();
         

         //Truncating the whole file and writing to it again
          let mut output = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(config_file).unwrap();
        
        for line in &vec_lines{
           
            writeln!(output,"{}",line);
        }
    }

}
