package common.graph

import common.`object`.*

class Link<out Id, out Data>(val from: Node<Id, Data>, val to: Node<Id, Data>, val weight: Int = 1) {
    fun <NewId, NewData> mapNodes(mutation: (Node<Id, Data>) -> Node<NewId, NewData>) =
        Link(mutation(from), mutation(to), weight)

    fun <NewData> mapData(mutation: (Data) -> NewData) = Link(from.mapData(mutation), to.mapData(mutation), weight)

    operator fun contains(edge: Node<@UnsafeVariance Id, @UnsafeVariance Data>) = edge == from || edge == to

    override fun hashCode() = hash(from, to)
    override fun equals(other: Any?) = eq(other, { from }, { to })
    override fun toString() = "$from <-[$weight]-> $to"
}