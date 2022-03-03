package common.graph

import common.`object`.*

class Link<Id, Data>(val from: Node<Id, Data>, val to: Node<Id, Data>, val weight: Int = 1) {
    fun <NewData> mapNodes(mutation: (Node<Id, Data>) -> Node<Id, NewData>) =
        Link(mutation(from), mutation(to), weight)

    fun <NewData> mapData(mutation: (Data) -> NewData) = Link(from.mapData(mutation), to.mapData(mutation), weight)

    operator fun contains(edge: Node<Id, Data>) = edge == from || edge == to

    override fun hashCode() = hash(from, to)
    override fun equals(other: Any?) = eq(other, { from }, { to })
    override fun toString() = "$from <-[$weight]-> $to"
}