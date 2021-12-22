package year2021.day15

import common.*
import common.graph2d.*

fun main() = day(2021, 15, part1, part2)

val part1 = { lines: List<String> ->
    val map = lines.asSequence()
        .flatMapIndexed { y, line ->
            line.asSequence().mapIndexed { x, c -> Node(Coordinate(x, y), null, c.digitToInt()) }
        }
        .toMap2d()

    val shortestPath = map.shortestPath(
        Coordinate(map.nodes.minOf { it.coordinate2d.x }, map.nodes.minOf { it.coordinate2d.y }),
        Coordinate(map.nodes.maxOf { it.coordinate2d.x }, map.nodes.maxOf { it.coordinate2d.y })
                                       ) ?: throw Exception("no path found")
    shortestPath
        .nodes.asSequence()
        .drop(1)
        .sumOf { it.weight }
}

val part2 = { lines: List<String> ->
    val flatMap = (0..4).asSequence()
        .flatMap { repeatY ->
            lines.map { (0..4).flatMap { repeatX -> it.map { (it.digitToInt() + repeatX + repeatY).let { if (it > 9) it - 9 else it } } } }
        }.toList()

    val map = flatMap
        .flatMapIndexed { y, l -> l.mapIndexed { x, i -> Node(Coordinate(x, y), null, i) } }
        .toMap2d()

    val shortestPath = map.shortestPath(
        Coordinate(map.nodes.minOf { it.coordinate2d.x }, map.nodes.minOf { it.coordinate2d.y }),
        Coordinate(map.nodes.maxOf { it.coordinate2d.x }, map.nodes.maxOf { it.coordinate2d.y })
                                       ) ?: throw Exception("no path found")
    shortestPath
        .nodes.asSequence()
        .drop(1)
        .sumOf { it.weight }
}

