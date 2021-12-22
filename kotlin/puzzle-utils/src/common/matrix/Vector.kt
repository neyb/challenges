package common.matrix

class Vector<T>(val values: List<T>) {
    val size get() = values.size
    operator fun get(index: Int) = values[index]
}