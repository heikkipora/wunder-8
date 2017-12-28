use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;
use std::io::Read;

fn main() {
    let mut text = String::new();
    load_text("../alastalon_salissa.txt", &mut text);

    let mut words_by_length: BTreeMap<usize, VecDeque<String>> = BTreeMap::new();
    group_words_by_length(&text, &mut words_by_length);

    for (length, words) in &words_by_length {
        println!("{}: {}", length, words.get(0).unwrap());
    }
    write_text("../alastalon_salissa_output.txt", &text);
}

fn group_words_by_length(text: &String, words_by_length: &mut BTreeMap<usize, VecDeque<String>>) {
    for word in text.split_whitespace() {
        add_to_map(word.to_string(), words_by_length);
    }
}

fn add_to_map(word: String, words_by_length: &mut BTreeMap<usize, VecDeque<String>>) {
    let length = word.chars().count();
    if !words_by_length.contains_key(&length) {
        words_by_length.insert(length, VecDeque::new());
    }
    words_by_length.get_mut(&length).unwrap().push_back(word)
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