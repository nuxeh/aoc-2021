#!/usr/bin/env fish

aocf checkout 2 2021

echo Part 1
time cat test.txt | ./part1_pre_process.fish | fish
time aocf input | ./part1_pre_process.fish | fish

echo \nPart 2
time cat test.txt | ./part2_pre_process.fish | fish
time aocf input | ./part2_pre_process.fish | fish
