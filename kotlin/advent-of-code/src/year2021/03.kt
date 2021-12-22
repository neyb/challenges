package year2021.day3

import common.day

fun main() = day(2021, 3, ::part1, ::part2)

fun part1(lines: List<String>): Int {

    val gammaBinary = lines
        .fold((1..lines[0].length).map { 0 }.toMutableList()) { oneCounts, line ->
            line.asSequence()
                .foldIndexed(oneCounts) { index, _, char -> oneCounts.also { if (char == '1') it[index] += 1 } }
        }
        .map { if (it >= lines.size / 2) "1" else "0" }

    val gamma = gammaBinary.joinToString("").toInt(2)
    val epsilon = gammaBinary.map { if (it == "1") "0" else "1" }.joinToString("").toInt(2)

    return gamma * epsilon
}

fun part2(lines: List<String>): Int {

    fun calcRating(winner: (nbOnes: Int, nbZeroes: Int) -> Char) = (0..lines[0].length - 1)
        .fold(lines) { remainingLines, index ->
            if (remainingLines.size == 1) return@calcRating remainingLines.first().toInt(2)

            val nbOnes = remainingLines.asSequence().map { it[index] }.count('1'::equals)
            val nbZeroes = remainingLines.size - nbOnes
            val winnerChar = winner(nbOnes, nbZeroes)

            remainingLines.filter { it[index] == winnerChar }
        }.first().toInt(2)

    val o2rating = calcRating { nbOnes: Int, nbZeroes: Int -> if (nbOnes >= nbZeroes) '1' else '0' }
    val co2rating = calcRating { nbOnes: Int, nbZeroes: Int -> if (nbOnes >= nbZeroes) '0' else '1' }

    return o2rating * co2rating
}
