use std::fs::{OpenOptions};
use std::io::{Write};
use tauri;
use crate::helper_fns::fns::read_lines;


//Function writing the recent extracted files
#[tauri::command]
pub fn config_write(zipPath:String){
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
