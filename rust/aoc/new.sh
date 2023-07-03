#!/bin/bash

cargo new "aoc_$1_$2"
mkdir $1
mv "aoc_$1_$2" $1/$2
cd $1/$2
cargo add challenges_common
cargo add itertools
cargo add anyhow
