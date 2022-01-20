package common

import java.util.*
import java.util.Comparator.comparing

fun <T> explore(start: T, next: (T) -> Sequence<T>) = ExploreSequence.explore(start, next)

class ExploreSequence<T> private constructor(
    val start: T,
    val next: (T) -> Sequence<T>,
    val createQueue: () -> Queue<T> = { ArrayDeque() },
    val filter: (T) -> Boolean = { true },
                                            ) : Sequence<T> {

    companion object {
        fun <T> explore(start: T, next: (T) -> Sequence<T>) = ExploreSequence(start, next)
    }

    fun filterExploration(filter: (T) -> Boolean) = ExploreSequence(start, next, createQueue, filter)

    fun <Comparing : Comparable<Comparing>> minimizing(keyExtractor: (T) -> Comparing) =
        ExploreSequence(start, next, { PriorityQueue(comparing(keyExtractor)) }, filter)

    override fun iterator(): Iterator<T> = with(createQueue()) {
        generateSequence(start) { poll() }
            .filter(filter)
            .onEach { addAll(next(it)) }
    }.iterator()
}