use arrayvec::ArrayString;
use solirom_lemmatizer::Lemmatizer;

const MAX_STACK_TERM_LEN: usize = 15;

pub trait Tokenizer<'a> {
    /// A Tokenizer always needs to produce an Iterator of Tokens.
    type TokenIter: Iterator<Item = Token>;

    /// Takes the input string and tokenizes it based on the implementations rules.
    fn tokenize(&self, input: &'a str, lemmatizer: Lemmatizer) -> Self::TokenIter;
}

#[derive(PartialEq, Debug)]
enum Term {
    Stack(ArrayString<MAX_STACK_TERM_LEN>),
    Heap(String),
}

#[derive(PartialEq, Debug)]
enum Lemma {
    Stack(ArrayString<MAX_STACK_TERM_LEN>),
    Heap(String),
}

#[derive(PartialEq, Debug)]
pub struct Token {
    term: Term,
    start_offset: usize,
    position: usize,
    lemma: Lemma,
}

impl Token {
    #[inline]
    pub fn from_str(term: &str, start_offset: usize, position: usize, lemma: &str) -> Self {
        Token {
            term: Token::convert_term(term),
            start_offset: start_offset,
            position: position,
            lemma: Token::convert_lemma(lemma),
        }
    }

    #[inline]
    fn convert_term(term: &str) -> Term {
        if term.len() <= MAX_STACK_TERM_LEN {
            Term::Stack(ArrayString::<MAX_STACK_TERM_LEN>::from(term).unwrap())
        } else {
            Term::Heap(term.to_string())
        }
    }

    #[inline]
    fn convert_lemma(term: &str) -> Lemma {
        if term.len() <= MAX_STACK_TERM_LEN {
            Lemma::Stack(ArrayString::<MAX_STACK_TERM_LEN>::from(term).unwrap())
        } else {
            Lemma::Heap(term.to_string())
        }
    }

    /*#[inline]
    pub fn term(&self) -> &str {
        match self.term {
            Term::Heap(ref s) => s.as_ref(),
            Term::Stack(ref s) => s.as_ref(),
        }
    }*/
}

pub struct CharTokenIter<'a> {
    filter: fn(&(usize, (usize, char))) -> bool,
    input: &'a str,
    byte_offset: usize,
    char_offset: usize,
    position: usize,
    lemmatizer: Lemmatizer,
}

impl<'a> CharTokenIter<'a> {
    pub fn new(
        filter: fn(&(usize, (usize, char))) -> bool,
        input: &'a str,
        lemmatizer: Lemmatizer,
    ) -> Self {
        CharTokenIter {
            filter: filter,
            input: input,
            byte_offset: 0,
            char_offset: 0,
            position: 0,
            lemmatizer,
        }
    }
}

impl<'a> Iterator for CharTokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let mut skipped_bytes = 0;
        let mut skipped_chars = 0;
        for (cidx, (bidx, c)) in self.input[self.byte_offset..]
            .char_indices()
            .enumerate()
            .filter(&self.filter)
        {
            let char_len = c.len_utf8();
            if cidx - skipped_chars == 0 {
                self.byte_offset = self.byte_offset + char_len;
                self.char_offset += 1;
                skipped_bytes = skipped_bytes + char_len;
                skipped_chars += 1;
                continue;
            }

            let slice = &self.input[self.byte_offset..self.byte_offset + bidx - skipped_bytes];
            let lowercased_slice = slice.to_lowercase();
            let token = Token::from_str(
                lowercased_slice.as_str(),
                self.char_offset,
                self.position,
                self.lemmatizer.get_key(lowercased_slice.as_str()).unwrap_or(&lowercased_slice),
            );
            self.char_offset = self.char_offset + slice.chars().count() + 1;
            self.position += 1;
            self.byte_offset = self.byte_offset + bidx + char_len - skipped_bytes;

            return Some(token);
        }

        if self.byte_offset < self.input.len() {
            let slice = &self.input[self.byte_offset..];
            let lowercased_slice = slice.to_lowercase();
            let token = Token::from_str(
                lowercased_slice.as_str(),
                self.char_offset,
                self.position,
                self.lemmatizer.get_key(slice).unwrap_or(&lowercased_slice),
            );
            self.byte_offset = self.input.len();

            Some(token)
        } else {
            None
        }
    }
}

pub struct WhitespaceTokenizer;

impl<'a> Tokenizer<'a> for WhitespaceTokenizer {
    type TokenIter = CharTokenIter<'a>;

    fn tokenize(&self, input: &'a str, lemmatizer: Lemmatizer) -> Self::TokenIter {
        CharTokenIter::new(is_whitespace, input, lemmatizer)
    }
}

#[inline]
fn is_whitespace(input: &(usize, (usize, char))) -> bool {
    let (_, (_, c)) = *input;
    c.is_whitespace() || c.is_ascii_punctuation()
}

#[test]
fn should_split_between_words() {
    let lemmatizer = Lemmatizer::new(
        "../generate-en-language-tools/flexionary-forms-and-lemmas/flexionary-forms.fst",
        "../generate-en-language-tools/flexionary-forms-and-lemmas/lemmas.txt",
    );

    let expected = vec![
        Token::from_str("better", 0, 0, "well"),
        Token::from_str("timing", 7, 1, "time"),
    ];
    let actually = WhitespaceTokenizer
        .tokenize("better timing", lemmatizer)
        .collect::<Vec<Token>>();
    println!("{:?}", actually);
    assert_eq!(expected, actually);
}

/*
Token {
    term: Stack("brackets"),
    start_offset: 604,
    position: 109,
    lemma: Stack("bracket")
}
*/