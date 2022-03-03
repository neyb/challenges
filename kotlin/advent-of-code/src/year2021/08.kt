package year2021.day8

import common.day

fun main() = run().forEach(::println)
val run = { day(2021, 8, part1, part2) }

data class Signal(val segments: Set<Char>) {
    companion object {
        fun parse(s: String) = Signal(s.asSequence().toSet())
    }

    val nbSegments = segments.size

    operator fun contains(signal: Signal) = segments.containsAll(signal.segments)
}

class Patterns(val patterns: Set<Signal>) {
    fun deduceValues() = DeduceState(patterns)
        .deduce(1) { it.nbSegments == 2 }
        .deduce(7) { it.nbSegments == 3 }
        .deduce(4) { it.nbSegments == 4 }
        .deduce(8) { it.nbSegments == 7 }
        .deduce(9) { it.nbSegments == 6 && signal(4) in it }
        .deduce(3) { it.nbSegments == 5 && signal(1) in it }
        .deduce(5) { it.nbSegments == 5 && it in signal(9) }
        .deduce(2) { it.nbSegments == 5 }
        .deduce(6) { it.nbSegments == 6 && signal(5) in it }
        .deduce(0) { true }.deducedSignals.asSequence()
        .map { it.value to it.key }
        .toMap()

    private class DeduceState(patterns: Set<Signal>) {
        val deducedSignals = mutableMapOf<Byte, Signal>()
        val remaining = patterns.toMutableSet()
        fun deduce(value: Byte, predicate: MutableMap<Byte, Signal>.(Signal) -> Boolean) = apply {
            val signal = remaining.single { deducedSignals.predicate(it) }
            remaining.remove(signal)
            deducedSignals.put(value, signal)
        }
    }

    private fun Map<Byte, Signal>.signal(value: Byte) = getValue(value)
}

data class Entry(val patterns: Patterns, val outputSignals: List<Signal>) {
    companion object {
        private fun String.parseSignals() = splitToSequence(" ").map(Signal::parse)
        fun parse(s: String) =
            s.split(" | ")
                .let { (patterns, output) ->
                    Entry(
                        Patterns(patterns.parseSignals().toSet()),
                        output.parseSignals().toList()
                         )
                }
    }

    fun outputValue() = with(patterns.deduceValues()) {
        outputSignals.asSequence().map(::getValue).fold(0) { acc, value -> acc * 10 + value }
    }

}

val part1 = { lines: List<String> ->
    lines
        .asSequence()
        .map(Entry::parse)
        .map { (_, output) -> output.count { listOf(2, 4, 3, 7).contains(it.nbSegments) } }
        .sum()
}

val part2 = { lines: List<String> ->
    lines.asSequence()
        .map(Entry::parse)
        .map { it.outputValue() }
        .sum()
}

