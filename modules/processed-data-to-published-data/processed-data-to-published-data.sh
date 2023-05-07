#!/bin/bash

base_data_dir=$(realpath "data")

for i in {1990..2022}
do
    current_data_dir="$base_data_dir/$i"
    
    for file in "$current_data_dir/*.txt"
    do
        sed -i -e \
            '/^\s*$\|^ *$\|^>\|^--\|^__\|^From[:]\{0,1\} \|^To: \|^Cc: \|^Date: \|^Message-ID: \|^Message-Id: \|^In-Reply-To: \|^Reply-To: \|^Received: \|^Return-path: \|^Phone \|^Tel: \|^Tel. \|was scrubbed...\|^Name: \|^Type: \|^Size: \|^Desc: \|^URL: \|^References: \|https:\/\/wetransfer.com\//d' $file
        sed -i 's|Subject: ||g' $file        
    done
done
# filter out //  At 08:48 AM