package aoc2021.day1

import common.day

fun main() = day(2021, 1, ::part1, ::part2);

fun part1(lines: List<String>) = lines.map(String::toInt).countIncreases()

fun part2(lines: List<String>) = lines.asSequence()
    .map(String::toInt)
    .windowed(3)
    .map { it[0] + it[1] + it[2] }
    .countIncreases()

fun Sequence<Int>.countIncreases() = windowed(2).count { it[0] < it[1] }

fun Iterable<Int>.countIncreases() = asSequence().countIncreases()
