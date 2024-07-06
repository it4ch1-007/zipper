use std::fs::{File};
use std::path::{PathBuf};
use tauri;

use crate::password;
use crate::helper_fns::fns::{fn_pswd,fn_without_pswd};

#[tauri::command]
pub fn read_zip_files(zippath:String,pswd:String) -> Vec<PathBuf>{
  
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
