use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;
use std::io::Read;

const LINE_MAX_LENGTH: usize = 80;
const SPACE_LENGTH: usize = 1;

fn main() {
    let mut text = String::new();
    load_text("../alastalon_salissa.txt", &mut text);

    let mut words_by_length: BTreeMap<usize, VecDeque<String>> = BTreeMap::new();
    group_words_by_length(&text, &mut words_by_length);

    let text = collect_optimal_rows(&mut words_by_length);
    write_text("../alastalon_salissa_output.txt", &text);
}

fn collect_optimal_rows(words_by_length: &mut BTreeMap<usize, VecDeque<String>>) -> String {
  let mut rows: Vec<String> = Vec::new();
  loop {
      match collect_optimal_row(words_by_length) {
          Some(row) => rows.push(row),
          None => break
      }
  }
  return rows.join("\n");
}

fn collect_optimal_row(words_by_length: &mut BTreeMap<usize, VecDeque<String>>) -> Option<String> {
    let mut row: Vec<String> = Vec::new();
    let mut line_length: usize = 0;
    while line_length < LINE_MAX_LENGTH {
        match longest_word(LINE_MAX_LENGTH - line_length, words_by_length) {
            Some(word) => {
                line_length += word.chars().count() + SPACE_LENGTH;
                row.push(word);
            },
            None => break
        }
    }
    if row.len() > 0 {
        return Some(row.join(" "));
    }
    return None;
}

fn longest_word(max_length: usize, words_by_length: &mut BTreeMap<usize, VecDeque<String>>) -> Option<String> {
    words_by_length
        .iter_mut()
        .rev()
        .find(|&(&length, ref words)| length <= max_length && words.iter().count() > 0)
        .map(|(_, words)| words.pop_front().unwrap())
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
    words_by_length
        .get_mut(&length)
        .unwrap()
        .push_back(word)
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