package year2015.day9

import common.*

typealias Node = common.graph.Node<String, Nothing?>
typealias Link = common.graph.Link<String, Nothing?>
typealias Graph = common.graph.Graph<String, Nothing?>


fun main() = run().forEach(::println)
fun run() = day(2015, 9, part1, part2) {
    useLines {
        val linkRegex = Regex("""(.*) to (.*) = (\d+)""")
        it.map { line ->
            val (_, from, to, weight) = linkRegex.matchEntire(line)!!.groupValues
            Link(Node(from, null), Node(to, null), weight.toInt())
        }.toList()
            .let(Graph::invoke)
    }
}

val part1 = { graph: Graph ->
    val allPathsWithAllNodes = graph.allPathsWithAllNodes()
    allPathsWithAllNodes.minOf { it.weight }
}

val part2 = { graph: Graph ->
    val allPathsWithAllNodes = graph.allPathsWithAllNodes()
    allPathsWithAllNodes.maxOf { it.weight }
}

