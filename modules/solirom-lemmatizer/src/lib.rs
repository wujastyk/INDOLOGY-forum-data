use fst::Map;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Clone)]
pub struct Lemmatizer {
    flexionary_forms: Map<Vec<u8>>,
    lemmas: Vec<String>,
}

impl Lemmatizer {
    pub fn new(flexionary_forms_file_path: &str, lemmas_file_path: &str) -> Lemmatizer {
        let mut flexionary_forms_file_handle =
            File::open(flexionary_forms_file_path).expect("cannot open file");
        let mut bytes = vec![];
        flexionary_forms_file_handle
            .read_to_end(&mut bytes)
            .expect("cannot convert bytes");

        let flexionary_forms = Map::new(bytes).expect("cannot read the FST");

        let lemma_file = File::open(lemmas_file_path).expect("cannot open file");
        let lemmas: Vec<String> = BufReader::new(lemma_file)
            .lines()
            .map(|line| line.unwrap())
            .collect();

        Lemmatizer {
            flexionary_forms,
            lemmas,
        }
    }

    pub fn get_key(&mut self, flexionary_form: &str) -> Option<&String> {
        let fst_value = &self.flexionary_forms.get(flexionary_form);

        if let Some(flexionary_form_indicial_number) = fst_value {
            let lemma_indicial_number: usize =
                flexionary_form_indicial_number.to_string().parse().unwrap();

            Some(&self.lemmas[lemma_indicial_number])
        } else {
            None
        }
    }
}

#[test]
fn load_fst() {
    let mut lemmatizer = Lemmatizer::new(
        "../generate-en-language-tools/flexionary-forms-and-lemmas/flexionary-forms.fst",
        "../generate-en-language-tools/flexionary-forms-and-lemmas/lemmas.txt",
    );
    println!("{:?}", lemmatizer.get_key("readable"));
}
