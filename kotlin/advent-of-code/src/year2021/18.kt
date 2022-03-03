package year2021.day18

import common.*
import year2021.day18.Direction.Left
import year2021.day18.Direction.Right

fun main() = run().forEach(::println)

val run = { day(2021, 18, part1, part2) { readLines().map(Snailfish.Companion::parse) } }

val part1 = { snailfishes: List<Snailfish> ->
    snailfishes.reduce(Snailfish::plus).magnitude
}

val part2 = { snailfishes: List<Snailfish> ->
    snailfishes.asSequence()
        .mapIndexed { index, s ->
            snailfishes.asSequence()
                .filterIndexed { withIndex, _ -> withIndex != index }
                .map { s + it }
                .maxOf { it.magnitude }
        }
        .maxOf { it }
}

class Snailfish(val element: Element) {

    companion object {
        fun parse(s: String) = Snailfish(Element.parse(s))
    }

    val magnitude get() = element.magniture

    fun reduce() = generateSequence(this) { snailfish ->
        snailfish.nextExplodePath()?.let { return@generateSequence snailfish.explode(it) }
        snailfish.nextSplitPath()?.let { return@generateSequence snailfish.split(it) }
        null
    }.last()

    private fun Path<Element>.outleafAt(direction: Direction): Path<Value>? =
        directions.indexOfLast { it != direction }.let { indexLastRight ->
            if (indexLastRight < 0) return null
            else (take<Element>(indexLastRight).plus<Element>(direction)).let { from ->
                @Suppress("UNCHECKED_CAST")
                element[from].traverse(from).asSequence()
                    .filter { element[it] is Value }
                    .filter { it.directions.drop(from.depth).all { it != direction } }
                    .first() as Path<Value>
            }
        }

    @Suppress("UNCHECKED_CAST")
    private fun nextExplodePath() =
        element.traverse().asSequence().firstOrNull { it.depth >= 4 && element[it] is Pair } as Path<Pair>?

    private fun explode(path: Path<Pair>): Snailfish {
        val toExplode = element[path]
        val leftValue = (toExplode.left as Value).value
        val rightValue = (toExplode.right as Value).value

        return Snailfish(element
                             .run { update(path) { Value(0) } }
                             .run { path.outleafAt(Left)?.let { update(it) { Value(it.value + leftValue) } } ?: this }
                             .run { path.outleafAt(Right)?.let { update(it) { Value(it.value + rightValue) } } ?: this }
                        )
    }

    @Suppress("UNCHECKED_CAST")
    private fun nextSplitPath() = element.traverse().asSequence()
        .firstOrNull { element[it].let { e -> e is Value && e.value >= 10 } } as Path<Value>?

    private fun split(path: Path<Value>) =
        Snailfish(element.update(path) { Pair(Value(it.value / 2), Value(it.value - (it.value / 2))) })

    operator fun plus(other: Snailfish) = Snailfish(Pair(element, other.element)).reduce()

    override fun hashCode() = element.hashCode()
    override fun equals(other: Any?) = other is Snailfish && element == other.element

    override fun toString() = element.toString()
}

enum class Direction { Left, Right }

class Path<out E : Element>(val directions: List<Direction>) {
    companion object {
        fun <E : Element> empty() = Path<E>(emptyList())
    }

    val depth get() = directions.size

    fun head() = directions.first()
    fun tail() = drop(1)
    fun drop(n: Int) = Path<E>(directions.subList(n, directions.size))

    fun isEmpty() = directions.isEmpty()
    fun <NewE : Element> take(n: Int) = Path<NewE>(directions.take(n))

    fun startsWith(other: Path<Element>) = directions.subList(0, other.depth) == other

    operator fun <E : Element> plus(direction: Direction) = Path<E>(directions + direction)

    override fun toString() = directions.toString()
}

sealed interface Element {
    companion object {
        fun parse(s: String): Element {
            fun parse(s: String): kotlin.Pair<Element, String> = when (s.first()) {
                '[' -> {
                    val (left, restL) = parse(s.substring(1))
                    val (right, restR) = parse(restL.substring(1))
                    Pair(left, right) to restR.substring(1)
                }
                else -> s.indexOfFirst { !it.isDigit() }
                    .let { Value(s.substring(0, it).toInt()) to s.substring(it) }
            }

            return parse(s).first
        }
    }

    val magniture: Int

    fun traverse(path: Path<Element> = Path.empty()): List<Path<Element>>

    operator fun <E : Element> get(path: Path<E>): E
    fun <E : Element> update(pathToUpdate: Path<E>, update: (E) -> Element): Element
}

data class Pair(val left: Element, val right: Element) : Element {
    override val magniture get() = 3 * left.magniture + 2 * right.magniture

    @Suppress("UNCHECKED_CAST")
    override fun <E : Element> get(path: Path<E>) =
        if (path.isEmpty()) this as E
        else this[path.head()][path.tail()]

    override fun traverse(path: Path<Element>) =
        listOf(path) + left.traverse(path + Left) + right.traverse(path + Right)

    private operator fun get(direction: Direction) = when (direction) {
        Left -> left
        Right -> right
    }

    @Suppress("UNCHECKED_CAST")
    override fun <E : Element> update(pathToUpdate: Path<E>, update: (E) -> Element) =
        if (pathToUpdate.isEmpty()) update(this as E)
        else when (pathToUpdate.head()) {
            Left -> copy(left = left.update(pathToUpdate.tail(), update))
            Right -> copy(right = right.update(pathToUpdate.tail(), update))
        }

    override fun toString() = "[$left,$right]"
}

data class Value(val value: Int) : Element {
    override val magniture get() = value

    @Suppress("UNCHECKED_CAST")
    override fun <E : Element> get(path: Path<E>) =
        if (path.isEmpty()) this as E
        else throw Exception("cannot get $path of leaf")

    override fun traverse(path: Path<Element>) = listOf(path)

    @Suppress("UNCHECKED_CAST")
    override fun <E : Element> update(pathToUpdate: Path<E>, update: (E) -> Element) =
        if (pathToUpdate.isEmpty()) update(this as E)
        else throw Exception("$pathToUpdate not in leaf")

    override fun toString() = value.toString()
}

