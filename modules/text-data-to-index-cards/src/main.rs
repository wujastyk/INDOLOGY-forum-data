use std::path::{Path, PathBuf};
use std::{env, fs};

use crate::tokenizer::{Token, Tokenizer, WhitespaceTokenizer};

mod tokenizer;

fn main() {
    // get the base dir path
    let args: Vec<String> = env::args().collect();
    let source_files_glob = &args[1];
    let indexes_dir = Path::new(&args[2]);

    generate_index_cards_for_files(source_files_glob, indexes_dir);
}

fn generate_index_cards_for_files(source_files_glob: &String, indexes_dir: &Path) {
    let mut global_index_cards: Vec<String> = Vec::new();
    let mut global_cross_references: Vec<String> = Vec::new();

    for (file_ordinal_number, file_path) in glob::glob(source_files_glob)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .enumerate()
    {
        dbg!(&file_path);

        let mut file_index_cards: Vec<String> = generate_index_cards_for_file(&file_path, file_ordinal_number);

        global_index_cards.append(&mut file_index_cards);

        // generate the title of the cross reference to file
        let file_identifier = format!(
            "{}/{}",
            &file_path
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
            &file_path.file_name().unwrap().to_str().unwrap()
        );

        global_cross_references.push(file_identifier);
    }

    fs::write(
        indexes_dir.join("index-cards.txt"),
        &global_index_cards.join("\n"),
    )
    .expect("Write file.");

    fs::write(
        indexes_dir.join("cross-references.json"),
        serde_json::to_string(&global_cross_references).expect("Serialize file."),
    )
    .expect("Write file.");
}

fn generate_index_cards_for_file(file_path: &PathBuf, file_ordinal_number: usize) -> Vec<String> {
    // get the file contents
    let file_contents: String = fs::read_to_string(file_path)
        .expect("cannot read file")
        .parse()
        .expect("cannot parse file");

    // get the contents' tokens
    let file_tokens = WhitespaceTokenizer
        .tokenize(&file_contents)
        .collect::<Vec<Token>>();

    // generate the index cards
    let mut index_cards: Vec<String> = Vec::new();
    file_tokens.iter().for_each(|file_token| {
        let token = file_token.term();
        index_cards.push(format!("{}\t{}\t{}", token, &file_ordinal_number, token));
    });

    index_cards
}

#[test]
fn test_generate_index_cards_for_file() {
    generate_index_cards_for_file(&PathBuf::from("/home/claudius/workspace/repositories/git/github.com/wujastyk/INDOLOGY-forum-data/data/_test/01-00003.txt"), 1);
}

#[test]
fn test_generate_index_cards_for_files() {
    let source_files_glob = &"/home/claudius/workspace/repositories/git/github.com/wujastyk/INDOLOGY-forum-data/data/**/*.txt".to_string();
    let indexes_dir = Path::new("/home/claudius/workspace/repositories/git/github.com/wujastyk/INDOLOGY-forum-data/public/indexes/text/");

    generate_index_cards_for_files(source_files_glob, indexes_dir);
}

// time cargo build --bin text-data-to-index-cards --target-dir ./target --release --target x86_64-unknown-linux-musl
// time cargo run --bin text-data-to-index-cards --target-dir ./target -- "/home/claudius/workspace/repositories/git/github.com/wujastyk/INDOLOGY-forum-data/data/**/*.txt"
// time ./text-data-to-index-cards "/home/claudius/workspace/repositories/git/github.com/wujastyk/INDOLOGY-forum-data/data/**/*.txt"
// https://nitschinger.at/Text-Analysis-in-Rust-Tokenization/

// initiate the lemmatizer
/*let lemmatizer: Lemmatizer = Lemmatizer::new(
    "../generate-en-language-tools/flexionary-forms-and-lemmas/flexionary-forms.fst",
    "../generate-en-language-tools/flexionary-forms-and-lemmas/lemmas.txt",
);*/
// self.lemmatizer.get_key(lowercased_slice.as_str()).unwrap_or(&lowercased_slice),
