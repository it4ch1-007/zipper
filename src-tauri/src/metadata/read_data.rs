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

//Function to read the metadata of the zip file central directory
#[tauri::command]
pub fn read_metadata(archive: String) -> String{
     
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