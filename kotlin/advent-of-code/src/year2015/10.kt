package year2015.day10

fun main() = run().forEach(::println)
val run = { sequenceOf(part1(), part2()) }

val input = "1321131112"
val part1 = {
    (1..40).fold(input) { c, _ -> next(c) }.length
}

val part2 = {
    (1..50).fold(input) { c, _ -> next(c) }.length
}

fun next(s: String) = with(StringBuilder(s.length * 2)) {
    var lastChar: Char? = null
    var nbCurrentChar = 0
    fun append() {
        if (lastChar != null) {
            append(nbCurrentChar)
            append(lastChar)
        }
    }
    s.forEach { char ->
        if (char == lastChar) nbCurrentChar += 1
        else {
            append()
            lastChar = char
            nbCurrentChar = 1
        }
    }
    append()
    toString()
}

