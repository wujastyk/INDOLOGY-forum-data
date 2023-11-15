
GITHUB_WORKSPACE=$(pwd)
archives_dir_path=$GITHUB_WORKSPACE/archives
public_dir_path=$GITHUB_WORKSPACE/public
raw_data_dir_path=$public_dir_path/raw-data
data_dir_path=$public_dir_path/data

input_dir_path=$GITHUB_WORKSPACE/input

rm -rf $public_dir_path
mkdir -p $public_dir_path
mkdir -p $raw_data_dir_path
mkdir -p $data_dir_path

# copy the archives to the 'public/raw-data' folder
cp -a $archives_dir_path/. $raw_data_dir_path/

# unarchive the archives from the 'public/raw-data' folder
find "$raw_data_dir_path" -name "*.gz" -exec gunzip -d {} \;

# rename each file from the 'public/raw-data' folder and move it to a subfolder with year as name
declare -A months
months=([January]=01 [February]=02 [March]=03 [April]=04 [May]=05 [June]=06 [July]=07 [August]=08 [September]=09 [October]=10 [November]=11 [December]=12)
for file in $raw_data_dir_path/**/*.txt; do
    file_name=${file##*/}
    file_stem_name=${file_name%.txt}
    IFS=- read -r year month_name <<< $file_stem_name
    month_number=${months[$month_name]}
    new_file_name=$year-$month_number.txt
    mkdir -p $raw_data_dir_path/$year
    mv $file $raw_data_dir_path/$year/$new_file_name
done

# split each file in the 'public/raw-data' into messages
for file in $raw_data_dir_path/**/*.txt; do
    file_name=${file##*/}
    file_stem_name=${file_name%.txt}
    IFS=- read -r year month_number <<< $file_stem_name
    file_prefix=$raw_data_dir_path/$year/$month_number-
    csplit -s -z "$file" --prefix "$file_prefix" --suffix "%05d.txt" --elide-empty-files '/From /' '{*}'
    rm "$file"
done

# copy all the files from 'public/raw-data' folder to 'public/data' folder
cp -a $raw_data_dir_path/. $data_dir_path/

# process every file in the 'public/data' folder
time for file in $data_dir_path/**/*.txt; do
    sed -i -e \
        '/^\s*$\|^ *$\|^>\|^--\|^__\|^From[:]\{0,1\} \|^To: \|^Cc: \|^Date: \|^Message-ID: \|^Message-Id: \|^In-Reply-To: \|^Reply-To: \|^Received: \|^Return-path: \|^Phone \|^Tel: \|^Tel. \|was scrubbed...\|^Name: \|^Type: \|^Size: \|^Desc: \|^URL: \|^References: \|https:\/\/wetransfer.com\/\|=\?utf\-8?B\?/d' $file
    sed -i 's|Subject: ||g' $file
done

# generate the triples
time "$GITHUB_WORKSPACE/modules/text-data-to-triples/text-data-to-triples" "$GITHUB_WORKSPACE/data/" "**/*.txt" "$GITHUB_WORKSPACE/public/datasets/" "https://wujastyk.github.io/INDOLOGY-forum-data/indexes/words-fulltext/" "https://wujastyk.github.io/INDOLOGY-forum-data/data/"

# generate the indexes
time "$GITHUB_WORKSPACE/modules/triples-to-indexes/triples-to-indexes" "$GITHUB_WORKSPACE/public/datasets/terms/terms.ttl" "$GITHUB_WORKSPACE/public/indexes/"
time "$GITHUB_WORKSPACE/modules/triples-to-indexes/triples-to-indexes" "$GITHUB_WORKSPACE/public/datasets/text-relative-iris/text-relative-iris.ttl" "$GITHUB_WORKSPACE/public/indexes/"

# cd /home/claudius/workspace/repositories/git/github.com/wujastyk/INDOLOGY-forum-data/public/ && devserver --header Access-Control-Allow-Origin='*'