
GITHUB_WORKSPACE=$(pwd)

rm -rf "$GITHUB_WORKSPACE/public"

mkdir -p $GITHUB_WORKSPACE/public/{indexes,indexes-ngram,indexes-fst}
public_dir_absolute_path="$(realpath $GITHUB_WORKSPACE/public)"
cp -r ./data/ ./public
./modules/text-data-to-index-cards/text-data-to-index-cards "$GITHUB_WORKSPACE/data" "**/*.txt" "$GITHUB_WORKSPACE/public"
# copy the configuration files for indexes
cp "$GITHUB_WORKSPACE/modules/index-cards-to-indexes/indexes-config-fulltext.json" "$GITHUB_WORKSPACE/public/indexes/indexes-config.json"
cp "$GITHUB_WORKSPACE/modules/index-cards-to-indexes/indexes-config-ngram.json" "$GITHUB_WORKSPACE/public/indexes-ngram/indexes-config.json"
cp "$GITHUB_WORKSPACE/modules/index-cards-to-indexes/indexes-config-fst.json" "$GITHUB_WORKSPACE/public/indexes-fst/indexes-config.json"
# generate the indexes
time $GITHUB_WORKSPACE/modules/index-cards-to-indexes/index-cards-to-indexes "$public_dir_absolute_path/index-cards/**/*.idxc" "$public_dir_absolute_path/indexes/"
time $GITHUB_WORKSPACE/modules/index-cards-to-indexes/index-cards-to-indexes "$public_dir_absolute_path/index-cards/**/*.idxc" "$public_dir_absolute_path/indexes-ngram/"
time $GITHUB_WORKSPACE/modules/index-cards-to-indexes/index-cards-to-indexes "$public_dir_absolute_path/index-cards/**/*.idxc" "$public_dir_absolute_path/indexes-fst/"
# copy the indexes
cp -a "$GITHUB_WORKSPACE/public/indexes-ngram/." "$GITHUB_WORKSPACE/public/indexes/"
cp -a "$GITHUB_WORKSPACE/public/indexes-fst/." "$GITHUB_WORKSPACE/public/indexes/"
rm -rf "$GITHUB_WORKSPACE/public/indexes-ngram"
rm -rf "$GITHUB_WORKSPACE/public/indexes-fst"