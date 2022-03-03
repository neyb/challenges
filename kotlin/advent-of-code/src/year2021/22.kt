package year2021.day22

import common.*
import kotlin.math.max
import kotlin.math.min

fun main() = run().forEach(::println)
val run = { day(2021, 22, part1, part2) { readText().parseInput() } }

typealias Input = List<Pair<Cuboid, Boolean>>

fun String.parseInput(): Input = split("\n").toList()
    .filter { it.isNotBlank() }
    .map {
        val regex = Regex("""(on|off) (.*)""")
        val (_, onoff, cube) = regex.matchEntire(it)?.groupValues ?: throw Exception("cannot parse $it as a cube")
        Cuboid.parse(cube) to (onoff == "on")
    }

val part1 = { input: Input ->
    input
        .asSequence()
        .filter { (cube) -> cube.run { sequenceOf(xRange, yRange, zRange) }.all { it in (-50..50) } }
        .fold(setOf<Cuboid>()) { cubes, (cube, on) ->
            cubes
                .flatMap { it - cube }
                .toSet()
                .let { if (on) it + cube else it }
        }
        .sumOf { it.size }
}

val part2 = { input: Input ->
    input
        .fold(setOf<Cuboid>()) { cubes, (cube, on) ->
            cubes
                .flatMap { it - cube }
                .toSet()
                .let { if (on) it + cube else it }
        }
        .sumOf { it.size.toLong() }
}

fun IntRange.before(other: IntRange) = first until other.first
fun IntRange.after(other: IntRange) = (other.last + 1)..last
val IntRange.size get() = last - first + 1
infix fun IntRange.cross(other: IntRange) = !(first > other.last || last < other.first)
fun IntRange.intersect(other: IntRange) = max(first, other.first)..min(last, other.last)
operator fun IntRange.contains(other: IntRange) = other.first in this && other.last in this

data class Cuboid(val xRange: IntRange, val yRange: IntRange, val zRange: IntRange) {
    companion object {
        fun parse(s: String) = s.split(",").map {
            it.split("=")[1].split("..").let { (from, to) -> from.toInt()..to.toInt() }
        }.let { (x, y, z) -> Cuboid(x, y, z) }
    }

    val size get() = xRange.size.toLong() * yRange.size.toLong() * zRange.size.toLong()

    fun cross(other: Cuboid) = xRange cross other.xRange && yRange cross other.yRange && zRange cross other.zRange

    operator fun contains(other: Cuboid) = other.xRange in xRange && other.yRange in yRange && other.zRange in zRange


    operator fun minus(other: Cuboid): Set<Cuboid> {
        fun Cuboid.split(
            getRange: Cuboid.() -> IntRange,
            copyWithRange: Cuboid.(IntRange) -> Cuboid
                        ): Pair<Set<Cuboid>, Cuboid> {
            val range = getRange()
            val otherRange = other.getRange()

            return sequenceOf(range.before(otherRange), range.after(otherRange))
                .filter { !it.isEmpty() }
                .map { copyWithRange(it) }
                .toSet() to copyWithRange(range.intersect(otherRange))
        }

        return when {
            !cross(other) -> setOf(this)
            this in other -> setOf()
            else -> sequenceOf<Pair<Cuboid.() -> IntRange, Cuboid.(IntRange) -> Cuboid>>(
                Pair({ xRange }, { copy(xRange = it) }),
                Pair({ yRange }, { copy(yRange = it) }),
                Pair({ zRange }, { copy(zRange = it) }),
                                                                                        )
                .fold(emptySet<Cuboid>() to this) { (cubes, rest), (get, copy) ->
                    rest.split(get, copy).let { cubes + it.first to it.second }
                }.first
        }
    }

    override fun toString() = "$xRange,$yRange,$zRange"
}

//operator fun Int.rangeTo(to: Int) = Range(this, to)
//data class Range(val from: Int, val to: Int)