use std::fs::{File};
use std::{fs};
use std::path::{PathBuf};
use tauri;


use crate::helper_fns::fns::{fn_pswd,fn_without_pswd};
use crate::password;

//Extracts the files without the need of password
#[tauri::command]
pub fn extract_zip(zippath:String,pswd:String){
  
  let zipname = std::path::Path::new(&*zippath); //getting the dereference to get the value of the string but passing it as a reference to the actual function paramter.
  
  let file = File::open(&zipname).unwrap();
  
  let mut archive = zip::ZipArchive::new(file).unwrap(); //making a new instance of the zip file
  
  let mut password ="trello".to_string(); //to ensure that the empty password does not crash the app
  
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