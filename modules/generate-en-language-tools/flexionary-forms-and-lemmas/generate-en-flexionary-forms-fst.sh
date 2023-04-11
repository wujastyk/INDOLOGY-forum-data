#!/bin/bash

# wget https://raw.githubusercontent.com/skywind3000/lemma.en/master/lemma.en.txt

COUNTER=0

rm lemmas.txt flexionary-forms.txt flexionary-forms.fst

grep -v '^;' lemma.en.txt | while read -r line; do
    # process the lemma
    lemma="${line%% ->*}"
    lemma="${lemma%%/*}"
    echo $lemma >> lemmas.txt

    # process the flexionary forms
    flexionary_forms=${line##*> }
    IFS=',' read -r -a array <<< "$flexionary_forms"
    for element in "${array[@]}"
    do
        echo "$element,$COUNTER" >> flexionary-forms.txt
    done

    # increment the counter
    COUNTER=$((COUNTER + 1))
done

#sort -o flexionary-forms.txt flexionary-forms.txt

time ~/workspace/repositories/git/github.com/BurntSushi/fst/target/release/fst map flexionary-forms.txt flexionary-forms.fst
