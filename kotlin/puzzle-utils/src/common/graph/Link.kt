package common.graph

class Link<Id, Data>(val edge1: Node<Id, Data>, val edge2: Node<Id, Data>, val weight: Int = 1) {
    fun edgeFrom(edge: Node<Id, Data>) = when (edge) {
        edge1 -> edge2
        edge2 -> edge1
        else -> throw Exception("$edge not in $this")
    }

    operator fun contains(edge: Node<Id, Data>) = edge == edge1 || edge == edge2

    override fun hashCode() = edge1.hashCode() + edge2.hashCode()
    override fun equals(other: Any?) =
        other is Link<*, *> && ((edge1 == other.edge1 && edge2 == other.edge2) || (edge1 == other.edge2 && edge2 == other.edge1))

    override fun toString() = "$edge1 <-[$weight]-> $edge2"
}