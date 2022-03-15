package year2021.day23

import common.*
import common.`object`.*
import year2021.day23.Move.Companion.at
import common.collections.plus as fastPlus

fun main() = run().forEach(::println)

val run = { day(2021, 23, part1, part2) }

val part1 = { lines: List<String> ->
    val gameState = GameState.parse(lines)
    findBestSolution(gameState).energyConsumed
}

val part2 = { lines: List<String> ->
    val gameState = GameState.parse(lines.toMutableList().apply {
        add(3, "  #D#C#B#A#")
        add(4, "  #D#B#A#C#")
    })
    findBestSolution(gameState).energyConsumed
}

fun findBestSolution(state: GameState): GameState =
    explore(state) { it.exploreNext() }.minimizing { it.energyConsumed + it.heuristic }.first { it.solved }

class Board private constructor(
    val nodes: Collection<Node>, val linksByNode: Map<Node, Set<Link>>
                               ) {
    constructor(nodes: Collection<Node>, links: List<Link>) : this(nodes, mutableMapOf<Node, MutableSet<Link>>().apply {
        links.forEach {
            merge(it.edge1, mutableSetOf(it)) { set1, set2 -> set1.apply { addAll(set2) } }
            merge(it.edge2, mutableSetOf(it)) { set1, set2 -> set1.apply { addAll(set2) } }
        }
    })

    fun amphipodStateFor(move: Move): AmphipodState {
        val amphipod = move.amphipod
        val newNode = move.path.to

        fun ghostPaths(): Sequence<Path> {
            val visitedNodes = mutableSetOf<Node>()
            return explore(Path.from(newNode)) { path ->
                linksByNode[path.to]!!.asSequence().map { path + it }
            }.filterExploration { it.to !in visitedNodes }.minimizing { it.length }.onEach { visitedNodes.add(it.to) }
        }

        val inGhostMoves = if (move.isInMove()) emptySequence()
        else ghostPaths().filter { it.to.target == amphipod.type }.map { Move(amphipod, it) }

        //        val outGhostMoves = ghostPaths()
        //            .filter { it.to.type == NodeType.hallway }
        //            .map { Move(amphipod, it) }
        //            .toList()

        return AmphipodState(newNode, inGhostMoves.firstOrNull())
    }

}

// optimise #3 : fastplus on moves
class GameState private constructor(private val board: Board, private val moves: Moves) {
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
            val board = Board(nodes, links)

            val amphypodsPlaceMoves = charByCoords.asSequence().filter { (_, c) -> c in 'A'..'D' }
                .map { (coord, c) -> AmphipodType.valueOf(c.toString()) at nodes.single { it.coordinates == coord } }
                .toList().let { Moves(it, board::amphipodStateFor) }

            return GameState(board, amphypodsPlaceMoves)
        }
    }

    private val nodes get() = board.nodes
    private val linksByNode = board.linksByNode

    private val amphipods get() = moves.amphipods.keys
    val energyConsumed get() = moves.energyConsumed

    private val usedNodes by lazy { moves.amphipods.values.map { it.position } }
    private fun nodeOf(amphipod: Amphipod) = moves.nodeOf(amphipod)
    private fun findAmphipodAt(coordinates: Coordinates) = moves.findAmphipodAt(coordinates)

    val solved: Boolean = moves.amphipods.all { (amphipod, state) -> amphipod.type == state.position.target }

    val heuristic = moves.heuristic

    fun exploreNext(): Sequence<GameState> = moveinMoves().ifEmpty { outMoves() }.map { this + it }

    private operator fun plus(move: Move) = GameState(board, moves.plus(move, board.amphipodStateFor(move)))

    private fun outMoves(): Sequence<Move> {
        fun outPaths(from: Node): Sequence<Path> =
            availablePaths(from).filter { it.to.type == NodeType.hallway }.filter { target -> !isEntrance(target.to) }

        fun outMoves(amphipod: Amphipod) = outPaths(nodeOf(amphipod)).filter { path -> //to simplify
            moves.amphipods.asSequence().filter { it.value.position.target == amphipod.type }
                .minByOrNull { it.value.position.coordinates.y }?.let { it.value.fastestInGhostMove }
                ?.let { path.to !in it.path } ?: true
        }.map { Move(amphipod, it) }

        return moves.amphipods.asSequence().filter { it.value.position.type == NodeType.sideRooms }
            .filter { !isSideRoomFree(it.value.position.target!!) }.flatMap { outMoves(it.key) }
    }

    private fun isEntrance(node: Node) = nodes.any { it.coordinates == with(node.coordinates) { copy(y = y + 1) } }

    private fun moveinMoves(): Sequence<Move> {
        fun inPaths(from: Node, toType: AmphipodType) = availablePaths(from).filter { it.to.target == toType }
        fun inMoves(amphipod: Amphipod) = inPaths(nodeOf(amphipod), amphipod.type).map { Move(amphipod, it) }

        val validSideroomsType = AmphipodType.values().asSequence().filter(::isSideRoomFree).toSet()
        return amphipods.asSequence().filter { it.type in validSideroomsType }.filter { nodeOf(it).target != it.type }
            .mapNotNull { inMoves(it).maxByOrNull { it.energyCost() } }
    }

    private fun availablePaths(from: Node): Sequence<Path> {
        val visitedNodes = mutableSetOf<Node>()
        return explore(Path.from(from)) { path ->
            linksByNode[path.to]!!.asSequence().map { path + it }
        }.filterExploration { it.to !in visitedNodes && (it.to !in usedNodes || it.length == 0) }
            .minimizing { it.length }.onEach { visitedNodes.add(it.to) }
    }

    private fun isSideRoomFree(amphipodType: AmphipodType) =
        moves.amphipods.asSequence()
            .filter { it.key.type != amphipodType }
            .map { it.value.position }
            .all { it.target !== amphipodType }

    override fun toString(): String {
        val xRange = nodes.minOf { it.coordinates.x }..nodes.maxOf { it.coordinates.x }
        return (nodes.minOf { it.coordinates.y }..nodes.maxOf { it.coordinates.y }).map { y ->
            xRange.map { x ->
                when {
                    findAmphipodAt(Coordinates(x, y)) != null -> findAmphipodAt(Coordinates(x, y))?.type?.toString()
                    nodes.any { it.coordinates == Coordinates(x, y) } -> "."
                    else -> " "
                }
            }.joinToString("")
        }.joinToString("\n")
    }
}

class Moves private constructor(
    val moves: List<Move>,
    val amphipods: Map<Amphipod, AmphipodState>,
    val energyConsumed: Int,
                               ) {

    val heuristic = amphipods.asSequence().sumOf { it.value.heuristic }

    constructor(moves: List<Move>, calcState: (Move) -> AmphipodState) : this(
        moves,
        moves.asSequence().map { it.amphipod to calcState(it) }.toMap(),
        moves.sumOf { it.energyCost() },
                                                                             )

    fun nodeOf(amphipod: Amphipod) = amphipods.getValue(amphipod).position

    fun findAmphipodAt(coordinates: Coordinates) =
        amphipods.entries.asSequence().firstOrNull { it.value.position.coordinates == coordinates }?.key

    fun plus(move: Move, newAmphipodState: AmphipodState) = Moves(
        moves.fastPlus(move),
        amphipods + (move.amphipod to newAmphipodState),
        energyConsumed + move.energyCost(),
                                                                 )
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

// to optimise #2 : use common.collections.plus
data class Path private constructor(
    val from: Node,
    val to: Node,
    val links: List<Link>,
    val length: Int,
                                   ) {

    companion object {
        fun from(node: Node) = Path(node, node, emptyList(), 0)
    }

    operator fun plus(link: Link) = Path(from, link.leadsTo(to), links.fastPlus(link), length + 1)
    operator fun contains(node: Node) = node == from || links.any { it.edge1 == node || it.edge2 == node }
    override fun toString() = "$from -> $to"
}

class Amphipod(val type: AmphipodType) {
    override fun toString() = type.toString()
}

class AmphipodState(val position: Node, val fastestInGhostMove: Move?) {
    val heuristic: Int = fastestInGhostMove?.energyCost() ?: 0
}

data class Move(val amphipod: Amphipod, val path: Path) {
    companion object {
        infix fun AmphipodType.at(node: Node) = Move(Amphipod(this), Path.from(node))
    }

    fun energyCost() = path.length * amphipod.type.movementCost
    fun isInMove() = amphipod.type == path.to.target
}

enum class NodeType { hallway, sideRooms }

enum class AmphipodType(val movementCost: Int) {
    A(1), B(10), C(100), D(1000)
}
