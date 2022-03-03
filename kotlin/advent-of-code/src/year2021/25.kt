package year2021.day25

import common.*
import common.`object`.*

fun main() = run().forEach(::println)
val run = { day(2021, 25, part1) { RegionState.parse(readLines()) } }

val part1 = { regionState: RegionState ->
    generateSequence(regionState) { it.next() }
        .windowed(2)
        .takeWhile { (current, next) -> current != next }
        .withIndex()
        .last()
        .index + 2 // +1 because index start at 0 & +1 because we need the first that is same than previous
}

data class Position(val x: Int, val y: Int)

enum class Direction { east, south }

@Suppress("EqualsOrHashCode")
class RegionState private constructor(
    private val seaCucumberPositions: Map<Position, Direction>,
    private val height: Int,
    private val width: Int
                                     ) {
    companion object {
        fun parse(lines: List<String>) = RegionState(
            lines.asSequence().flatMapIndexed { y, line ->
                line.asSequence().mapIndexedNotNull { x, c ->
                    when (c) {
                        '>' -> Position(x, y) to Direction.east
                        'v' -> Position(x, y) to Direction.south
                        else -> null
                    }
                }
            }.toMap(),
            lines.size,
            lines[0].length,
                                                    )
    }

    fun Position.at(direction: Direction) = when (direction) {
        Direction.east -> copy(x = if (x >= width - 1) 0 else x + 1)
        Direction.south -> copy(y = if (y >= height - 1) 0 else y + 1)
    }

    fun next() = moveHerd(Direction.east).moveHerd(Direction.south)

    private fun moveHerd(direction: Direction) = with(seaCucumberPositions.mapKeys { (position, herd) ->
        if (herd == direction && !seaCucumberPositions.containsKey(position.at(direction))) position.at(direction)
        else position
    })

    private fun with(seaCucumberPositions: Map<Position, Direction>) = RegionState(seaCucumberPositions, height, width)

    override fun equals(other: Any?) = eq(other, { seaCucumberPositions })
}