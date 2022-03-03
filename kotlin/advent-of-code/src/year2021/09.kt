package year2021.day9

import common.*

fun main() = run().forEach(::println)
val run = { day(2021, 9, part1, part2) }

typealias Height = Int

data class Coordinate(val x: Int, val y: Int) {
    val adjacent get() = sequenceOf(copy(x = x - 1), copy(x = x + 1), copy(y = y - 1), copy(y = y + 1))
}

class Area(val heights: List<List<Height>>) {
    data class Node(val coordinate: Coordinate, val height: Height)

    fun Node.adjacents() = coordinate.adjacent.mapNotNull { nodes[it] }

    val nodes by lazy {
        heights
            .asSequence()
            .flatMapIndexed { y, line -> line.asSequence().mapIndexed { x, height -> Node(Coordinate(x, y), height) } }
            .associateBy { it.coordinate }
    }

    val lowPoints by lazy { nodes.values.filter { (coordinate) -> isLowPoint(coordinate) }.toSet() }

    val bassins by lazy {
        lowPoints.map {
            bassinFrom(setOf(it), setOf(it))
        }
    }

    fun getHeight(coordinate: Coordinate) = heights.getOrNull(coordinate.y)?.getOrNull(coordinate.x)

    private fun isLowPoint(coordinate: Coordinate) = getHeight(coordinate)!!.let { height ->
        coordinate.adjacent.all { getHeight(it)?.let { adjHeight -> height < adjHeight } ?: true }
    }

    private tailrec fun bassinFrom(froms: Set<Node>, visited: Set<Node>): Set<Node> {
        val next = froms.asSequence()
            .flatMap { from -> from.adjacents().map { from to it } }
            .filter { (from, to) -> to.height != 9 && to.height > from.height && to !in visited }
            .map { (_, to) -> to }
            .toSet()
        return if (next.isEmpty())
            visited + next
        else
            bassinFrom(next, visited + next)
    }

}

val part1 = { lines: List<String> ->
    lines.map { it.asSequence().map { it.digitToInt() }.toList() }
        .let(::Area)
        .lowPoints
        .map { it.height + 1 }
        .sum()
}

val part2 = { lines: List<String> ->
    lines.map { it.asSequence().map { it.digitToInt() }.toList() }
        .let(::Area)
        .bassins
        .asSequence()
        .map { it.size }
        .sortedDescending()
        .take(3)
        .reduce(Int::times)
}