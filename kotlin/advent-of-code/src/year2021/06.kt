package year2021.day6

import common.*
import kotlin.math.max

fun main() = run().forEach(::println)
val run = { day(2021, 6, populationAfter(80), populationAfter(256)) }

class LanternfishPopulation(
    populationByTimer: Map<Int, Long>,
    private val repops: List<Int>
                           ) {
    private val populationByTimerCircle = (0..max(repops.maxOf { it }, populationByTimer.maxOf { it.key }))
        .asSequence()
        .map { populationByTimer[it] ?: 0 }
        .toMutableList()

    private var shift = 0

    val population get() = populationByTimerCircle.sum()

    fun passADay() {
        val repopPopulation = populationByTimerCircle[shift]
        populationByTimerCircle[shift] = 0
        shift = shiftAfter(1)
        repops.forEach {
            shiftAfter(it).let {
                populationByTimerCircle[it] = populationByTimerCircle[it] + repopPopulation
            }
        }
    }

    private fun shiftAfter(nbDay: Int) = (shift + nbDay) % populationByTimerCircle.size

}

val populationAfter = { nbDays: Int ->
    { lines: List<String> ->
        lines.asSequence()
            .flatMap { it.split(",") }
            .groupingBy { it.toInt() }.eachCount()
            .let { LanternfishPopulation(it.mapValues { it.value.toLong() }, listOf(6, 8)) }
            .also { population -> repeat(nbDays) { population.passADay() } }
            .population
    }
}

//fun populationAfter(nbDays: Int) = { lines: List<String> ->
//    lines.asSequence()
//        .flatMap { it.split(",") }
//        .groupingBy { it.toInt() }.eachCount()
//        .let { LanternfishPopulation(it.mapValues { it.value.toLong() }, listOf(6, 8)) }
//        .also { population -> repeat(nbDays) { population.passADay() } }
//        .population
//}
