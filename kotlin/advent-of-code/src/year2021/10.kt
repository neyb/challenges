package year2021.day10

import common.day
import java.util.*

fun main() = day(2021, 10, part1, part2)

sealed interface Result {
    val keepTaking: Boolean

    object Ok : Result {
        override val keepTaking = true
    }

    object Incomplete : Result {
        override val keepTaking = true
    }

    class UnexpectedChar(val expected: Char, val actual: Char) : Result {
        override val keepTaking = false

        val score = when (actual) {
            ')' -> 3
            ']' -> 57
            '}' -> 1197
            '>' -> 25137
            else -> throw Exception("unexpected unexpected char : $actual")
        }
    }
}

class State {
    companion object {
        private val closeByOpen = mapOf('(' to ')', '[' to ']', '{' to '}', '<' to '>')
    }

    val openChunks: Deque<Char> = ArrayDeque()
    var result: Result = Result.Ok
    val expectedCloser get() = closeByOpen[openChunks.last]!!

    fun take(char: Char) = apply {
        if (result.keepTaking) {
            when (char) {
                '(', '[', '{', '<' -> {
                    openChunks.addLast(char)
                    Result.Incomplete
                }
                else -> if (expectedCloser == char) {
                    openChunks.removeLast()
                    if (openChunks.isEmpty()) Result.Ok else Result.Incomplete
                } else Result.UnexpectedChar(expectedCloser, char)
            }.also { result = it }
        }
    }

    val completionScore
        get() = openChunks.reversed().asSequence()
            .map { closeByOpen[it]!! }
            .fold(0L) { score, char ->
                (score * 5) + when (char) {
                    ')' -> 1
                    ']' -> 2
                    '}' -> 3
                    '>' -> 4
                    else -> throw Exception("unexpected closing char :$char")
                }
            }


}

val part1 = { lines: List<String> ->
    lines.asSequence()
        .mapNotNull { line ->
            line.asSequence()
                .runningFold(State(), State::take)
                .map { it.result }
                .filterIsInstance<Result.UnexpectedChar>()
                .firstOrNull()
                ?.score
        }
        .sum()
}

val part2 = { lines: List<String> ->
    lines.asSequence()
        .mapNotNull { line -> line.fold(State(), State::take).takeIf { it.result is Result.Incomplete } }
        .map { it.completionScore }
        .toList().run { sorted()[(size - 1) / 2] }
}

