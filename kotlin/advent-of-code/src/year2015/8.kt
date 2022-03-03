package year2015.day8

import common.*

fun main() = run().forEach(::println)
fun run() = day(2015, 8, part1, part2)

val part1 = { lines: List<String> ->
    lines.asSequence().map { it.length - it.decode().length }.sum()
}

val part2 = { lines: List<String> ->
    lines.asSequence().map { it.encodeDiff() }.sum()
}

fun String.decode() = asSequence()
    .fold("" to "") { (result, state), char ->
        when {
            state.isEmpty() && char == '"' -> result to ""
            state.isEmpty() && char == '\\' -> result to "\\"
            state.isNotEmpty() && Regex("""\\x(.{2})""").matches(state + char) ->
                (result + (state[2].toString() + char).toInt(16).toChar()) to ""
            state.isNotEmpty() && (state + char).startsWith("\\x") -> result to state + char
            else -> (result + char) to ""
        }
    }.first

fun String.encodeDiff() = count { it == '"' || it == '\\' } + 2