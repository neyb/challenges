package common.graph2d

fun <T> Iterable<Node<T>>.toMap2d(): Map2d<T> = Map2d.of(associateBy({ it.coordinate2d }, { it.value }))
fun <T> Sequence<Node<T>>.toMap2d(): Map2d<T> = Map2d.of(associateBy({ it.coordinate2d }, { it.value }))

@JvmRecord
data class Node<out T>(val coordinate2d: Coordinate2d, val value: T)

interface Map2d<out T> {

    fun nodes(): Sequence<Node<T>>

    operator fun get(x: Int, y: Int) = get(Coordinate2d(x, y))
    operator fun get(coordinate2d: Coordinate2d): T?

    fun <R> map(mutation: (Node<T>) -> Node<R>): Map2d<R>
    fun <R> mapValues(mutation: (T) -> R): Map2d<R>

    fun edit(mutation: (MutableMap2d<@UnsafeVariance T>) -> Unit): Map2d<T>

    fun area(
        from: Coordinate2d,
        withDiagonal: Boolean = false,
        linkFilter: (from: Node<T>, to: Node<T>) -> Boolean
            ): Set<Node<T>>

    companion object {
        fun <T> of(content: Map<Coordinate2d, T>): Map2d<T> = Impl(content)
        fun <T> ofLines(lines: Iterable<Iterable<T>>): Map2d<T> = lines.asSequence()
            .flatMapIndexed { y, line -> line.mapIndexed { x, v -> Coordinate2d(x, y) to v } }
            .toMap()
            .let(::of)
    }

    private class Impl<T>(content: Map<Coordinate2d, T>) : Map2d<T> {
        private val content = content.toMutableMap()

        override fun nodes() = content.asSequence().map { Node(it.key, it.value) }

        override fun get(x: Int, y: Int) = get(Coordinate2d(x, y))
        override fun get(coordinate2d: Coordinate2d) = content[coordinate2d]

        override fun <R> mapValues(mutation: (T) -> R) = map { Node(it.coordinate2d, mutation(it.value)) }

        override fun <R> map(mutation: (Node<T>) -> Node<R>) = nodes().map(mutation).toMap2d()

        override fun edit(mutation: MutableMap2d<T>.() -> Unit) = Impl(content).apply(mutation)

        override fun area(
            from: Coordinate2d,
            withDiagonal: Boolean,
            linkFilter: (from: Node<T>, to: Node<T>) -> Boolean
                         ): Set<Node<T>> {
            TODO("Not yet implemented")
        }

        private tailrec fun areaFrom(
            froms: Set<Coordinate2d>,
            withDiagonal: Boolean = false,
            linkFilter: (from: Node<T>, to: Node<T>) -> Boolean
            visited: Set<Coordinate2d> = froms
                                    ): Set<Node<T>> {

            val next = froms.asSequence()
                .flatMap { from -> from.neightbours(withDiagonal).map { from to it } }
                .filter { (from, to) -> to !in visited }
                .map { (from, to) ->  }
                    && linkFilter(from, to) }
                .map { (_, to) -> to }
                .toSet()
            return if (next.isEmpty())
                visited + next
            else
                bassinFrom(next, visited + next)
        }

    }
}

