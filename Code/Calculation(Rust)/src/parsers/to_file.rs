/*


++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++

This module serves some basic functions for writing into a file. 
Normally:  'create_empty_file' is called -> calls 'params_to_string' and 'create_file'
But each method can also be used individually.

*/

use std::fs::File;
use std::fs::create_dir_all;
use std::path::Path;


//------------------------Public API functions------------------------------------
pub fn create_empty_file(path : &str, name : &str, ending: &str) -> File{
    assert!(ending.len() > 0);
    assert!(ending.starts_with("."));
    assert!(name.len() > 0);
    assert!(path.len() > 0);

    let mut full_filepath = params_to_string(path, name, ending);
    create_directory(path);
    create_file(&full_filepath)
}


pub fn params_to_string(path : &str, name : &str, ending: &str) -> String{
    let mut full_filepath = path.clone().to_string();
    full_filepath.push_str(name);
    full_filepath.push_str(ending);

    full_filepath
}


pub fn create_file(path : &String) -> File{
    File::create(path).unwrap()
}

pub fn create_directory(pathstr : &str){
    let path = Path::new(pathstr);
    create_dir_all(&path);
}
//------------------------private functions --------------------------------------
