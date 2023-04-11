use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::{env, fs};

use crate::tokenizer::{Token, Tokenizer, WhitespaceTokenizer};
use solirom_lemmatizer::Lemmatizer;

mod tokenizer;

fn main() {
    // get the base dir path
    let args: Vec<String> = env::args().collect();
    let source_files_glob = &args[1];
    dbg!(&source_files_glob);

    // initiate the lemmatizer
    let lemmatizer: Lemmatizer = Lemmatizer::new(
        "../generate-en-language-tools/flexionary-forms-and-lemmas/flexionary-forms.fst",
        "../generate-en-language-tools/flexionary-forms-and-lemmas/lemmas.txt",
    );

    for file_path in glob::glob(source_files_glob)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
    {
        dbg!(&file_path);
        filter_and_tokenize_file(file_path, lemmatizer.clone());
        /*let filtered_file: Vec<&str> = filter_file(file_path);

        let index = WhitespaceTokenizer
            .tokenize(&filtered_file.join("\n"), lemmatizer)
            .collect::<Vec<Token>>();*/

        //println!("{:?}\n", index);
    }
}

fn filter_and_tokenize_file(file_path: PathBuf, lemmatizer: Lemmatizer) {
    if let Ok(lines) = read_lines(file_path) {
        let mut filtered_file: Vec<&str> = Vec::new();

        for line in lines.iter() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with(['>', '\n']) {
                continue;
            }
            if line.starts_with("--") {
                continue;
            }
            if line.starts_with("From ") {
                continue;
            }
            if line.starts_with("From: ") {
                continue;
            }
            if line.starts_with("To: ") {
                continue;
            }
            if line.starts_with("Cc: ") {
                continue;
            }
            if line.starts_with("Date: ") {
                continue;
            }
            //if line.starts_with("Subject: ") { continue; }
            if line.starts_with("Message-ID: ") {
                continue;
            }
            if line.starts_with("Message-Id: ") {
                continue;
            }
            if line.starts_with("In-Reply-To: ") {
                continue;
            }
            if line.starts_with("Reply-To: ") {
                continue;
            }
            if line.starts_with("Received: ") {
                continue;
            }
            if line.starts_with("Return-path: ") {
                continue;
            }
            if line.starts_with("Phone ") {
                continue;
            }
            if line.starts_with("Tel: ") {
                continue;
            }
            if line.starts_with("Tel. ") {
                continue;
            }

            // filter out the attachments related lines
            if line.contains("was scrubbed...") {
                continue;
            }
            if line.starts_with("Name: ") {
                continue;
            }
            if line.starts_with("Type: ") {
                continue;
            }
            if line.starts_with("Size: ") {
                continue;
            }
            if line.starts_with("Desc: ") {
                continue;
            }
            if line.starts_with("URL: ") {
                continue;
            }

            filtered_file.push(line);
        }

        let tokenized_file = WhitespaceTokenizer
            .tokenize(&filtered_file.join("\n"), lemmatizer)
            .collect::<Vec<Token>>();

        //filtered_lines
        println!("{:?}\n", tokenized_file);
    }
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    let mut lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    lines.sort();

    Ok(lines)
}

#[test]
fn tokenize_file() {
    let lemmatizer = Lemmatizer::new(
        "../generate-en-language-tools/flexionary-forms-and-lemmas/flexionary-forms.fst",
        "../generate-en-language-tools/flexionary-forms-and-lemmas/lemmas.txt",
    );

    filter_and_tokenize_file(PathBuf::from("/home/claudius/workspace/repositories/git/github.com/wujastyk/INDOLOGY-forum-data/data/1990/11-00000.txt"), lemmatizer);
}

// time cargo build --bin text-data-to-index-cards --target-dir ./target --release --target x86_64-unknown-linux-musl
// time cargo run --bin text-data-to-index-cards --target-dir ./target -- "/home/claudius/workspace/repositories/git/github.com/wujastyk/INDOLOGY-forum-data/data/**/*.txt"
// time ./text-data-to-index-cards "/home/claudius/workspace/repositories/git/github.com/wujastyk/INDOLOGY-forum-data/data/**/*.txt"
// https://nitschinger.at/Text-Analysis-in-Rust-Tokenization/
