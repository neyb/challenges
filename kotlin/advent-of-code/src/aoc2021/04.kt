package aoc2021.day4

import common.day

fun main() = day(2021, 4, ::part1, ::part2)

class Board(val lines: List<List<Int>>) {
    val drawnNumbers = mutableSetOf<Int>()

    val columns =
        lines.fold((1..5).map { emptyList<Int>() }) { columns, line -> columns.zip(line) { column, v -> column + v } }
    val positionsByNumbers =
        lines.flatMapIndexed { y, line -> line.mapIndexed { x, number -> number to (x to y) } }
            .groupBy({ it.first }, { it.second })

    fun linesOf(number: Int) = positionsByNumbers.getOrDefault(number, emptyList()).map { (_, y) -> lines[y] }
    fun columnsOf(number: Int) =
        positionsByNumbers.getOrDefault(number, emptyList()).map { (x, _) -> columns[x] }

    fun Iterable<Int>.isComplete() = all { drawnNumbers.contains(it) }

    data class Result(val indexLastDrawn: Int, val lastDrawn: Int, val sumUndrawn: Int)

    //    data class Result(val index, val lastDrawn: Int, val sumUndrawn: Int)
    fun winResult(numbers: Iterable<Int>) = numbers.asSequence()
        .withIndex()
        .filter { draw(it.value) }
        .firstOrNull()

    fun draw(drawnNumber: Int): Boolean {
        drawnNumbers += drawnNumber
        return (linesOf(drawnNumber).asSequence() + columnsOf(drawnNumber)).any { it.isComplete() }
    }

    fun sumUndrawn() = lines.asSequence().flatMap { it }.filter { !drawnNumbers.contains(it) }.reduce(Int::plus)


}

fun part1(lines: List<String>) = parseInput(lines).let { (drawn, boards) ->
    drawn.firstNotNullOfOrNull { drawnNumber ->
        boards.asSequence()
            .firstOrNull { it.draw(drawnNumber) }
            ?.let { it.sumUndrawn() * drawnNumber }
    }
}

fun part2(lines: List<String>) = parseInput(lines).let { (numbersToDraw, boards) ->
    boards.asSequence()
        .map { it to it.winResult(numbersToDraw) }
        .maxByOrNull { (_, result) -> result?.index ?: 0 }
        ?.let { (board, result) -> board.sumUndrawn() * result!!.value }
}

fun parseInput(lines: List<String>) = lines[0].split(",").map(String::toInt) to lines.asSequence()
    .drop(2)
    .chunked(6) { it.subList(0, 5) }
    .map { it.map { it.split(" ").filter { it.length > 0 }.map(String::toInt) } }
    .map { Board(it) }
    .toList()

