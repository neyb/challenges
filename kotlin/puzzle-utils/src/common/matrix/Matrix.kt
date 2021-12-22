package common.matrix

fun List<List<Int>>.toMatrix() = Matrix.int(this)

class Matrix<T>(val lines: List<List<T>>, private val plus: (T, T) -> T, private val times: (T, T) -> T) {
    companion object {
        fun int(lines: List<List<Int>>) = Matrix(lines, Int::plus, Int::times)
    }

    init {
        if (lines.map { it.size }.distinct().size > 1)
            throw Exception("all line should have same size")
    }

    val m = lines.size
    val n = lines.first().size

    private operator fun T.plus(other: T) = plus(this, other)
    private operator fun T.times(other: T) = times(this, other)

    operator fun get(j: Int, i: Int) = lines[j][i]

    operator fun times(other: Matrix<T>): Matrix<T> =
        if (n != other.m) throw Exception("n($n) should match other's m(${other.m})")
        else Matrix((0..m - 1).map { j ->
            (0..other.n - 1).map { i ->
                (0..n - 1).asSequence().map { this[j, it] * other[it, i] }.reduce { a, b -> a + b }
            }
        }, plus, times)

    operator fun times(vector: Vector<T>): Vector<T> =
        if (n == vector.size) Vector((0..m - 1).map { j ->
            (0..n - 1).asSequence().map { this[j, it] * vector[it] }.reduce { a, b -> a + b }
        })
        else throw Exception("n ($n) should match vector size (${vector.size})")

    override fun hashCode() = lines.hashCode()

    override fun equals(other: Any?) = if (other is Matrix<*>) lines == other.lines else false
}