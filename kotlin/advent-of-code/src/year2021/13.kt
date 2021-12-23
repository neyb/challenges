package year2021.day13

import common.*

fun main() = day(2021, 13, part1, part2)

enum class FoldDir {
    X, Y;

    companion object {
        fun parse(s: String) = when (s) {
            "x", "X" -> X
            "y" -> Y
            else -> throw Exception("$s is not a fold direction")
        }
    }
}

val part1 = { lines: List<String> ->
    val (areaLines, foldLines) = lines.split { it.isBlank() }

    val startingPoints =
        areaLines.map { it.split(",").let { (x, y) -> x.toInt() to y.toInt() } }.toSet()

    foldLines.asSequence()
        .mapNotNull { Regex("fold along ([yx])=(\\d+)").matchEntire(it)?.destructured }
        .map { (foldDir, foldPos) -> FoldDir.parse(foldDir) to foldPos.toInt() }
        .take(1)
        .fold(startingPoints) { points, (foldDir, foldPos) -> fold(points, foldDir, foldPos) }
        .count()
}

val part2 = { lines: List<String> ->
    val (areaLines, foldLines) = lines.split { it.isBlank() }

    val startingPoints =
        areaLines.map { it.split(",").let { (x, y) -> x.toInt() to y.toInt() } }.toSet()

    val folded = foldLines.asSequence()
        .mapNotNull { Regex("fold along ([yx])=(\\d+)").matchEntire(it)?.destructured }
        .map { (foldDir, foldPos) -> FoldDir.parse(foldDir) to foldPos.toInt() }
        .fold(startingPoints) { points, (foldDir, foldPos) -> fold(points, foldDir, foldPos) }

    val maxX = folded.maxOf { it.first }
    val maxY = folded.maxOf { it.second }

    (0..maxY).forEach { y ->
        println((0..maxX).map { x -> if ((x to y) in folded) "X" else " " }.joinToString(""))
    }

}

private fun fold(
    points: Set<Pair<Int, Int>>,
    foldDir: FoldDir,
    foldPos: Int
                ) = points.asSequence()
    .filter { (x, y) ->
        when (foldDir) {
            FoldDir.X -> x
            FoldDir.Y -> y
        } != foldPos
    }
    .map { (x, y) ->
        when (foldDir) {
            FoldDir.X -> (if (x > foldPos) (2 * foldPos) - x else x) to y
            FoldDir.Y -> x to (if (y > foldPos) (2 * foldPos) - y else y)
        }
    }
    .toSet()

