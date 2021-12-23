package year2021.day20

import common.*
import common.graph2d.*
import java.util.Comparator.comparing

fun main() = day(2021, 20, part1, part2) { Input.parse(readLines()) }

val run = { nb: Int ->
    { (algo, originalImage): Input ->
        (1..nb).asSequence().fold(originalImage) { image, _ -> image.improve(algo) }.count()
    }
}

val part1 = run(2)

val part2 = run(50)

data class Input(val algo: Algo, val image: Image) {
    companion object {
        fun parse(lines: List<String>) = lines.split { it.isBlank() }.let { (algoLines, imageLines) ->
            Input(Algo.from(algoLines.joinToString("")), Image.parse(imageLines))
        }
    }
}

typealias Signature = Int

class Algo(s: String) {
    companion object {
        fun from(s: String) = Algo(s)
    }

    private val b = s.map { it == '#' }
    fun compute(signature: Signature) = b[signature]
}

class Image(val map: Map2d<Boolean>, val defaultValue: Boolean) {
    companion object {
        fun parse(lines: List<String>) = Image(
            lines.asSequence().flatMapIndexed { y, line ->
                line.mapIndexed { x, c ->
                    Node(Coordinate(x, y), c == '#')
                }
            }.toMap2d(), false
                                              )
    }

    operator fun get(coordinate: Coordinate) = map.findValue(coordinate) ?: defaultValue

    fun signature(coordinate: Coordinate) = (coordinate.neightbours(true) + coordinate)
        .sortedWith(comparing<Coordinate, Int> { it.y }.thenComparing(comparing { it.x }))
        .map(this::get)
        .fold(0) { r, b -> r * 2 + if (b) 1 else 0 }

    fun improve(algo: Algo) = Image(
        map.nodes.asSequence()
            .map { it.coordinate2d }
            .flatMap { it.neightbours(true) }
            .distinct()
            .map { Node(it, algo.compute(signature(it))) }
            .toMap2d(),
        algo.compute(if (defaultValue) 511 else 0)
                                   )

    override fun toString() = with(map) {
        (minY..maxY).asSequence().map { y ->
            (minX..maxX).asSequence().map { x ->
                if (getValue(Coordinate(x, y))) '#' else '.'
            }.joinToString("")
        }.joinToString("\n")
    }

    fun count() = map.nodes.count { it.value }
}