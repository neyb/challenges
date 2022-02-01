package common.graph

class Graph<Id, Data>(val nodes: Collection<Node<Id, Data>>, links: Collection<Link<Id, Data>>) {
    companion object {
        fun <Id, Data> of(links: Collection<Link<Id, Data>>) = Graph(
            links.flatMapTo(mutableSetOf()) { listOf(it.edge1, it.edge2) },
            links
                                                                    )
    }

    val linksByNode = links.asSequence()
        .flatMap { link -> sequenceOf(link.edge1, link.edge2).map { it to link } }
        .groupBy({ it.first }, { it.second })

    fun allPathsWithAllNodes(): Collection<Path<Id, Data>> {
        fun next(path: Path<Id, Data>) =
            (linksByNode[path.end]?.asSequence() ?: emptySequence())
                .filter { it.edge1 !in path || it.edge2 !in path }
                .map { path.append(it) }

        return nodes.fold(listOf()) { paths, _ ->
            if (paths.isEmpty()) nodes.map { Path.at(it) }
            else paths.flatMap(::next)
        }
    }
}