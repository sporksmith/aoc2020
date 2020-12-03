#!/bin/bash

set -euo pipefail

test() {
    name=$1
    actual=$2
    expected=$3
    if [ "$actual" != "$expected" ]
    then
        echo "$name:"
        echo "actual: $actual"
        echo "expect: $expected"
        exit 1
    fi
}

test 1a `cargo run --bin day1 -- 2 2020 < inputs/1a.txt` 1019371
test 1b `cargo run --bin day1 -- 3 2020 < inputs/1a.txt` 278064990
test 2a `cargo run --bin password_check -- old < inputs/day2` 398
test 2b `cargo run --bin password_check -- new < inputs/day2` 562
test 3a `cargo run --bin toboggan < inputs/day3` 242
