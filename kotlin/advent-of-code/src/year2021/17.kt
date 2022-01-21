package year2021.day17

import common.*
import kotlin.math.abs
import kotlin.math.max

fun main() = day(2021, 17, part1, part2) { //target area: x=119..176, y=-141..-84
    val matchEntire = Regex("""target area: x=([\d-]+)..([\d-]+), y=([\d-]+)..([\d-]+)""").matchEntire(readLines()[0])
    val (_, minX, maxX, minY, maxY) = matchEntire!!.groupValues
    Target(minX.toInt()..maxX.toInt(), minY.toInt()..maxY.toInt())
}


val part1 = { (_, yRange): Target -> (1..abs(yRange.start) - 1).sum() }

val part2 = { target: Target ->
    val touchTarget = { speed: Speed ->
        generateSequence(Probe(Position(0, 0), speed)) { it.next() }
            .takeWhile { with(it.position) { x <= target.xRange.endInclusive && y >= target.yRange.start } }
            .any { it.position in target }
    }

    val minXSpeed = generateSequence(1) { it + 1 }.first { (1..it).sum() >= target.xRange.start }
    val maxXSpeed = target.xRange.endInclusive
    val minYSpeed = target.yRange.start
    val maxYSpeed = abs(target.yRange.start) - 1

    (minXSpeed..maxXSpeed).asSequence()
        .flatMap { xSpeed -> (minYSpeed..maxYSpeed).asSequence().map { ySpeed -> Speed(xSpeed, ySpeed) } }
        .count(touchTarget)
}

data class Position(val x: Int, val y: Int) {
    fun next(speed: Speed) = copy(x + speed.x, y + speed.y)
}

data class Speed(val x: Int, val y: Int) {
    fun next() = copy(max(0, (x - 1)), y - 1)
}

data class Probe(val position: Position, val speed: Speed) {
    fun next() =
        copy(position.next(speed), speed.next())
}

data class Target(val xRange: IntRange, val yRange: IntRange) {
    operator fun contains(position: Position) = position.x in xRange && position.y in yRange
}

