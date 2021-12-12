package aoc2021.day7

import common.day

fun main() = day(2021, 7, part1, part2)


val part1 = { lines: List<String> ->
    fun MutableMap<Int, Int>.move(from: Int, to: Int) = remove(from)?.also { put(to, (get(to) ?: 0) + it) }

    lines.asSequence().flatMap { it.split(",") }.groupingBy { it.toInt() }.eachCountTo(mutableMapOf())
        .let { numberByPosition ->
            var fuelConsumed = 0
            while (numberByPosition.size > 1) {
                val (minPosition, nbAtMinPosition) = numberByPosition.minByOrNull { it.key }!!
                val (maxPosition, nbAtMaxPosition) = numberByPosition.maxByOrNull { it.key }!!
                if (nbAtMinPosition < nbAtMaxPosition) fuelConsumed += numberByPosition.move(
                    minPosition,
                    minPosition + 1
                                                                                            )!!
                else fuelConsumed += numberByPosition.move(maxPosition, maxPosition - 1)!!
            }
            fuelConsumed
        }
}

data class PositionCrabsData(val nbCrabsval: Int, val fuelToConsume: Int) {
    companion object {
        val NoCrabs = PositionCrabsData(0, 0)
    }

    fun afterMove() = PositionCrabsData(nbCrabsval, fuelToConsume + nbCrabsval)

    operator fun plus(o: PositionCrabsData) =
        PositionCrabsData(o.nbCrabsval + nbCrabsval, o.fuelToConsume + fuelToConsume)
}

val part2 = { lines: List<String> ->
    fun MutableMap<Int, PositionCrabsData>.move(from: Int, to: Int) =
        remove(from)?.also { put(to, (get(to) ?: PositionCrabsData.NoCrabs) + it.afterMove()) }?.fuelToConsume

    lines.asSequence().flatMap { it.split(",") }
        .groupingBy { it.toInt() }.eachCount()
        .mapValuesTo(mutableMapOf()) { (_, v) -> PositionCrabsData(v, v) }.let { dataByPosition ->
            var fuelConsumed = 0
            while (dataByPosition.size > 1) {
                val (minPosition, dataAtMinPosition) = dataByPosition.minByOrNull { it.key }!!
                val (maxPosition, dataAtMaxPosition) = dataByPosition.maxByOrNull { it.key }!!

                if (dataAtMinPosition.fuelToConsume < dataAtMaxPosition.fuelToConsume)
                    fuelConsumed += dataByPosition.move(minPosition, minPosition + 1)!!
                else
                    fuelConsumed += dataByPosition.move(maxPosition, maxPosition - 1)!!
            }
            fuelConsumed
        }
}

