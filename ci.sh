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

case "${1:-}" in
    "dev" | "")
       bindir=target/debug
       ;;
   "release")
       bindir=target/release
       ;;
esac

test 1a `$bindir/day1 2 2020 < inputs/1a.txt` 1019371
test 1b `$bindir/day1 3 2020 < inputs/1a.txt` 278064990
test 2a `$bindir/password_check old < inputs/day2` 398
test 2b `$bindir/password_check new < inputs/day2` 562
test 3a `$bindir/toboggan 3 1 < inputs/day3` 242
test 3b11 `$bindir/toboggan 1 1 < inputs/day3` 82
test 3b51 `$bindir/toboggan 5 1 < inputs/day3` 71
test 3b71 `$bindir/toboggan 7 1 < inputs/day3` 67
test 3b12 `$bindir/toboggan 1 2 < inputs/day3` 24
test 4a `$bindir/day4-passport a < inputs/day4` 170
test 4b `$bindir/day4-passport b < inputs/day4` 103
test 5a `$bindir/day5-boarding a < inputs/day5` 915
test 5a `$bindir/day5-boarding b < inputs/day5` 699
test 6a `$bindir/day6-customs a < inputs/day6` 6504
test 6a `$bindir/day6-customs b < inputs/day6` 3351

unified_test() {
    day=$1
    part=$2
    expected=$3
    test $day$part `$bindir/aoc2020 $day$part < inputs/day$day` $3
}

unified_test 7 a 265
unified_test 7 b 14177
unified_test 8 a 2080
unified_test 8 b 2477
unified_test 9 a 104054607
unified_test 9 b 13935797
unified_test 10 a 1920
unified_test 10 b 1511207993344
unified_test 11 a 2334
unified_test 11 b 2100
unified_test 12 a 2270
unified_test 12 b 138669
unified_test 13 a 261
unified_test 13 b 807435693182510
unified_test 14 a 6513443633260
unified_test 14 b 3442819875191
unified_test 15 a 468
unified_test 15 b 1801753
