package common.graph

import common.*

data class NodeAndLinks<Id, Data>(val node: Node<Id, Data>, val links: Collection<Link<Id, Data>>) {
    fun <NewData> mapNodes(mutation: (Node<Id, Data>) -> Node<Id, NewData>) =
        NodeAndLinks(mutation(node), links.map { it.mapNodes(mutation) })
}

interface Graph<Id, Data> {

    companion object {
        operator fun <Id, Data> invoke(content: Map<Id, NodeAndLinks<Id, Data>>): Graph<Id, Data> = Impl(content)

        operator fun <Id, Data> invoke(links: Collection<Link<Id, Data>>): Graph<Id, Data> =
            invoke(links.groupBy { it.from.id }.mapValues { (_, links) -> NodeAndLinks(links.first().from, links) })
    }

    val nodes: Collection<Node<Id, Data>>
    fun findNode(id: Id): Node<Id, Data>?

    fun <NewData> mapNodes(mutation: (Node<Id, Data>) -> Node<Id, NewData>): Graph<Id, NewData>
    fun <NewData> mapValues(mutation: (Data) -> NewData): Graph<Id, NewData>

    fun allPathsWithAllNodes(): Collection<Path<Id, Data>>
    fun shortestPath(start: Id, end: Id, heuristic: (Node<Id, Data>) -> Int = { 0 }): Path<Id, Data>?

    private class Impl<Id, Data>(
        val content: Map<Id, NodeAndLinks<Id, Data>>,
                                ) : Graph<Id, Data> {
        init {
            if (debug) {
                val linkNodesNotInNodes = links
                    .flatMap { sequenceOf(it.from, it.to) }
                    .filter { it.id !in content }
                    .toSet()
                if (linkNodesNotInNodes.isNotEmpty()) throw Exception("some links contains nodes not in nodes : $linkNodesNotInNodes")
            }
        }

        override val nodes by lazy { content.values.map { it.node } }
        private val links get() = content.values.asSequence().flatMap { it.links }

        override fun findNode(id: Id) = content[id]?.node
        operator fun contains(id: Id) = content.containsKey(id)

        fun group(from: Id, linkFilter: (Link<Id, Data>) -> Boolean): Set<Node<Id, Data>> {
            tailrec fun groupFrom(
                froms: Set<Node<Id, Data>>,
                visited: Set<Node<Id, Data>> = froms
                                 ): Set<Node<Id, Data>> {
                val next = froms.asSequence()
                    .flatMap { from -> linksFrom(from) }
                    .filter { it.to !in visited }
                    .filter(linkFilter)
                    .map { it.to }
                    .toSet()

                return if (next.isEmpty()) visited else groupFrom(next, visited + next)
            }

            return groupFrom(setOf(getNode(from)))
        }

        override fun shortestPath(
            start: Id,
            end: Id,
            heuristic: (Node<Id, Data>) -> Int,
                                 ): Path<Id, Data>? {
            val bestPathTo = hashMapOf<Id, Path<Id, Data>>()
            return explore(Path.at(getNode(start))) { path -> linksFrom(path.end).asSequence().map { path + it } }
                .minimizing { it.weight(heuristic) }
                .filterExploration { path -> bestPathTo[path.end.id]?.let { it == path } ?: true }
                .onEach { path ->
                    val alreadyExistingPath = bestPathTo[path.end.id]
                    if (alreadyExistingPath == null || alreadyExistingPath.weight > path.weight)
                        bestPathTo[path.end.id] = path
                }
                .firstOrNull { it.end.id == end }
        }


        override fun allPathsWithAllNodes(): Collection<Path<Id, Data>> {
            fun next(path: Path<Id, Data>) =
                (content[path.end.id]?.links?.asSequence() ?: emptySequence())
                    .filter { it.from !in path || it.to !in path }
                    .map { path.append(it) }

            return nodes.fold(listOf()) { paths, _ ->
                if (paths.isEmpty()) nodes.map { Path.at(it) }.toList()
                else paths.flatMap(::next)
            }
        }

        override fun <NewData> mapNodes(mutation: (Node<Id, Data>) -> Node<Id, NewData>) =
            Impl(content.mapValues { it.value.mapNodes(mutation) })

        override fun <NewData> mapValues(mutation: (Data) -> NewData) = mapNodes { it.mapData(mutation) }

        private fun at(id: Id) = content.getValue(id)
        private fun linksFrom(id: Id) = at(id).links
        private fun linksFrom(node: Node<Id, Data>) = linksFrom(node.id)
        private fun getNode(id: Id) = at(id).node
    }

}

