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
test 3a `cargo run --bin toboggan -- 3 1 < inputs/day3` 242
test 3b11 `cargo run --bin toboggan -- 1 1 < inputs/day3` 82
test 3b51 `cargo run --bin toboggan -- 5 1 < inputs/day3` 71
test 3b71 `cargo run --bin toboggan -- 7 1 < inputs/day3` 67
test 3b12 `cargo run --bin toboggan -- 1 2 < inputs/day3` 24
test 4ba `cargo run --bin passport < inputs/day4` 170
test 4bb `cargo run --bin strict_passport < inputs/day4` 103
test 5a `cargo run --bin max_boarding < inputs/day5` 915
