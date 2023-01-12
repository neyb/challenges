#!/bin/bash

cargo new "aoc_2022_$1"
mv "aoc_2022_$1" $1
cd $1
cargo add challenges_common
cargo add itertools
cargo add anyhow