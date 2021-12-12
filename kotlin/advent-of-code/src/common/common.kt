package common

import java.io.File

fun day(year: Int, day: Int, vararg parts: (lines: List<String>) -> Any?) {
    val lines = linesOfDay(year, day)
    parts.forEach { println(it(lines)) }
}

private fun linesOfDay(year: Int, day: Int) = File("advent-of-code/inputs/$year/$day.txt").readLines()
