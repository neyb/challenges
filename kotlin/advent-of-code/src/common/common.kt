package common

import java.io.File

fun <Return> day(year: Int, day: Int, vararg parts: (lines: List<String>) -> Return): Sequence<Return> =
    day(year, day, *parts) { readLines() }

fun <Return, Parsed> day(
    year: Int,
    day: Int,
    vararg parts: (lines: Parsed) -> Return,
    parse: File.() -> Parsed
                        ): Sequence<Return> {
    val parsed = fileOfDay(year, day).run { parse() }
    return parts.asSequence().map { it(parsed) }
}

private fun linesOfDay(year: Int, day: Int) = fileOfDay(year, day).readLines()

val inputsDir = (File("advent-of-code").takeIf(File::exists) ?: File(".")).resolve("inputs")
private fun fileOfDay(year: Int, day: Int) =
    inputsDir.resolve("$year/$day.txt")

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