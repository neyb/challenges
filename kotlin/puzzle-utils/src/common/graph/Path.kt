package common.graph

import common.collections.*
import java.util.*

class Path<Id, Data> private constructor(
    val start: Node<Id, Data>,
    val end: Node<Id, Data>,
    val nodes: List<Node<Id, Data>>,
    val links: List<Link<Id, Data>>,
    val weight: Int
                                        ) {
    companion object {
        fun <Id, Data> at(node: Node<Id, Data>) =
            Path(node, node, LinkedList<Node<Id, Data>>().apply { add(node) }, LinkedList(), node.weight)
    }

    fun weight(heuristic: (Node<Id, Data>) -> Int) = weight + heuristic(end)

    operator fun contains(node: Node<Id, Data>) = links.any { node in it }
    operator fun contains(link: Link<Id, Data>) = links.contains(link)
    operator fun plus(link: Link<Id, Data>) = append(link)
    fun append(link: Link<Id, Data>): Path<Id, Data> {
        val newEnd = link.to
        return Path(start, newEnd, nodes + newEnd, links + link, weight + link.weight + newEnd.weight)
    }

    override fun toString(): String {
        var lastNode = start
        return links.joinToString("", prefix = "(${weight}) $lastNode") {
            lastNode = it.to
            " --[${it.weight}]--> $lastNode"
        }
    }
}