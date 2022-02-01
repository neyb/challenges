package common.graph

class Path<Id, Data>(val start: Node<Id, Data>, val links: List<Link<Id, Data>>) {
    companion object {
        fun <Id, Data> at(node: Node<Id, Data>) = Path(node, emptyList())
    }

    val nodes: List<Node<Id, Data>> by lazy {
        links.fold(mutableListOf(start)) { nodes, link -> nodes.apply { add(link.edgeFrom(nodes.last())) } }
    }

    val weight get() = links.sumOf { it.weight } + nodes.sumOf { it.weight }
    val end get() = links.fold(start) { edge, link -> link.edgeFrom(edge) }

    operator fun contains(node: Node<Id, Data>) = links.any { node in it }
    operator fun contains(link: Link<Id, Data>) = links.contains(link)
    fun append(link: Link<Id, Data>) = Path(start, links + link)

    override fun toString(): String {
        var lastNode = start
        return links.joinToString("", prefix = "($weight) $lastNode") {
            lastNode = it.edgeFrom(lastNode)
            " --[${it.weight}]--> $lastNode"
        }
    }
}