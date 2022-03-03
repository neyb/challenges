package year2015.day6

import common.*
import java.lang.Integer.max

fun main() = run().forEach(::println)
fun run() = day(2015, 6, part1, part2) {
    val regex = Regex("""(turn on|turn off|toggle) (.*) through (.*)""")

    useLines {
        it.map { line ->
            val (_, action, from, to) = regex.matchEntire(line)!!.groupValues

            Instruction(
                when (action) {
                    "turn on" -> Action.turnOn
                    "turn off" -> Action.turnOff
                    "toggle" -> Action.toggle
                    else -> throw Exception("...$action ???")
                },
                Rectangle(Point.parse(from), Point.parse(to))
                       )
        }.toList()
    }
}

val part1 = { instructions: List<Instruction> ->
    run(instructions, false) { action, v ->
        when (action) {
            Action.turnOn -> true
            Action.turnOff -> false
            Action.toggle -> !v
        }
    }.count { it }
}

val part2 = { instructions: List<Instruction> ->
    run(instructions, 0) { action, v ->
        when (action) {
            Action.turnOn -> v + 1
            Action.turnOff -> max(0, v - 1)
            Action.toggle -> v + 2
        }
    }.sum()
}

fun <T> run(instructions: List<Instruction>, initial: T, update: (Action, T) -> T) =
    instructions
        .fold((0..999).map { MutableList(1000) { initial } }) { map, instr ->
            instr.points.fold(map) { _, point ->
                map.also { it[point.x][point.y] = update(instr.action, it[point.x][point.y]) }
            }
        }.asSequence().flatten()


enum class Action { turnOn, turnOff, toggle }

data class Instruction(val action: Action, val rect: Rectangle) {
    val points get() = rect.x.asSequence().flatMap { x -> rect.y.asSequence().map { y -> Point(x, y) } }
}

data class Rectangle(val from: Point, val to: Point) {
    val x = from.x..to.x
    val y = from.y..to.y
}

data class Point(val x: Int, val y: Int) {
    companion object {
        fun parse(s: String) = s.split(",").let { (from, to) -> Point(from.toInt(), to.toInt()) }
    }
}

