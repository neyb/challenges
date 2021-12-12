package aoc2021.day11

import common.graph2d.*

fun main() = common.day(2021, 11, part1(100), part2)

val part1 = { nbStep: Int ->
    { lines: List<String> ->
        lines.map { it.asSequence() }
            .toOctopusMap()
            .let {
                (1..nbStep).fold(it to 0) { (currentMap, totalNbFlashes), step ->
                    currentMap.nextStep().let { (nextMap, nbFlashes) ->
                        nextMap to totalNbFlashes + nbFlashes
                    }
                }.second
            }
    }
}

val part2 = { lines: List<String> ->
    val octopusMap = lines.map { it.asSequence() }.toOctopusMap()

    generateSequence(1) { it + 1 }
        .runningFold(octopusMap to 0) { (octopusMap), step -> octopusMap.nextStep().map to step }
        .first { (octopusMap) -> octopusMap.allBlinked() }
        .second
}

typealias EnergyLevel = Int

fun Iterable<Sequence<Char>>.toOctopusMap() = OctopusMap2d(map { it.map { it.digitToInt() }.toList() })
class OctopusMap2d(private val map: Map2d<EnergyLevel>) {

    constructor(lines: List<List<EnergyLevel>>) : this(Map2d.ofLines(lines))

    data class NextStepResult(val map: OctopusMap2d, val nbFlashes: Int)

    fun nextStep(): NextStepResult {
        val flashed = mutableSetOf<Coordinate2d>()

        return map
            .mapValues { it + 1 }
            .edit { map ->
                generateSequence { map.nodes().filter { it.coordinate2d !in flashed && it.value > 9 }.toList() }
                    .takeWhile { it.size > 0 }
                    .flatMap { it }
                    .map { it.coordinate2d }
                    .onEach { flashed.add(it) }
                    .flatMap { it.neightbours(true) }
                    .forEach { coord -> map.mutateValue(coord) { it + 1 } }
            }
            .mapValues { if (it > 9) 0 else it }
            .let { NextStepResult(OctopusMap2d(it), flashed.size) }
    }

    fun allBlinked() = map.nodes().all { it.value == 0 }


}

