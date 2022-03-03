package year2015.day2

import common.day

fun main() = run().forEach(::println)
fun run() = day(2015, 2, ::part1, ::part2)

fun part1(lines: List<String>) = lines
    .map { it.split("x").map(String::toInt) }
    .map { (l, h, w) -> listOf(l * h, l * w, h * w) }
    .map { sides -> 2 * (sides.sum()) + sides.minOf { it } }
    .sum()

fun part2(lines: List<String>) = lines
    .map { it.split("x").map(String::toInt).sorted() }
    .map { (d1, d2, d3) -> 2*d1+2*d2 + d1*d2*d3 }
    .sum()

