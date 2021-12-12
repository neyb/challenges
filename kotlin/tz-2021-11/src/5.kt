import kotlin.math.min
import kotlin.math.pow

fun main() {
    val input = generateSequence(::readLine)
    val lines = input.toList()
    println(run5(lines[0]))
}

val magicBase = 31
val magnitude = { index: Int -> magicBase.toDouble().pow(index).toLong() }
val magicMod = 4294967296
val minCharCode = 33
val maxCharCode = 128

fun run5(input: String) = (1..10).asSequence()
    .mapNotNull { tryResolveFor(input, "alban", it) }
    .first()

fun tryResolveFor(input: String, base: String, nbAdditionnalChars: Int): String? {
    val baseHash = base.hash() * (magnitude(nbAdditionnalChars))
    var hashDiff = ((input.hash() - baseHash) % magicMod).let { if (it >= 0) it else it + magicMod }
    val maxHashDiff = 128 * (magnitude(nbAdditionnalChars - 1))

    while (hashDiff < maxHashDiff) {
        val result = tryResolveWith(nbAdditionnalChars, hashDiff)
        if (result != null) return base + result
        hashDiff += magicMod
    }

    return null
}

fun tryResolveWith(nbAdditionnalChars: Int, hashDiff: Long): String? {

    var rest = hashDiff
    val result = (nbAdditionnalChars - 1).downTo(0)
        .map {
            if (it == 0) rest
            else magnitude(it).let { magnitude ->
                (rest / magnitude)
                    .let { if (magnitude != 1L) min(it, maxCharCode.toLong()) else it }
                    .also { rest -= (it * magnitude) }
            }
        }
        .reversed()
        .toMutableList()

    if (result.first() > maxCharCode) return null;

    result
        .forEachIndexed { index, value ->
            while (result[index] < minCharCode && index != result.size - 1) {
                result[index + 1] = result[index + 1] - 1
                result[index] = result[index] + magicBase
            }
        }

    val isValid = result.all { it >= minCharCode && it <= maxCharCode } &&
            result.foldRight(0L) { value, acc -> acc * magicBase + value } == hashDiff

    return if (isValid) result.reversed()
        .asSequence()
        .map { it.toInt().toChar() }
        .joinToString("")
    else null
}

fun String.hash() = fold(0) { acc, c -> acc * 31 + c.code } % magicMod
