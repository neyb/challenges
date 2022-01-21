package year2021.day12

fun main() = common.day(2021, 12, part1, part2)

val start = Cave("start")
val end = Cave("end")

val part1 = { lines: List<String> ->
    parseGraph(lines).allPath(start, end) { node, path -> node != start && (!node.small || node !in path.nodes) }.size
}

val canAppendpart2 = { cave: Cave, path: Path ->
    val hasSeveralSmallCave =
        { path.nodes.asSequence().filter { it.small }.groupingBy { it }.eachCount().values.any { it > 1 } }
    cave != start && (!cave.small || cave !in path.nodes || !hasSeveralSmallCave())
}

val part2 = { lines: List<String> -> parseGraph(lines).allPath(start, end, canAppend = canAppendpart2).size }

fun parseGraph(lines: List<String>) = lines.asSequence()
    .map { it.split("-") }
    .map { (a, b) -> Link(Cave(a), Cave(b)) }
    .toSet()
    .let(::Graph)

@JvmRecord
data class Cave(val name: String) {
    val small get() = name.all { it.isLowerCase() }

    override fun toString() = name
}

@JvmRecord
data class Link(val a: Cave, val b: Cave) {

    fun target(from: Cave) = when (from) {
        a -> b
        b -> a
        else -> throw Exception("$from not in link $a <-> $b")
    }

    override fun hashCode() = a.hashCode() + b.hashCode()
    override fun equals(other: Any?) =
        if (other is Link) (a == other.a && b == other.b) || (a == other.b && b == other.a)
        else false
}

data class Path constructor(val from: Cave, val links: List<Link>) {
    companion object {
        fun startingAt(start: Cave) = Path(start, emptyList())
    }

    val nodes by lazy { links.runningFold(from) { node, link -> link.target(node) } }
    val end by lazy { links.fold(from) { node, link -> link.target(node) } }

    operator fun plus(next: Link) = copy(links = links + next)

    override fun toString() = nodes.joinToString("->")
}

class Graph(links: Set<Link>) {
    private val links: Map<Cave, Set<Link>> = mutableMapOf<Cave, MutableSet<Link>>().apply {
        val remaping = { link: Link ->
            { _: Cave, links: MutableSet<Link>? -> (links ?: mutableSetOf()).apply { add(link) } }
        }

        links.forEach {
            compute(it.a, remaping(it))
            compute(it.b, remaping(it))
        }
    }

    fun allPath(
        from: Cave,
        to: Cave,
        currentPath: Path = Path.startingAt(from),
        canAppend: (Cave, Path) -> Boolean = { node, path -> node !in path.nodes }
               ): Set<Path> =
        (links[currentPath.end]?.asSequence() ?: emptySequence())
            .filter { canAppend(it.target(currentPath.end), currentPath) }
            .map { currentPath + it }
            .flatMap { path -> if (path.end == to) setOf(path) else allPath(from, to, path, canAppend) }
            .toSet()
}