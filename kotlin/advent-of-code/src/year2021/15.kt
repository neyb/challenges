package year2021.day15

import common.*
import common.graph.graph2d.*

fun main() = run().forEach(::println)

val run = { day(2021, 15, part1, part2) }

val Map2d<Nothing?>.start get() = Coordinate(minX, minY)
val Map2d<Nothing?>.end get() = Coordinate(maxX, maxY)
fun Map2d<Nothing?>.path() = shortestPath(start, end) ?: throw Exception("no path found")

val part1 = { lines: List<String> ->
    val map = lines.asSequence().flatMapIndexed { y, line ->
        line.asSequence().mapIndexed { x, c ->
            val coordinate = Coordinate(x, y)
            coordinate to Node(coordinate, null, c.digitToInt())
        }
    }
        .toMap()
        .let { Map2d.ofNodes(it, withDiagonals = false, linksWeight = 0) }

    val path = map.path()
    path.nodes.asSequence().drop(1).sumOf { it.weight }
}

val part2 = { lines: List<String> ->
    val duplicated = (0..4).asSequence().flatMap { repeatY ->
        lines.map { (0..4).flatMap { repeatX -> it.map { (it.digitToInt() + repeatX + repeatY).let { if (it > 9) it - 9 else it } } } }
    }

    val map = duplicated.flatMapIndexed { y, l ->
        l.mapIndexed { x, i ->
            val coordinate = Coordinate(x, y)
            coordinate to Node(coordinate, null, i)
        }
    }.toMap()
        .let { Map2d.ofNodes(it, withDiagonals = false, linksWeight = 0) }

    val path = map.path()
    val sumOf = path.nodes.asSequence().drop(1).sumOf { it.weight }
    sumOf


}

