// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::fs::{read, File, OpenOptions};
use std::io::{self, BufRead,Write};
use std::fs;
use zip::result::ZipError;
use zip::{CompressionMethod, ZipArchive};
use std::io::BufReader;
use std::path::Path;

use zip::read::ZipFile;

struct var{
  flag:bool,
}
fn fn_nice(i:usize,archive:&mut ZipArchive<File>) -> zip::read::ZipFile{
  archive.by_index(i.try_into().unwrap()).unwrap()
  
}
fn fn_not_nice(i:usize,archive:&mut ZipArchive<File>) -> zip::read::ZipFile{
  let mut pswd = String::new();
  println!("The zip is password encrypted\nPlease Enter the Password: ");
  io::stdin().read_line(&mut pswd).expect("Error reading the user input");
  archive.by_index_decrypt(i,pswd.as_bytes()).unwrap()
}
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![config_read])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[tauri::command]
fn basic_extrn(args: Vec<String>){

  let zipname = std::path::Path::new(&*args[1]); //getting the dereference to get the value of the string but passing it as a reference to the actual function paramter.
  let file = File::open(&zipname).unwrap();
  let mut archive = zip::ZipArchive::new(file).unwrap(); //making a new instance of the zip file
  let mut password = "hello".to_string();
  // let mut decrypted_file = ZipFile::new();
  for i in 0..archive.len(){
    let mut new_archive = zip::ZipArchive::new(File::open(&zipname).unwrap()).unwrap();
      let file = match archive.by_index(i){
          Ok(mut file) => fn_nice(i,&mut new_archive),
          Err(err)=> 
              // ZipError::Io(_) => {
              //     eprintln!("IO error in opening the zip!! {:?}",err);
              // }
              // ZipError::FileNotFound => {
              //     eprint!("File not found!! {:?}",err);
              // }
              {
                
                  fn_not_nice(i,&mut new_archive)
              //    archive.by_index_decrypt(i, &password.as_bytes()).expect("Failed!!!").unwrap()
              },
              
     
      };

  let outpath = match file.enclosed_name(){ //This resolves a security issue as here it checks whether the path is trying to get out of the directory or not

      Some(path) => path.to_owned(), //borrowing the instance of the filepath
      None => continue,
    };
//now check for the folders inside the zip
  if(*file.name()).ends_with('/'){
  //if the file is not a folder
    println!("File {} extracted to {:?}",i,outpath); //for displaying the path buffers we can also use outpath.display()
    fs::create_dir_all(&outpath).unwrap();//passing a reference as parameter

  }
  else{
  //if the file is a folder
    println!("File {} extracted to {:?} of {} bytes",i,outpath,file.size());
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
fn read_metadata(file:ZipFile){
    println!("File name: {}",file.name());
        println!("File size: {}",file.compressed_size());
        println!("File size uncompressed: {}",file.size());
        println!("Is directory: {}",file.is_dir());
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
  let path = "C:\\Users\\akshi\\Downloads\\config.txt";
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
                println!("{:?}",utf8_line);
                vec_lines.push(utf8_line);
                
            }
        }
        vec_lines
  }
  else{
    vec_none.push("Error reading the config file".to_string());
    println!("{:?}",vec_none);
    vec_none
  }
  
}
// fn config_read() -> Vec<String> {
//   let path = "C:\\Users\\akshi\\Downloads\\config.txt"; // Replace with your actual file path
//   let config_file = path;

//   if let Ok(lines) = read_lines(config_file) {
//       let vec_lines: io::Result<Vec<String>> = lines
//           .map(|line| line.map(|l| decode_utf16le(&l).unwrap_or_else(|_| String::new())))
//           .collect();
//       vec_lines.unwrap()
      
//   } else {
//       println!("Error opening config file");
//       vec![]
//   }
// }