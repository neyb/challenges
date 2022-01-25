fun main() {
    val input = generateSequence(::readLine)
    val lines = input.toList()

    println(run4(lines))
}

typealias InclRange = Pair<Int, Int>

//data class Point(val x:Int, val y:Int) {
//    val distance = {other:Point -> sqrt((x-other.x).toDouble().pow(2) + (y-other.y).toDouble().pow(2))}
//    fun distance(other:Point) = sqrt((x-other.x).toDouble().pow(2) + (y-other.y).toDouble().pow(2))
//}

fun <I, O> ((I) -> O).memoize() =
    with(mutableMapOf<I, O>()) { { input: I -> computeIfAbsent(input, this@memoize) } }

fun rangeXorFor(numbers: List<Int>): ((InclRange) -> Int) {
    var previous = 0
    val xorsTo = numbers.map { previous.xor(it).also { previous = it } }
    return { (start, end): InclRange -> (xorsTo.getOrNull(start - 1) ?: 0).xor(xorsTo[end]) }.memoize()
}

fun run4(lines: List<String>): String {
    val numbers = lines[1].split(" ").map { it.toInt() }

    val rangeXor = rangeXorFor(numbers)

    val results = lines.asSequence()
        .drop(2)
        .map {
            val splitted = it.split(" ")
            Pair(splitted[0].toInt(), splitted[1].toInt())
        }
        .map(rangeXor)
        .groupingBy { it }
        .eachCount()

    return (0..255)
        .asSequence()
        .map { results[it] ?: 0 }
        .map { it.toString() }
        .joinToString(" ")
}