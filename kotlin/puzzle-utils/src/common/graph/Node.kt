package common.graph

import common.`object`.*
import java.util.*

class Node<Id, Data>(val id: Id, val data: Data, val weight: Int = 0) {
    override fun hashCode() = Objects.hash(id)
    override fun equals(other: Any?) = eq(other, { id })
    override fun toString() = "$id"
}