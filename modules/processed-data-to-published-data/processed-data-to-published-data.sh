#!/bin/bash

base_data_dir="/home/claudius/varia/Dominik/data"

for i in {1990..2022}
do
    current_data_dir="$base_data_dir/$i"
    
    find "$current_data_dir" -name "*.gz" -exec gunzip -d {} \;
    
    find "$current_data_dir" -name "*January.txt" -exec mv {} "$current_data_dir/01.txt" \;
    find "$current_data_dir" -name "*February.txt" -exec mv {} "$current_data_dir/02.txt" \;
    find "$current_data_dir" -name "*March.txt" -exec mv {} "$current_data_dir/03.txt" \;
    find "$current_data_dir" -name "*April.txt" -exec mv {} "$current_data_dir/04.txt" \;
    find "$current_data_dir" -name "*May.txt" -exec mv {} "$current_data_dir/05.txt" \;
    find "$current_data_dir" -name "*June.txt" -exec mv {} "$current_data_dir/06.txt" \;
    find "$current_data_dir" -name "*July.txt" -exec mv {} "$current_data_dir/07.txt" \;
    find "$current_data_dir" -name "*August.txt" -exec mv {} "$current_data_dir/08.txt" \;
    find "$current_data_dir" -name "*September.txt" -exec mv {} "$current_data_dir/09.txt" \;
    find "$current_data_dir" -name "*October.txt" -exec mv {} "$current_data_dir/10.txt" \;
    find "$current_data_dir" -name "*November.txt" -exec mv {} "$current_data_dir/11.txt" \;
    find "$current_data_dir" -name "*December.txt" -exec mv {} "$current_data_dir/12.txt" \;
    
    for file in "$current_data_dir"/*.txt; do
        file_name=${file##*/}
        file_stem_name=${file_name%.txt}
        file_prefix="$current_data_dir"/"$file_stem_name"-
        csplit -s -z "$file" --prefix "$file_prefix" --suffix "%05d.txt" --elide-empty-files '/From /' '{*}'
        rm "$file"        
    done
done
