use std::io::BufReader;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;
use std::io::Read;

fn main() {
    let mut text = String::new();
    load_text("../alastalon_salissa.txt", &mut text);
    write_text("../alastalon_salissa_output.txt", &text);
}

fn load_text(file_name: &str, text: &mut String) -> Result<(), Error> {
    let text_file = File::open(file_name)?;
    let mut reader = BufReader::new(text_file);

    reader.read_to_string(text)?;
    Ok(())
}

fn write_text(file_name: &str, text: &String) -> Result<(), Error> {
    let mut text_file = File::create(file_name)?;
    text_file.write_all(text.as_bytes())?;
    Ok(())
}