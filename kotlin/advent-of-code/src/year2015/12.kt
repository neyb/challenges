package year2015.day12

import common.*

fun main() = run().forEach(::println)
val run = { day(2015, 12, part1) }

val part1 = { lines: List<String> ->
    val numbers = Regex(""":(-?\d+)[]},]""")
    numbers.findAll(lines.joinToString(""))
        .map { it.groupValues[1] }
        .map { it.toInt() }
        .sum()
}

