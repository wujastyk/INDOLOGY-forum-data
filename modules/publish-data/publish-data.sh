
GITHUB_WORKSPACE=$(pwd)
input_dir_path=$GITHUB_WORKSPACE/input
data_dir_path=$GITHUB_WORKSPACE/data
raw_data_dir_path=$GITHUB_WORKSPACE/raw-data

rm -rf "$GITHUB_WORKSPACE/public"
mkdir -p "$GITHUB_WORKSPACE/public"

# unarchive the archives from the 'input' folder
#find "$input_dir_path" -name "*.gz" -exec gunzip -d {} \;

# rename each file from 'input' folder and move it to a subfolder with year as name
declare -A months
months=([January]=01 [February]=02 [March]=03 [April]=04 [May]=05 [June]=06 [July]=07 [August]=08 [September]=09 [October]=10 [November]=11 [December]=12)
for file in $input_dir_path/*.txt; do
    file_name=${file##*/}
    file_stem_name=${file_name%.txt}
    IFS=- read -r year month_name <<< $file_stem_name
    month_number=${months[$month_name]}
    new_file_name=$month_number.txt
    mkdir -p $input_dir_path/$year
    mv $file $input_dir_path/$year/$new_file_name
done

# split each file in the 'input' into messages
for file in $input_dir_path/**/*.txt; do
    file_name=${file##*/}
    file_stem_name=${file_name%.txt}
    file_prefix=$input_dir_path/$year/$file_stem_name-   
    echo $file_prefix
    csplit -s -z "$file" --prefix "$file_prefix" --suffix "%05d.txt" --elide-empty-files '/From /' '{*}'
    rm "$file"
done

# copy all the files from 'input' folder to 'raw-data' folder
cp -a $input_dir_path/. $raw_data_dir_path/

# process every file in the 'input' folder
for file in "$input_dir_path/**/*.txt"; do
    sed -i -e \
        '/^\s*$\|^ *$\|^>\|^--\|^__\|^From[:]\{0,1\} \|^To: \|^Cc: \|^Date: \|^Message-ID: \|^Message-Id: \|^In-Reply-To: \|^Reply-To: \|^Received: \|^Return-path: \|^Phone \|^Tel: \|^Tel. \|was scrubbed...\|^Name: \|^Type: \|^Size: \|^Desc: \|^URL: \|^References: \|https:\/\/wetransfer.com\/\|=\?utf\-8?B\?/d' $file
    sed -i 's|Subject: ||g' $file
done

# copy all the processed files in 'input' folder to 'data' folder
cp -a $input_dir_path/. $data_dir_path/

# copy the raw data to the 'public' folder
cp -r "$GITHUB_WORKSPACE/raw-data/" "$GITHUB_WORKSPACE/public"

# copy the data to the 'public' folder
cp -r "$GITHUB_WORKSPACE/data/" "$GITHUB_WORKSPACE/public"

# generate the triples
time "$GITHUB_WORKSPACE/modules/text-data-to-triples/text-data-to-triples" "$GITHUB_WORKSPACE/data/" "**/*.txt" "$GITHUB_WORKSPACE/public/datasets/" "https://wujastyk.github.io/INDOLOGY-forum-data/indexes/words-fulltext/" "https://wujastyk.github.io/INDOLOGY-forum-data/data/"

# generate the indexes
time "$GITHUB_WORKSPACE/modules/triples-to-indexes/triples-to-indexes" "$GITHUB_WORKSPACE/public/datasets/terms/terms.ttl" "$GITHUB_WORKSPACE/public/indexes/"
time "$GITHUB_WORKSPACE/modules/triples-to-indexes/triples-to-indexes" "$GITHUB_WORKSPACE/public/datasets/text-relative-iris/text-relative-iris.ttl" "$GITHUB_WORKSPACE/public/indexes/"

# cd /home/claudius/workspace/repositories/git/github.com/wujastyk/INDOLOGY-forum-data/public/ && devserver --header Access-Control-Allow-Origin='*'