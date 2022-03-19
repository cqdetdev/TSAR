use std::fs::{read, File, read_dir, read_to_string, write, create_dir_all};
use std::io::*;
use std::path::Path;
use std::env::args;
use std::io::{ErrorKind, Error};
use std::process::exit;

pub fn main() {
    let args: Vec<String> = args().collect();
    let cmd = args.get(1);
    let dir = args.get(2);
    if cmd.is_none() { err_exit("No command is specified") }
    if dir.is_none() { err_exit("No directory is specified") }
    let cmd = &cmd.unwrap()[..];
    let dir = dir.unwrap();
    match cmd {
        "bundle" => { bundle(dir) }
        "unbundle" => { unbundle(dir) }
        _ => {
            err_exit("Invalid command is specified");
        }
    }
}

pub fn bundle(dir: &str) {
    append_out_file(None);
    read_recursive(dir, Vec::new());
}

pub fn unbundle(file: &str) {
    let data = read_to_string(file).expect("FILE ERROR: Unable to read from file");
    for line in data.split("\n") {
        if line.len() < 1 { continue };
        let split = line.split(":").collect::<Vec<&str>>();
        let (name, content) = (split[0], split[1]);
        if name.chars().nth(0).unwrap() == '.' { continue }; // Ignores hidden files
        let mut path_arr = name.split("/").collect::<Vec<&str>>();
        &path_arr.remove(path_arr.len() - 1);
        let full_path = &format!("out/{}", path_arr.join("/"));
        if !Path::new(&full_path).exists() {
            create_dir_all(full_path).expect("FILE ERROR: Unable to create directory");
        }
        let uint8_data: Vec<u8> = content.split(",").collect::<Vec<&str>>().iter().map(|x| {
            x.parse::<u8>().unwrap()
        }).collect();
        write(format!("out/{}", name), uint8_data).expect("FILE ERROR: Unable to create/write file");
    }
}   


fn append_out_file(data: Option<&str>) {
    if data == None {
        File::create("out.tsar").expect("FILE ERROR: Unable to create file");
    } else {
        let data = data.unwrap();
        let mut buf = File::create("out.tsar").unwrap();
        buf.write(data.as_bytes()).unwrap();
    }   
}

fn read_recursive(path: &str, mut arr_files: Vec<String>) -> Vec<String> {
    for item in read_dir(path).unwrap() {
        let item = item.unwrap();
        let new_path = item.path();
        let name = item.file_name();
        let name = name.to_str().to_owned().unwrap();
        let full = &format!("{}/{}", path, name).to_owned();
        if new_path.is_dir() {
            arr_files = read_recursive(full, arr_files)
        } else {
            let data = read(full).unwrap();
            let mapped = &data.iter().map(|i| {
                i.to_string()
            }).collect::<Vec<String>>().join(",")[..];
            let f = &format!("{}:{}\n", full, mapped);
            append_out_file(Some(f));
        }
    }
    arr_files
}

pub fn err_exit(msg: &str) {
    let err = Error::new(ErrorKind::Other, msg);
    println!("Error: {}", err); 
    exit(1);
}