#!/bin/bash

base_data_dir=$(realpath "data")

cp "$base_data_dir/01-00003.txt" "$base_data_dir/_test"

for i in {1990..2022}
do
    current_data_dir=$(realpath "$base_data_dir/$i")
    
    for file in "$current_data_dir/*.txt"
    do
        sed -i -e \
            '/^\s*$\|^ *$\|^>\|^--\|^__\|^From[:]\{0,1\} \|^To: \|^Cc: \|^Date: \|^Message-ID: \|^Message-Id: \|^In-Reply-To: \|^Reply-To: \|^Received: \|^Return-path: \|^Phone \|^Tel: \|^Tel. \|was scrubbed...\|^Name: \|^Type: \|^Size: \|^Desc: \|^URL: /d' $file
        sed -i 's|Subject: ||g' $file        
    done
done
