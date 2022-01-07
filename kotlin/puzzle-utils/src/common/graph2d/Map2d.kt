package common.graph2d

import common.algo.*
import kotlin.math.abs

fun <T> Iterable<Node<T>>.toMap2d(): Map2d<T> = Map2d(associateBy({ it.coordinate2d }))
fun <T> Sequence<Node<T>>.toMap2d(): Map2d<T> = Map2d(associateBy({ it.coordinate2d }))

class Map2d<T>(private val nodesByCoord: Map<Coordinate, Node<T>>) {

    companion object {
        fun <T> ofValueLines(lines: Iterable<Iterable<T>>): Map2d<T> = lines.asSequence()
            .flatMapIndexed { y, line -> line.mapIndexed { x, value -> Node(Coordinate(x, y), value) } }.toMap2d()

        fun ofWeightLines(lines: Iterable<Iterable<Int>>): Map2d<Nothing?> = lines.asSequence()
            .flatMapIndexed { y, line -> line.mapIndexed { x, weight -> Node(Coordinate(x, y), null, weight) } }
            .toMap2d()
    }


    //    private val nodesByCoord = content.mapValues { (coord, value) -> Node(coord, value) }
    val nodes by lazy { nodesByCoord.values }

    val minX by lazy { nodes.minOf { it.coordinate2d.x } }
    val minY by lazy { nodes.minOf { it.coordinate2d.y } }
    val maxX by lazy { nodes.maxOf { it.coordinate2d.x } }
    val maxY by lazy { nodes.maxOf { it.coordinate2d.y } }

    fun findValue(coordinate2d: Coordinate) = nodesByCoord[coordinate2d]?.value
    fun getValue(coordinate2d: Coordinate) =
        nodesByCoord[coordinate2d]?.value ?: throw Exception("$coordinate2d not in map")

    fun getNode(coordinate2d: Coordinate) = findNode(coordinate2d) ?: throw Exception("$coordinate2d not in map")
    fun findNode(coordinate2d: Coordinate) = nodesByCoord[coordinate2d]
    operator fun contains(coordinate2d: Coordinate) = nodesByCoord.containsKey(coordinate2d)

    fun filter(predicate: (Node<T>) -> Boolean) = nodes.asSequence().filter(predicate).toMap2d()

    fun <R> map(
        mutation: (Node<T>) -> Node<R>,
        mergeValues: (R, R) -> R = { a: R, b: R -> (throw Exception("conflict during node mapping : maybe pass a merge function ?")) }
               ): Map2d<R> = nodes.asSequence().map(mutation).groupingBy { it.coordinate2d }
        .reduce { coord, nodeA, nodeB -> Node(coord, mergeValues(nodeA.value, nodeB.value)) }.let(::Map2d)

    fun <R> mapValues(mutation: (T) -> R) =
        nodes.asSequence().map { Node(it.coordinate2d, mutation(it.value)) }.toMap2d()

    fun edit(mutation: (MutableMap<Coordinate, Node<T>>) -> Unit) = Map2d(nodesByCoord.toMutableMap().also(mutation))

    fun area(
        startingPoint: Coordinate, withDiagonal: Boolean = false, linkFilter: (from: Node<T>, to: Node<T>) -> Boolean
            ) = areaFrom(
        setOf(startingPoint), withDiagonal, linkFilter
                        )

    private tailrec fun areaFrom(
        froms: Set<Coordinate>,
        withDiagonal: Boolean,
        linkFilter: (from: Node<T>, to: Node<T>) -> Boolean,
        visited: Set<Coordinate> = froms,
                                ): Set<Node<T>> {

        val next = froms.asSequence().flatMap { from -> from.neightbours(withDiagonal).map { from to it } }
            .filter { (_, to) -> to !in visited }.filter { (_, to) -> to in this }
            .filter { (from, to) -> linkFilter(getNode(from), getNode(to)) }.map { (_, to) -> to }.toSet()

        return if (next.isEmpty()) visited.asSequence().map { getNode(it) }.toSet()
        else areaFrom(next, withDiagonal, linkFilter, visited + next)
    }

    fun shortestPath(
        start: Coordinate,
        end: Coordinate,
        heuristic: (Node<T>) -> Int = { it.coordinate2d.run { abs(y - end.y) + abs(x - end.x) } },
                    ): Path<T>? {
        val bestPathTo = hashMapOf<Coordinate, Path<T>>()
        return explore(Path.from(getNode(start))) { path -> path.end.neightbours().map { path + it } }
            .minimizing { it.weight(heuristic) }
            .filterExploration { path -> bestPathTo[path.end.coordinate2d]?.let { it == path } ?: true }
            .onEach { path ->
                val alreadyExistingPath = bestPathTo[path.end.coordinate2d]
                if (alreadyExistingPath == null || alreadyExistingPath.weight() > path.weight())
                    bestPathTo[path.end.coordinate2d] = path
            }
            .firstOrNull { it.end.coordinate2d == end }
    }

    private fun Node<T>.neightbours(withDiagonal: Boolean = false) =
        coordinate2d.neightbours(withDiagonal).mapNotNull { findNode(it) }
}

