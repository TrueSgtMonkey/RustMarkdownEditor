use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::io;
use std::env;

fn main() {
    // printing out the environment the program starts in for debug
    let path_result = env::current_dir();
    match path_result {
        Ok(path) => println!("The current directory is {}", path.display()),
        Err(e) => println!("Cannot retrieve env: {}", e),
    };

    // Initial state -- Nothing is happening
    // Start state --  User selects file -- through the interface
    // * Check if file is good
    //      * Process begins if file is good
    //      * Otherwise, nothing happens
    // * Read contents of file
    // * Display contents of file
    // * Make changes
    // * Save changes
    let result = begin_processing("file.md");
    match result {
        Ok(_res) => println!("good!"),
        Err(e) => println!("Error: {}", e),
    };
}

fn begin_processing(file_name: &str) -> io::Result<()>{
    if !is_file_good(file_name) {
        return Ok(());
    }

    let mut reader = my_reader::BufReader::open(file_name)?;
    let mut buffer = String::new();


    while let Some(line) = reader.read_line(&mut buffer) {
        println!("{}", line?.trim());
    }

    Ok(())
}

// Need to transfer the ownership of the file to the caller so that the file
// remains open
// Cannot simply return File type
fn is_file_good(file_name: &str) -> bool {
    // we only open markdown files with this app
    if !file_name.ends_with(".md") {
        return false;
    }

    let file_result = File::open(file_name);
    let file_check = match file_result {
        // Lambda -- e is not needed and should have nothing happen
        Ok(file) => Some(file),
        Err(_e) => None,
    };

    // not markdown file and not file in general
    if file_check.is_none() {
        return false;
    }

    // is markdown file and exists
    return true;
}

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        // TODO: Need to figure out how to poke around here?
        // TODO: Only print out a portion of the file at a time?
        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}