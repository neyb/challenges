package common.graph2d

import common.graph2d.Map2d.Companion.toMap2d

interface MutableMap2d<T> : Map2d<T> {
    operator fun set(coordinate2d: Coordinate2d, value: T)
    fun mutateValue(coordinate2d: Coordinate2d, mutation: (T) -> T): MutableMap2d<T>

    private class Impl<T>(content: Map<Coordinate2d, T>) : MutableMap2d<T> {
        private val content = content.toMutableMap()

        override fun nodes() = content.asSequence().map { Node(it.key, it.value) }

        override fun get(x: Int, y: Int) = get(Coordinate2d(x, y))
        override fun get(coordinate2d: Coordinate2d) = content[coordinate2d]

        override fun set(coordinate2d: Coordinate2d, value: T) {
            content[coordinate2d] = value
        }

        override fun <R> mapValues(mutation: (T) -> R) = map { Node(it.coordinate2d, mutation(it.value)) }

        override fun mutateValue(coordinate2d: Coordinate2d, mutation: (T) -> T) = apply {
            content.computeIfPresent(coordinate2d) { coor, v -> mutation(v) }
        }

        override fun <R> map(mutation: (Node<T>) -> Node<R>) = nodes().map(mutation).toMap2d()

        override fun edit(mutation: MutableMap2d<T>.() -> Unit) = Impl(content).apply(mutation)
    }
}

