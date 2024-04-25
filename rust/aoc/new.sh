#!/bin/bash

cargo new --name "aoc_$1_$2" "$1/$2"
cd $1/$2
cargo add challenges_common anyhow itertools lazy-regex
