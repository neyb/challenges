package common.graph.graph2d

//class Path<T>(val nodes: List<Node<T>>) {
//    companion object {
//        fun <T> from(node: Node<T>) = Path(listOf(node))
//    }
//
//    val start get() = nodes.first()
//    val end get() = nodes.last()
//
//    fun weight(heuristic: (Node<T>) -> Int = { 0 }) =
//        nodes.fold(0) { total, node -> total + node.weight } + heuristic(end)
//
//    operator fun contains(coordinate: Coordinate) = nodes.any { it.coordinate2d == coordinate }
//    operator fun plus(node: Node<T>) = Path(nodes + node)
//
//    override fun toString() = nodes.asSequence().joinToString(" -> ", postfix = " (${weight()})")
//}