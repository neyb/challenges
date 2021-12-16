package aoc2021.day14

import common.*

fun main() = day(2021, 14, run(10), run(40))

// polymer is just pair (string of 2 chars) counts & last character (being a string of 1 char)
typealias Polymer = Map<String, Long>
typealias MutablePolymer = MutableMap<String, Long>

fun MutablePolymer.add(pair: String, nb: Long) = this.compute(pair) { _, existingNb -> (existingNb ?: 0) + nb }

fun Polymer.score() = asSequence()
    .groupingBy { it.key.first() }
    .fold(0L) { nb, e -> nb + e.value }
    .values.run { maxOf { it } - minOf { it } }

val run = { nbStep: Int ->
    { lines: List<String> ->
        val (startPolymerLines, insertionsLines) = lines.split { it.isBlank() }

        val startPolymer = startPolymerLines[0].asSequence()
            .windowed(2, partialWindows = true) { l -> l.joinToString("") }
            .groupingBy { it }
            .eachCount()
            .mapValues { (_, v) -> v.toLong() }

        // insertion is mappings of 1 pair to 2 pairs : ex: CH -> B <=> ch -> cb + bh
        val insertions = insertionsLines.asSequence()
            .mapNotNull { Regex("""(.)(.) -> (.)""").matchEntire(it)?.destructured }
            .map { (a, b, target) -> "$a$b" to listOf("$a$target", "$target$b") }
            .toMap()


        fun Polymer.nextPolymer(): Polymer = asSequence().fold(mutableMapOf<String, Long>()) { polymer, (pair, nb) ->
            polymer.apply {
                insertions[pair]
                    ?.run { forEach { polymer.add(it, nb) } }
                    ?: add(pair, nb)
            }
        }

        (1..nbStep).fold(startPolymer) { pol, _ -> pol.nextPolymer() }.score()
    }
}
