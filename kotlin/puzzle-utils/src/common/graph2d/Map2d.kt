package common.graph2d

fun <T> Iterable<Node<T>>.toMap2d(): Map2d<T> = Map2d.of(associateBy({ it.coordinate2d }, { it.value }))
fun <T> Sequence<Node<T>>.toMap2d(): Map2d<T> = Map2d.of(associateBy({ it.coordinate2d }, { it.value }))

@JvmRecord
data class Node<out T>(val coordinate2d: Coordinate2d, val value: T)

interface Map2d<out T> {

    fun nodes(): Sequence<Node<T>>

    operator fun get(coordinate2d: Coordinate2d): T?
    operator fun contains(coordinate2d: Coordinate2d): Boolean
    fun getNode(coordinate2d: Coordinate2d): Node<T>?

    fun filter(predicate: (Node<T>) -> Boolean): Map2d<T>
    fun <R> map(
        mutation: (Node<T>) -> Node<R>,
        mergeValues: (R, R) -> R = { a: R, b: R -> (throw Exception("conflict during node mapping : maybe pass a merge function ?")) }
               ): Map2d<R>

    fun <R> mapValues(mutation: (T) -> R): Map2d<R>

    fun edit(mutation: (MutableMap<Coordinate2d, @UnsafeVariance T>) -> Unit): Map2d<T>

    fun area(
        startingPoint: Coordinate2d,
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
        private val content = content.toMap()

        override fun nodes() = content.asSequence().map { Node(it.key, it.value) }

        override fun get(coordinate2d: Coordinate2d) = content[coordinate2d]
        override fun getNode(coordinate2d: Coordinate2d) = get(coordinate2d)?.let { Node(coordinate2d, it) }
        override fun contains(coordinate2d: Coordinate2d) = content.containsKey(coordinate2d)

        override fun filter(predicate: (Node<T>) -> Boolean) = nodes().filter(predicate).toMap2d()

        override fun <R> map(mutation: (Node<T>) -> Node<R>, mergeValues: (R, R) -> R): Map2d<R> = nodes()
            .map(mutation)
            .groupingBy { it.coordinate2d }
            .reduce { coord, nodeA, nodeB -> Node(coord, mergeValues(nodeA.value, nodeB.value)) }
            .mapValues { (_, node) -> node.value }
            .let(::Impl)

        override fun <R> mapValues(mutation: (T) -> R) =
            nodes().map { Node(it.coordinate2d, mutation(it.value)) }.toMap2d()

        override fun edit(mutation: (MutableMap<Coordinate2d, T>) -> Unit) = of(content.toMutableMap().also(mutation))

        override fun area(
            startingPoint: Coordinate2d,
            withDiagonal: Boolean,
            linkFilter: (from: Node<T>, to: Node<T>) -> Boolean
                         ) = areaFrom(
            setOf(startingPoint),
            withDiagonal,
            linkFilter
                                     )

        private tailrec fun areaFrom(
            froms: Set<Coordinate2d>,
            withDiagonal: Boolean,
            linkFilter: (from: Node<T>, to: Node<T>) -> Boolean,
            visited: Set<Coordinate2d> = froms,
                                    ): Set<Node<T>> {

            val next = froms.asSequence()
                .flatMap { from -> from.neightbours(withDiagonal).map { from to it } }
                .filter { (_, to) -> to !in visited }
                .filter { (_, to) -> to in this }
                .filter { (from, to) -> linkFilter(getNode(from)!!, getNode(to)!!) }
                .map { (_, to) -> to }
                .toSet()

            return if (next.isEmpty()) visited.asSequence().map { getNode(it)!! }.toSet()
            else areaFrom(next, withDiagonal, linkFilter, visited + next)
        }

    }
}

