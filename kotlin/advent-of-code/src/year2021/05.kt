package year2021.day5

import common.day
import kotlin.math.max
import kotlin.math.min

fun main() = day(2021, 5, commonRun { it.pointsPart1 }, commonRun { it.pointsPart2 })

data class Vent(val from: Point, val to: Point) {
    companion object {
        fun parse(s: String) = s.split(" -> ").let { (from, to) -> Vent(Point.parse(from), Point.parse(to)) }
    }

    val pointsPart1
        get() = when {
            from.x == to.x -> (min(from.y, to.y)..max(from.y, to.y)).map { Point(from.x, it) }
            from.y == to.y -> (min(from.x, to.x)..max(from.x, to.x)).map { Point(it, from.y) }
            else -> emptyList()
        }

    val pointsPart2
        get() = (from..to step Point(
            x = when {
                from.x < to.x -> 1
                from.x > to.x -> -1
                else -> 0
            },
            y = when {
                from.y < to.y -> 1
                from.y > to.y -> -1
                else -> 0
            }
                                    )).toList()

}

data class Point(val x: Int, val y: Int) {
    companion object {
        fun parse(s: String) = s.split(",").let { (x, y) -> Point(x.toInt(), y.toInt()) }
    }

    operator fun plus(other: Point) = Point(x + other.x, y + other.y)

    operator fun rangeTo(other: Point) = PointProgression(this, other)
}

class PointProgression(val from: Point, val to: Point, val step: Point = Point(1, 1)) : Iterable<Point> {
    private inner class PointIterator() : Iterator<Point> {
        private var current: Point = from

        override fun hasNext() = when {
            step.x > 0 -> current.x <= to.x
            step.x < 0 -> current.x >= to.x
            else -> true
        } && when {
            step.y > 0 -> current.y <= to.y
            step.y < 0 -> current.y >= to.y
            else -> true
        }

        override fun next() = current.also { current += step }
    }

    override fun iterator(): Iterator<Point> = PointIterator()

    infix fun step(step: Point) = PointProgression(from, to, step)
}

class VentsMap(val lines: List<Vent>) {
    fun pointsValue(pointsOfVent: (Vent) -> List<Point>): Map<Point, Int> =
        lines.asSequence().flatMap(pointsOfVent).groupBy { it }.mapValues { (_, points) -> points.size }

}

fun commonRun(pointsOfVent: (Vent) -> List<Point>) = { lines: List<String> ->
    val pointsValue = lines.map(Vent.Companion::parse).let { VentsMap(it) }.pointsValue(pointsOfVent)
    pointsValue.values.count { it >= 2 }
}


