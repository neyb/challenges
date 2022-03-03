package common.graph

import common.`object`.*
import java.util.*

class Node<out Id, out Data>(val id: Id, val data: Data, val weight: Int = 0) {
    fun <NewData> mapData(mutation:(Data) -> NewData) = Node(id, mutation(data), weight)
    override fun hashCode() = Objects.hash(id)
    override fun equals(other: Any?) = eq(other, { id })
    override fun toString() = "$id"
}