#!/bin/bash

base_data_dir=$(realpath "data")

cp "$base_data_dir/01-00003.txt" "$base_data_dir/_test"
find "$base_data_dir/_test" -name "*.txt" -exec sed -i -e \
    '/^ *$\|^>\|^--\|^From[:]\{0,1\} \|^To: \|^Cc: \|^Date: \|^Message-ID: \|^Message-Id: \|^In-Reply-To: \|^Reply-To: \|^Received: \|^Return-path: \|^Phone \|^Tel: \|^Tel. /d' {} \;
find "$base_data_dir/_test" -name "*.txt" -exec sed -i 's|Subject: ||g' {} \;
#find "$base_data_dir/_test" -name "*.txt" -exec sed -i -e '/^Message-ID: /d' {} \;

#for i in {1990..2022}
#do
    #current_data_dir=$(realpath "$base_data_dir/$i")
    
    # find $current_data_dir -name "*.txt" -exec sed -e '/^$/d' {} \;
#done
