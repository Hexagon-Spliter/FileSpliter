use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::string::FromUtf8Error;
use console::Term;
use inquire::{CustomType, InquireError, Select};
use inquire::validator::Validation;
use crate::structurs::FileStruct;

mod structurs;

fn pause() {
    let term = Term::stdout();
    println!("Press any key to continue...\r");
    let a = term.read_char();
}
fn bytes_to_string(bytes: Vec::<u8>) -> Result<String, FromUtf8Error> {
    String::from_utf8(bytes)
}

fn main() {
    let term = Term::stdout();
    term.set_title("File Spliter");

    let file_text = CustomType::<String>::new("File:")
        .with_help_message("File for split\\join")
        .with_error_message("Please type a valid file")
        .with_validator(|val: &String| {
            let path = Path::new(val);
            if path.exists() {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "File does not exist".into(),
                ))
            }
        })
        .prompt();

    match file_text {
        Ok(file_text) => {
            let file_text2 = file_text.clone();
            let extension = Path::new(&file_text2);
            let options: Vec<&str> = vec!["Split", "Join"];

            let ans: Result<&str, InquireError> = Select::new("What action to take?", options).prompt();

            match ans {
                Ok(choice) => {
                    if choice == "Split" {
                        let mut f = File::open(file_text);
                        match f {
                            Ok(mut file) => {
                                let mut buffer = Vec::<u8>::new();
                                let readed = file.read_to_end(&mut buffer);
                                match readed {
                                    Ok(r) => {

                                        let chunks_count = CustomType::<usize>::new("Chunks:")
                                            .with_help_message("Number of file parts")
                                            .prompt();
                                        match chunks_count {
                                            Ok(count) => {
                                                let sliced = buffer.chunks(r / count);
                                                let mut index = 0;
                                                let mut hashes = Vec::<String>::new();
                                                let ext = extension.extension();
                                                let mut format: String = String::from("");
                                                match ext {
                                                    None => {}
                                                    Some(ext) => {
                                                        format = ext.to_string_lossy().to_string();
                                                    }
                                                }

                                                for data in sliced {
                                                    let hash = sha256::digest(&*data);
                                                    let hash2 = hash.clone();
                                                    hashes.push(hash);
                                                    println!("{}", format!("# Getting part: {0}/{1} hash", index, count));
                                                    let mut file = File::create(hash2 + ".sp");


                                                    match file {
                                                        Ok(mut file) => {
                                                            file.write_all(data);
                                                            index = index + 1;
                                                            println!("{}", format!("# Creating part: {0}/{1}", index, count));
                                                        }
                                                        Err(_) => {
                                                            println!("# Error creating file");
                                                            pause();
                                                        }
                                                    }
                                                }

                                                let filest = FileStruct {
                                                    format: format.parse().unwrap(),
                                                    hashes,
                                                };

                                                let mut file = File::create(format!("{0}.json", extension.file_stem().unwrap().to_str().unwrap()));
                                                match file {
                                                    Ok(mut file) => {
                                                        let json = serde_json::to_string(&filest);
                                                        file.write_all(json.unwrap().as_bytes());
                                                    }
                                                    Err(_) => {
                                                        println!("# Error creating file");
                                                        pause();
                                                    }
                                                }
                                            }
                                            Err(_) => {}
                                        }

                                    }
                                    Err(_) =>  { println!("# Error reading file"); pause(); }
                                }
                            }
                            Err(_) =>  { println!("# Error opening file"); pause(); }
                        }
                    }
                    else if choice == "Join" {
                        let mut f = File::open(file_text);
                        match f {
                            Ok(mut file) => {
                                let mut buffer = Vec::<u8>::new();
                                // read the whole file
                                let readed = file.read_to_end(&mut buffer);
                                match readed {
                                    Ok(..) => {
                                        let json =bytes_to_string(buffer);
                                        match json {
                                            Ok(json) => {
                                                println!("{}", json);
                                                let file_struct = serde_json::from_str::<FileStruct>(json.as_str());
                                                match file_struct {
                                                    Ok(mut FileStruct) => {
                                                        let mut filename = String::new();
                                                        if FileStruct.format == "" {
                                                            filename = extension.file_stem().unwrap().to_str().unwrap().parse().unwrap();
                                                        }
                                                        else {
                                                            filename = format!("{}.{}",  extension.file_stem().unwrap().to_str().unwrap(), FileStruct.format);
                                                        }

                                                        let mut buffers = Vec::<u8>::new();

                                                        for hash in FileStruct.hashes {
                                                            let mut f = File::open(hash + ".sp");
                                                            match f {
                                                                Ok(mut file) => {
                                                                    let mut buffer = Vec::<u8>::new();
                                                                    // read the whole file
                                                                    let readed = file.read_to_end(&mut buffer);
                                                                    buffers.extend(buffer.as_slice());

                                                                }
                                                                Err(_) =>  { println!("# Error opening file"); pause(); }
                                                            }
                                                        }

                                                        let mut f = File::create(filename);
                                                        match f {
                                                            Ok(mut file) => {
                                                                file.write_all(buffers.as_slice());
                                                            }
                                                            Err(_) => { println!("# Error creating file"); pause(); }
                                                        }
                                                    }
                                                    Err(_) =>  { println!("# Error reading file"); pause(); }
                                                }
                                            }
                                            Err(_) =>  { println!("# Error reading file"); pause(); }
                                        }
                                    }
                                    Err(_) =>  { println!("# Error reading file"); pause(); }
                                }
                            }
                            Err(_) =>  { println!("# Error opening file"); pause(); }
                        }


                    }
                }
                Err(_) =>  { println!("# Incorect choice"); pause(); }
            }
        }
        Err(_) =>  { println!("# File not found"); pause(); }
    }
}
