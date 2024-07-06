//Adding Rust cargo packages
use std::fs::{File};
use std::io::{self, BufRead};
use zip::result::ZipError;
use zip::{ZipArchive};
use std::path::{Path, PathBuf};
use zip::read::ZipFile;
use tauri;
use crate::password;


//Called when the file reading does not require any password
pub fn fn_without_pswd(i: usize, archive: &mut ZipArchive<File>) -> Result<ZipFile, ZipError> {
    archive.by_index(i)
  }

/// Helper function that always returns false
//Another helper function for error handling
pub fn fn_ret_false()->bool{
    false
  }


  //Called when password is required to decrypt the file
pub fn fn_pswd(i: usize, archive: &mut ZipArchive<File>,myPassword:password) -> Result<ZipFile, ZipError> {
    match archive.by_index_decrypt(i, myPassword.pswd.as_bytes()) {
        Ok(zip_file) => Ok(zip_file),
        Err(e) => {
            Err(e)
        }
    }
  }

  //Helper function for function read_zip_files
#[tauri::command]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//To check if the zipfile requires password to decrypt it 
#[tauri::command]
pub fn prior_check(zippath:String) -> bool{
  
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