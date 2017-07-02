extern crate glob;

use std::collections::HashMap;
use std::fmt;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
pub struct Manager {
    kernels: HashMap<String, String>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            kernels: HashMap::new()
        }
    }

    pub fn load_kernels<S: Into<String> + fmt::Display>(&mut self, path: S) {
        let path = path.into();
        // TODO: Make sure there are no std::path::MAIN_SEPARATORs
        let pattern = format!("{}/**/*.cl", path);
        for entry in glob::glob(&pattern[..]).expect("Failed to read glob pattern") {
            match entry {
                Ok(kernel_path) => {
                    match File::open(&kernel_path) {
                        Ok(mut f) => {
                            let mut buffer = String::new();
                            match f.read_to_string(&mut buffer) {
                                Ok(_) => {

                                    let kernel_path = kernel_path
                                        .into_os_string()
                                        .into_string()
                                        .unwrap()
                                        .replace(&path[..], "")
                                        .replace(".cl", "");

                                    println!("Loading kernel {:?}", &kernel_path);
                                    self.kernels.insert(kernel_path, buffer);
                                }
                                Err(e) => {
                                    println!("Unable to read file {:?}, reason: {:?}", &kernel_path, e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Unable to read file {:?}, reason: {:?}", &kernel_path, e);
                        }
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }

    pub fn get_kernel<S: Into<String>>(&self, name: S) -> Option<&String> {
        let name = name.into();
        self.kernels.get(&name)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_as_well() {}
}
