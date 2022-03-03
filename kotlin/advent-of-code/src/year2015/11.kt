package year2015.day11

fun main() = run().forEach(::println)
val run = { sequenceOf(part1(), part2()) }

val part1 = {
    generateSequence(Password("hxbxwxba"), Password::next)
        .drop(1)
        .first { it.isValid() }
}
val part2 = {
    generateSequence(part1(), Password::next)
        .drop(1)
        .first { it.isValid() }
}

@JvmInline
value class Password private constructor(private val chars: CharArray) {
    constructor(string: String) : this(string.toCharArray())

    private fun Char.next() = if (this == 'z') 'a' else inc()

    fun next(): Password {
        val newChars = chars.copyOf()
        for (i in chars.size - 1 downTo 0) {
            newChars[i] = chars[i].next()
            if (chars[i] != 'z') break
        }
        return Password(newChars)
    }

    fun isValid() = chars.size == 8 &&
            hasNoForbiddenLetters() &&
            hasThreeIncreasingLetters() &&
            hasTwoNonoverlappingPairs()

    private fun hasThreeIncreasingLetters() =
        chars.asSequence().windowed(3).any { (c1, c2, c3) -> c1 <= 'x' && c1.next() == c2 && c2.next() == c3 }

    private fun hasNoForbiddenLetters(): Boolean = arrayOf('i', 'o', 'l').none { it in chars }

    private fun hasTwoNonoverlappingPairs(): Boolean = chars.asSequence()
        .windowed(2)
        .withIndex()
        .filter { it.value.let { (c1, c2) -> c1 == c2 } }
        .map { it.index }
        .toList()
        .let { pairsIndex -> pairsIndex.size >= 2 && (pairsIndex.maxOf { it } - pairsIndex.minOf { it }) > 1 }

    override fun toString() = chars.concatToString()
}

