package year2021.day11

import common.graph.graph2d.*

fun main() = common.day(2021, 11, part1(100), part2) {
    OctopusMap2d(useLines { Map2d.parseLinesWithItem(it) { it.digitToInt() } })
}

val part1 = { nbStep: Int ->
    { octopusMap: OctopusMap2d ->
        (1..nbStep).fold(octopusMap to 0) { (currentMap, totalNbFlashes), _ ->
            currentMap.nextStep().let { (nextMap, nbFlashes) ->
                nextMap to totalNbFlashes + nbFlashes
            }
        }.second
    }
}

val part2 = { octopusMap: OctopusMap2d ->
    generateSequence(1) { it + 1 }
        .runningFold(octopusMap to 0) { (octopusMap), step -> octopusMap.nextStep().map to step }
        .first { (octopusMap) -> octopusMap.allBlinked() }
        .second
}

typealias EnergyLevel = Int

class OctopusMap2d(private val map: Map2d<EnergyLevel>) {
    data class NextStepResult(val map: OctopusMap2d, val nbFlashes: Int)

    fun nextStep(): NextStepResult {
        val flashed = mutableSetOf<Coordinate>()

        return map
            .mapValues { it + 1 }
            .edit { map ->
                generateSequence {
                    map.filter { (coordinate2d, node) -> coordinate2d !in flashed && node.value > 9 }.toList()
                }
                    .takeWhile { it.size > 0 }
                    .flatMap { it }
                    .map { (coordinate2d) -> coordinate2d }
                    .onEach { flashed.add(it) }
                    .flatMap { it.neightbours(true) }
                    .forEach { coord -> map.computeIfPresent(coord) { _, node -> node.copy(value = node.value + 1) } }
            }
            .mapValues { if (it > 9) 0 else it }
            .let { NextStepResult(OctopusMap2d(it), flashed.size) }
    }

    fun allBlinked() = map.nodes.asSequence().all { it.value == 0 }


}

