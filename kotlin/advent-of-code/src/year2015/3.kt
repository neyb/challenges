package year2015.day3

import common.day

fun main() = run().forEach(::println)
fun run() = day(2015, 3, part1, part2)

val part1 = { lines: List<String> ->
    lines.asSequence()
        .flatMap { it.asSequence() }
        .runningFold(0 to 0, Pair<Int, Int>::move)
        .distinct()
        .count()
}

fun Pair<Int, Int>.move(char: Char) = when (char) {
    '>' -> first + 1 to second
    '<' -> first - 1 to second
    '^' -> first to second + 1
    'v' -> first to second - 1
    else -> throw Exception("unsuuported char $char")
}

val part2 = { lines: List<String> ->
    lines.asSequence()
        .flatMap { it.asSequence() }
        .runningFoldIndexed((0 to 0) to (0 to 0)) { index, (santa, santaRobo), char ->
            (if (index % 2 == 0) santa.move(char) else santa) to
                    (if (index % 2 == 1) santaRobo.move(char) else santaRobo)
        }
        .flatMap { (santa, robo) -> listOf(santa, robo) }
        .distinct()
        .count()
}
