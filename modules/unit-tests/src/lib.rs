use ngrammatic::{CorpusBuilder, Pad};

#[test]
fn ngrammatic() {
    let mut corpus = CorpusBuilder::new().arity(2).pad_full(Pad::Auto).finish();

    // Build up the list of known words
    corpus.add_text("Mah?praj??p?ramit???stra");
    corpus.add_text("Pr?ti??khya");
    corpus.add_text("cIvakacintAmaNi");
    corpus.add_text("???avaci??am");
    corpus.add_text("k□lacakra");

    println!("{:?}", corpus);

    // Now we can try an unknown/misspelled word, and find a similar match
    // in the corpus
    let results = corpus.search("tattvacintam", 0.25);
    let top_match = results.first();

    println!("{:?}", results);

    assert!(top_match.is_some());
    assert!(top_match.unwrap().similarity > 0.25);
    //assert_eq!(top_match.unwrap().text, String::from("tomato"));
}

// https://github.com/lotabout/fuzzy-matcher
// https://docs.rs/ngrammatic/latest/ngrammatic/
// https://github.com/wolfgarbe/SymSpell
// https://github.com/justinwilaby/spellchecker-wasm
// https://github.com/forrestthewoods/lib_fts/blob/master/code/fts_fuzzy_match.js

// Algorithms
// https://chappers.github.io/web%20micro%20log/2015/04/29/comparison-of-ngram-fuzzy-matching-approaches/
// https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm
