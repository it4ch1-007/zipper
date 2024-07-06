use std::fs::{File};
use std;
use std::path::{PathBuf};
use tauri;


//To read the files inside the zip archive encrypted with the password
#[tauri::command]
pub fn read_zip_files_pswd(zippath:String,pswd:String) -> Vec<PathBuf>{

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

