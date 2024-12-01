#!/bin/bash

cargo new --name "aoc_$1_$2" "$1/$2" --vcs none
cd $1/$2
cargo add challenges_common anyhow itertools lazy-regex
cp -r ../../../template/aoc/* ./
