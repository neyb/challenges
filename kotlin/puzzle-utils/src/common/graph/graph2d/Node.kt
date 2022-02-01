package common.graph.graph2d

@JvmRecord
data class Node<out T>(val coordinate2d: Coordinate, val value: T, val weight:Int = 1) {
    override fun toString() = "(${coordinate2d.x},${coordinate2d.y}${value?.let { "[$it]" }?:"" }${if(weight != 1) ":$weight" else ""})"
}