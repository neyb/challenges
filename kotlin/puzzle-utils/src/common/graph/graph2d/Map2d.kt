package common.graph.graph2d

import common.graph.*

typealias Graph<T> = common.graph.Graph<Coordinate, T>
typealias Node<T> = common.graph.Node<Coordinate, T>
typealias Link<T> = common.graph.Link<Coordinate, T>

class Map2d<T> private constructor(val graph: Graph<T>) : Graph<T> by graph {
    companion object {
        fun <T> ofNodes(nodes: Collection<Node<T>>, withDiagonals: Boolean, linksWeight: Int = 1): Map2d<T> =
            ofNodes(nodes.associateByTo(HashMap(nodes.size)) { it.id }, withDiagonals, linksWeight)

        fun <T> ofNodes(nodesById: Map<Coordinate, Node<T>>, withDiagonals: Boolean, linksWeight: Int = 1): Map2d<T> {
            val content = nodesById.mapValues { (_, node) ->
                val links = node.id.neightbours(withDiagonals)
                    .mapNotNull(nodesById::get)
                    .map { Link(node, it, linksWeight) }
                    .toCollection(ArrayList(if (withDiagonals) 8 else 4))
                NodeAndLinks(node, links)
            }
            return Map2d(Graph(content))
        }

        fun <T> withoutLinks(nodesById: Map<Coordinate, Node<T>>): Map2d<T> =
            Map2d(Graph(nodesById.mapValues { (_, node) -> NodeAndLinks(node, emptyList()) }))

        fun <T> parseLinesWithItem(lines: Sequence<String>, withDiagonals: Boolean, parseItem: (Char) -> T) =
            parseLinesWithNodes(lines, withDiagonals) { coord, c -> Node(coord, parseItem(c)) }

        fun <T> parseLinesWithNodes(
            lines: Sequence<String>,
            withDiagonals: Boolean,
            parseNode: (Coordinate, Char) -> Node<T>
                                   ) = lines
            .flatMapIndexed { y, line -> line.mapIndexed { x, char -> parseNode(Coordinate(x, y), char) } }
            .let { ofNodes(it.toList(), withDiagonals) }
    }

    val minX by lazy { graph.nodes.minOf { it.id.x } }
    val minY by lazy { graph.nodes.minOf { it.id.y } }
    val maxX by lazy { graph.nodes.maxOf { it.id.x } }
    val maxY by lazy { graph.nodes.maxOf { it.id.y } }

    override fun <R> mapValues(mutation: (T) -> R) = Map2d(graph.mapValues(mutation))

    fun edit(mutation: (MutableMap<Coordinate, Node<T>>) -> Unit): Map2d<T> =
        graph.nodes.associateByTo(mutableMapOf()) { it.id }
            .also(mutation)
            .let { updatedNodesById -> Map2d(mapNodes { updatedNodesById[it.id]!! }) }
}