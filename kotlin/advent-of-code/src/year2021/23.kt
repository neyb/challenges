package year2021.day23

import common.*
import common.`object`.*
import common.algo.*
import year2021.day23.Move.Companion.at

fun main() = day(2021, 23, part1, part2)

val part1 = { lines: List<String> ->
    val gameState = GameState.parse(lines)
    val solution = findBestSolution(gameState)
    println(solution.historic().joinToString("\n\n"))
    solution.energyConsumed
}

val part2 = { lines: List<String> ->
    val gameState = GameState.parse(lines.toMutableList().apply {
        add(3, "  #D#C#B#A#")
        add(4, "  #D#B#A#C#")
    })
    val solution = findBestSolution(gameState)
    println(solution.historic().joinToString("\n\n"))
    solution.energyConsumed
}

fun findBestSolution(state: GameState): GameState = explore(state) { it.exploreNext() }
    .minimizing { it.energyConsumed + it.heuristic }
    .first { it.solved }

class Board private constructor(
    val nodes: Collection<Node>,
    val linksByNode: Map<Node, Set<Link>>
                               ) {
    constructor(nodes: Collection<Node>, links: List<Link>) : this(
        nodes,
        mutableMapOf<Node, MutableSet<Link>>().apply {
            links.forEach {
                merge(it.edge1, mutableSetOf(it)) { set1, set2 -> set1.apply { addAll(set2) } }
                merge(it.edge2, mutableSetOf(it)) { set1, set2 -> set1.apply { addAll(set2) } }
            }
        })
}

class GameState private constructor(
    private val board: Board,
    private val moves: Moves,
                                   ) {
    companion object {
        fun parse(lines: List<String>): GameState {
            val targetGen = generateSequence(AmphipodType.A) {
                when (it) {
                    AmphipodType.A -> AmphipodType.B
                    AmphipodType.B -> AmphipodType.C
                    AmphipodType.C -> AmphipodType.D
                    AmphipodType.D -> AmphipodType.A
                }
            }.iterator()

            val charByCoords =
                lines.flatMapIndexed { y, line -> line.asSequence().mapIndexed { x, c -> Coordinates(x, y) to c } }

            val nodes = charByCoords.filter { (_, c) -> c == '.' || c in 'A'..'D' }.map { (coords) ->
                val type = if (coords.y == 1) NodeType.hallway else NodeType.sideRooms
                Node(coords, type, if (type == NodeType.sideRooms) targetGen.next() else null)
            }

            val links = nodes.asSequence().flatMap { node ->
                nodes.asSequence().filter {
                    it.coordinates in with(node.coordinates) { arrayOf(copy(x = x - 1), copy(y = y - 1)) }
                }.map { other -> node linkTo other }
            }.toList()

            val amphypodsPlaceMoves = charByCoords.asSequence().filter { (_, c) -> c in 'A'..'D' }
                .map { (coord, c) -> AmphipodType.valueOf(c.toString()) at nodes.single { it.coordinates == coord } }
                .toList()
                .let(::Moves)

            return GameState(Board(nodes, links), amphypodsPlaceMoves)
        }
    }

    private val nodes get() = board.nodes
    private val linksByNode = board.linksByNode

    private val amphipods get() = moves.amphipods
    val energyConsumed get() = moves.energyConsumed
    private val amphipodsNode = moves.amphipodsNode
    private fun nodeOf(amphipod: Amphipod) = moves.nodeOf(amphipod)
    private fun findAmphipodAt(coordinates: Coordinates) = moves.findAmphipodAt(coordinates)

    val solved: Boolean = amphipodsNode.all { (amphipod, node) -> amphipod.type == node.target }

    val heuristic = amphipods.asSequence()
        .filter { nodeOf(it).target != it.type }
        .mapNotNull { fastestInGhostMove(it) }
        .sumOf { it.energyCost() }

    fun exploreNext(): Sequence<GameState> = moveinMoves().ifEmpty { outMoves() }.map { this + it }

    fun historic() = moves.historic().map { GameState(board, it) }

    private operator fun plus(move: Move) = GameState(board, moves + move)

    private fun outMoves() = amphipods.asSequence()
        .filter { nodeOf(it).type == NodeType.sideRooms }
        .filter { !isSideRoomFree(nodeOf(it).target!!) }
        .flatMap { outMoves(it) }

    private fun outMoves(amphipod: Amphipod) = outPaths(nodeOf(amphipod))
        .filter { path ->
            amphipodsNode.asSequence()
                .filter { it.value.target == amphipod.type }
                .minByOrNull { it.value.coordinates.y }
                ?.let { (amphipod) -> fastestInGhostMove(amphipod) }
                ?.let { path.to !in it.path }
                ?: true
        }
        .map { Move(amphipod, it) }

    private fun outPaths(from: Node): Sequence<Path> = availablePaths(from)
        .filter { it.to.type == NodeType.hallway }
        .filter { target -> !isEntrance(target.to) }

    private fun isEntrance(node: Node) = nodes.any { it.coordinates == with(node.coordinates) { copy(y = y + 1) } }

    private fun moveinMoves(): Sequence<Move> {
        val validSideroomsType = AmphipodType.values().asSequence().filter(::isSideRoomFree).toSet()
        return amphipods.asSequence()
            .filter { it.type in validSideroomsType }
            .filter { nodeOf(it).target != it.type }
            .mapNotNull { inMoves(it).maxByOrNull { it.energyCost() } }
    }

    private fun inMoves(amphipod: Amphipod) = inPaths(nodeOf(amphipod), amphipod.type).map { Move(amphipod, it) }

    private fun inPaths(from: Node, toType: AmphipodType) = availablePaths(from).filter { it.to.target == toType }

    private fun availablePaths(from: Node): Sequence<Path> {
        val visitedNodes = mutableSetOf<Node>()
        return explore(Path(from, emptyList())) { path -> linksByNode[path.to]!!.asSequence().map { path + it } }
            .filterExploration { it.to !in visitedNodes && (it.to !in amphipodsNode.values || it.length == 0) }
            .minimizing { it.length }
            .onEach { visitedNodes.add(it.to) }
    }

    private fun fastestInGhostMove(amphipod: Amphipod) = inGhostMoves(amphipod).firstOrNull()

    private fun inGhostMoves(amphipod: Amphipod) = availableGhostPaths(nodeOf(amphipod))
        .filter { it.to.target == amphipod.type }
        .map { Move(amphipod, it) }

    private fun availableGhostPaths(from: Node): Sequence<Path> {
        val visitedNodes = mutableSetOf<Node>()
        return explore(Path(from, emptyList())) { path ->
            linksByNode[path.to]!!.asSequence().map { path + it }
        }
            .filterExploration { it.to !in visitedNodes }
            .minimizing { it.length }
            .onEach { visitedNodes.add(it.to) }
    }

    private fun isSideRoomFree(amphipodType: AmphipodType) =
        amphipods.asSequence().filter { it.type != amphipodType }.map { amphipodsNode[it]!! }
            .all { it.target !== amphipodType }

    override fun toString(): String {
        val xRange = nodes.minOf { it.coordinates.x }..nodes.maxOf { it.coordinates.x }
        return (nodes.minOf { it.coordinates.y }..nodes.maxOf { it.coordinates.y })
            .map { y ->
                xRange.map { x ->
                    when {
                        findAmphipodAt(Coordinates(x, y)) != null ->
                            findAmphipodAt(Coordinates(x, y))?.type?.toString()
                        nodes.any { it.coordinates == Coordinates(x, y) } -> "."
                        else -> " "
                    }
                }.joinToString("")
            }.joinToString("\n")
    }
}

class Moves private constructor(
    val moves: List<Move>,
    val amphipods: Set<Amphipod>,
    val energyConsumed: Int,
    val amphipodsNode: Map<Amphipod, Node>,
                               ) {

    constructor(moves: List<Move>) : this(
        moves,
        moves.asSequence().map { it.amphipod }.toSet(),
        moves.sumOf { it.energyCost() },
        moves.groupingBy { it.amphipod }.fold({ _, t -> t.path.to }, { _, _, move2 -> move2.path.to }),
                                         )

    fun nodeOf(amphipod: Amphipod) = amphipodsNode[amphipod]!!

    fun findAmphipodAt(coordinates: Coordinates) = amphipodsNode.entries.asSequence()
        .firstOrNull { it.value.coordinates == coordinates }
        ?.key

    operator fun plus(move: Move) = Moves(
        moves + move,
        amphipods,
        energyConsumed + move.energyCost(),
        amphipodsNode + (move.amphipod to move.path.to),
                                         )

    fun historic(): List<Moves> = moves.fold(listOf()) { movess, move ->
        val newMoves = movess.lastOrNull()?.let { it + move } ?: Moves(listOf(move))
        if (movess.isEmpty() || move.path.length > 0) movess + newMoves
        else listOf(newMoves)
    }

}

data class Coordinates(val x: Int, val y: Int) {
    override fun toString() = "$x,$y"
}

class Node(val coordinates: Coordinates, val type: NodeType, val target: AmphipodType? = null) {
    infix fun linkTo(other: Node) = Link(this, other)

    override fun hashCode() = hash(coordinates)
    override fun equals(other: Any?) = eq(other, { coordinates })
    override fun toString() = "$coordinates"
}

class Link(val edge1: Node, val edge2: Node) {
    fun leadsTo(from: Node) = when (from) {
        edge1 -> edge2
        edge2 -> edge1
        else -> throw Exception("$from should be either $edge1 or $edge2")
    }

    override fun hashCode() = edge1.hashCode() + edge2.hashCode()
    override fun equals(other: Any?) =
        if (other is Link) (edge1 == other.edge1 && edge2 == other.edge2) || (edge2 == other.edge1 && edge1 == other.edge2)
        else false
}

data class Path(val from: Node, val links: List<Link>) {
    companion object {
        fun from(node: Node) = Path(node, emptyList())
    }

    val to = links.fold(from) { last, link -> link.leadsTo(last) }

    val length = links.size

    operator fun plus(link: Link) = copy(links = links + link)
    operator fun contains(node: Node) = node == from || links.any { it.edge1 == node || it.edge2 == node }
    override fun toString() = "$from -> $to"
}

class Amphipod(val type: AmphipodType) {
    override fun toString() = type.toString()
}

data class Move(val amphipod: Amphipod, val path: Path) {
    companion object {
        infix fun AmphipodType.at(node: Node) = Move(Amphipod(this), Path.from(node))
    }

    fun energyCost() = path.length * amphipod.type.movementCost

}

enum class NodeType { hallway, sideRooms }

enum class AmphipodType(val movementCost: Int) {
    A(1), B(10), C(100), D(1000)
}
