package common

import java.io.File

fun day(year: Int, day: Int, vararg parts: (lines: List<String>) -> Any?) = day(year, day, *parts) { readLines() }

fun <Parsed> day(year: Int, day: Int, vararg parts: (lines: Parsed) -> Any?, parse: File.() -> Parsed) {
    val parsed = fileOfDay(year, day).run { parse() }
    parts.forEach { println(it(parsed)) }
}

private fun linesOfDay(year: Int, day: Int) = fileOfDay(year, day).readLines()

private fun fileOfDay(year: Int, day: Int) = File("advent-of-code/inputs/$year/$day.txt")

fun <T> Iterable<T>.split(isSplitter: (T) -> Boolean): List<List<T>> =
    fold(mutableListOf(mutableListOf())) { list: MutableList<MutableList<T>>, item ->
        list.apply { if (isSplitter(item)) add(mutableListOf()) else last().add(item) }
    }

fun <T> Sequence<T>.split(isSplitter: (T) -> Boolean): Sequence<Iterable<T>> =
    generateSequence {
        val list = mutableListOf<T>()
        this@split.forEach {
            if (isSplitter(it)) return@generateSequence list
            else list.add(it)
        }
        return@generateSequence if (list.isEmpty()) null else list
    }
//    fold(mutableListOf(mutableListOf())) { list: MutableList<MutableList<T>>, item ->
//        list.apply { if (isSplitter(item)) add(mutableListOf()) else last().add(item) }
//    }