package common.graph2d

@JvmRecord
data class Coordinate(val x: Int, val y: Int) {
    fun neightbours(withDiag: Boolean = false) = sequenceOf(-1, 0, 1)
        .flatMap { sequenceOf(it to -1, it to 0, it to 1) }
        .filter { (xDiff, yDiff) -> xDiff != 0 || yDiff != 0 }
        .filter { (xDiff, yDiff) -> withDiag || xDiff == 0 || yDiff == 0 }
        .map { (xDiff, yDiff) -> copy(x + xDiff, y + yDiff) }

    override fun toString() = "$x,$y"
}