#!/bin/fish

argparse -N2 -- $argv
or return 1

set -x YEAR $argv[1]
set -x DAY $argv[2]
set AOC_FOLDER (realpath (dirname (status filename)))
set TARGET_FOLDER "$AOC_FOLDER/$YEAR/$DAY/"

echo "Creating new day $DAY for year $YEAR in $TARGET_FOLDER"
read -P "Press enter to continue"

mkdir -p $TARGET_FOLDER

cp -r $AOC_FOLDER/template/* $TARGET_FOLDER

for file in (find $TARGET_FOLDER -type f)
    envsubst <$file >$file.tmp
    mv $file.tmp $file
end