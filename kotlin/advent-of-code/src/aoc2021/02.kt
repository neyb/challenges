package aoc2021.day2

import common.day

fun main() = day(2021, 2, { run(it, Submarine::part1Move) }, { run(it, Submarine::part2Move) })

data class Submarine(val x: Int = 0, val y: Int = 0, val aim: Int = 0) {
    fun part1Move(op: String, v: Int) = when (op) {
        "forward" -> copy(x = x + v)
        "down" -> copy(y = y + v)
        "up" -> copy(y = y - v)
        else -> throw Exception("unknow op :$op")
    }

    fun part2Move(op: String, v: Int) = when (op) {
        "forward" -> copy(x = x + v, y = y + aim * v)
        "down" -> copy(aim = aim + v)
        "up" -> copy(aim = aim - v)
        else -> throw Exception("unknow op :$op")
    }
}

fun run(lines: List<String>, move: (Submarine, op: String, v: Int) -> Submarine) = lines.asSequence()
    .map { it.split(" ") }
    .fold(Submarine()) { submarine, (op, v) -> move(submarine, op, v.toInt()) }
    .let { (x, y) -> x * y }