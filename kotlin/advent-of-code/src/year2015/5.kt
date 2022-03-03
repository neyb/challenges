package year2015.day5

import common.*

fun main() = run().forEach(::println)
fun run() = day(2015, 5, part1, part2)

val part1 = { lines: List<String> ->
    val vowels = "aeiou".toCharArray()
    fun String.countVowels() = asSequence().count { it in vowels }
    fun String.hasDouble() = asSequence().windowed(2).any { it[0] == it[1] }
    fun String.noForbidden() = !Regex("ab|cd|pq|xy").containsMatchIn(this)
    fun String.isNice() = countVowels() >= 3 && hasDouble() && noForbidden()
    lines.count(String::isNice)
}

val part2 = { lines: List<String> ->
    fun String.rule1() = Regex("""(.{2}).*\1""").containsMatchIn(this)
    fun String.rule2() = asSequence().windowed(3).any { it[0] == it[2] }
    fun String.isNice() = rule1() && rule2()
    lines.count(String::isNice)
}


