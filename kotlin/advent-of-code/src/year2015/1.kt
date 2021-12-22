package year2015.day1

import common.day

fun main() = day(2015, 1, ::part1, ::part2)

fun part1(lines: List<String>) = lines.floorMoves().reduce(Int::plus)

fun part2(lines: List<String>) =
    lines.floorMoves()
        .reduceIndexed { index, acc, i -> (acc + i).also { if (it == -1) return@part2 index + 1 } }
        .also { throw Exception("basement not reached") }

private fun Iterable<String>.floorMoves() = asSequence().flatMap { it.asSequence() }.map {
    when (it) {
        '(' -> 1
        ')' -> -1
        else -> throw Exception("cannot read $it, expect ( or )")
    }
}

