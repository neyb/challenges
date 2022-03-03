package year2021.day20

import common.*
import common.graph.graph2d.*

fun main() = run().forEach(::println)
fun run() = day(2021, 20, part1, part2) { Input.parse(readLines()) }

val runUntil = { nb: Int ->
    { (algo, originalImage): Input ->
        (1..nb).asSequence().fold(originalImage) { image, _ -> image.improve(algo) }.count()
    }
}

val part1 = runUntil(2)

val part2 = runUntil(50)

data class Input(val algo: Algo, val image: Image) {
    companion object {
        fun parse(lines: List<String>) = lines.split { it.isBlank() }.let { (algoLines, imageLines) ->
            Input(
                Algo(algoLines.joinToString("")),
                Image(Map2d.parseLinesWithItem(imageLines.asSequence(), false) { it == '#' }, false)
                 )
        }
    }
}

typealias Signature = Int

class Algo(s: String) {
    private val b = s.map { it == '#' }
    fun compute(signature: Signature) = b[signature]
}

class Image(private val map: Map2d<Boolean>,private val defaultValue: Boolean) {

    operator fun get(coordinate: Coordinate) = map.findNode(coordinate)?.data ?: defaultValue

    fun signature(coordinate: Coordinate) =
        sequenceOf(-1, 0, 1).flatMap { sequenceOf(it to -1, it to 0, it to 1) }
            .map { (ydiff, xdiff) -> coordinate.copy(x = coordinate.x + xdiff, y = coordinate.y + ydiff) }
            .map { get(it) }
            .fold(0) { r, b -> r * 2 + if (b) 1 else 0 }

    fun improve(algo: Algo) = Image(
        (map.minX - 1..map.maxX + 1).asSequence()
            .flatMap { x -> (map.minY - 1..map.maxY + 1).map { Coordinate(x, it) } }
            .map { it to Node(it, algo.compute(signature(it))) }
            .toMap()
            .let { Map2d.withoutLinks(it) },
        algo.compute(if (defaultValue) 511 else 0)
                                   )

    override fun toString() = with(map) {
        (minY..maxY).asSequence().map { y ->
            (minX..maxX).asSequence().map { x ->
                if (get(Coordinate(x, y))) '#' else '.'
            }.joinToString("")
        }.joinToString("\n")
    }

    fun count() = map.nodes.count { it.data }
}