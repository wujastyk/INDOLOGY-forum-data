
GITHUB_WORKSPACE=$(pwd)
rm -rf "$GITHUB_WORKSPACE/public"

# the part below goes into Github workflows
mkdir -p "$GITHUB_WORKSPACE/public"
public_dir_absolute_path="$(realpath $GITHUB_WORKSPACE/public)"
cp -r "$GITHUB_WORKSPACE/data/" "$GITHUB_WORKSPACE/public"

# generate the triples
time "$GITHUB_WORKSPACE/modules/text-data-to-triples/text-data-to-triples" "$GITHUB_WORKSPACE/data/" "**/*.txt" "$GITHUB_WORKSPACE/public/datasets/" "https://wujastyk.github.io/INDOLOGY-forum-data/indexes/words-fulltext/" "https://wujastyk.github.io/INDOLOGY-forum-data/data/"

# generate the indexes
time "$GITHUB_WORKSPACE/modules/triples-to-indexes/triples-to-indexes" "$GITHUB_WORKSPACE/public/datasets/terms/terms.ttl" "$GITHUB_WORKSPACE/public/indexes/"
time "$GITHUB_WORKSPACE/modules/triples-to-indexes/triples-to-indexes" "$GITHUB_WORKSPACE/public/datasets/text-relative-iris/text-relative-iris.ttl" "$GITHUB_WORKSPACE/public/indexes/"

# cd /home/claudius/workspace/repositories/git/github.com/wujastyk/INDOLOGY-forum-data/public/ && devserver --header Access-Control-Allow-Origin='*'