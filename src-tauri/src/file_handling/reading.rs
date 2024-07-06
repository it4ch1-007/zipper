use tauri;
use crate::helper_fns::fns::read_lines;

//Function to read the configuration file to represent what files are viewed recently
#[tauri::command]
pub fn config_read() -> Vec<String>{
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