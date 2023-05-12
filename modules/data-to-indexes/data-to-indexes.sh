#!/bin/bash

base_public_dir=$(realpath "public")
index_cards_dir="$base_public_dir/indexes/text/index-cards"

echo "generate the index cards files"

echo "filter lines in index cards files"
for i in {1990..2022}
do
    current_data_dir="$index_cards_dir/$i"
    echo $current_data_dir
    for file in "$current_data_dir/*.idxc"
    do
        sed -i -e '/\t[?]\+$/d' $file
    done
done

echo "generate the indexes"


# filter out //  At 08:48 AM