package year2021.day19

import common.*
import common.matrix.*
import java.rmi.UnexpectedException
import kotlin.math.abs

fun main() = day(2021, 19, part1, part2) {
    readLines().split { it.isBlank() }
        .map(Scanner::parse)
        .let(World::from)
}

val part1 = { world: World ->
    world.beacons.size
}

val part2 = { world: World ->
    val scannersPosition = world.scanners.map { it.location.position }
    scannersPosition
        .flatMap { p1 -> scannersPosition.map { p2 -> (p1 - p2) } }
        .maxOf { it.run { abs(x) + abs(y) + abs(z) } }
}

val allRotations = Angle.values().asSequence().let { angles ->
    angles.flatMap { a -> angles.flatMap { b -> angles.map { c -> Rotation(a, b, c) } } }.distinctBy { it.matrix }
        .toSet()
}

class Rotation(val a: Angle, val b: Angle, val c: Angle) {
    val matrix = a.Rx * b.Ry * c.Rz

    val invert get() = Rotation(a.invert, b.invert, c.invert)

    override fun hashCode() = matrix.hashCode()
    override fun equals(other: Any?) = if (other is Rotation) matrix == other.matrix else false

    override fun toString() = "(α=$a,β=$b,γ=$c)"
}

enum class Angle(cos: Int, sin: Int) {
    `0`(1, 0), `90`(0, 1), `180`(-1, 0), `270`(0, -1);

    val Rx = listOf(
        listOf(1, 0, 0),
        listOf(0, cos, -sin),
        listOf(0, sin, cos),
                   ).toMatrix()

    val Ry = listOf(
        listOf(cos, 0, sin),
        listOf(0, 1, 0),
        listOf(-sin, 0, cos),
                   ).toMatrix()

    val Rz = listOf(
        listOf(cos, -sin, 0),
        listOf(sin, cos, 0),
        listOf(0, 0, 1),
                   ).toMatrix()

    val invert by lazy {
        when (this) {
            `0` -> `0`
            `90` -> `270`
            `180` -> `180`
            `270` -> `90`
        }
    }
}

class World(val scanners: Collection<LocalizedScanner>) {
    companion object {
        val empty = World(emptyList())
        fun from(scanner: Scanner) = World(listOf(scanner at ScannerLocation.ref))
        fun from(scanners: List<Scanner>): World {
            var world = from(scanners[0])
            while (scanners.any { it !in world }) {
                scanners.asSequence()
                    .filter { it !in world }
                    .forEach { world.localize(it)?.let { world += it } }
            }
            return world
        }
    }

    val beacons = scanners.asSequence().flatMap { it.rotatedBeacons }.toSet()

    operator fun contains(scanner: Scanner) = scanners.any { it.scanner == scanner }

    operator fun plus(scanner: LocalizedScanner) = World(scanners + scanner)

    fun localize(scanner: Scanner): LocalizedScanner? {
        fun bestLocation(rotation: Rotation): Scored<ScannerLocation> =
            scanner.withRotation(rotation).rotatedBeacons.asSequence()
                .flatMap { rotatedBeacon -> beacons.map { beacon -> beacon - rotatedBeacon } }
                .groupingBy { it }
                .eachCount().asSequence()
                .maxByOrNull { it.value }
                ?.let { (translation, score) -> Scored(score, ScannerLocation(rotation, translation)) }
                ?: throw UnexpectedException("no correlation ?")

        val guessedLocation: ScannerLocation? = allRotations.asSequence()
            .map(::bestLocation)
            .maxByOrNull { it.score }!!
            .takeIf { it.score >= 12 }
            ?.item

        return guessedLocation?.let { scanner at it }
    }
}

data class Scanner(val name: String, val beaconsPosition: Collection<Position>) {
    companion object {
        fun parse(s: String) = parse(s.lines())
        fun parse(lines: List<String>): Scanner {
            val matchResult = Regex("""--- (?<name>.*) ---""").matchEntire(lines[0])
                ?: throw Exception("first line should be the name")
            val name = matchResult.groups["name"]!!.value
            val beacons = lines.asSequence()
                .drop(1)
                .map(Position::parse)
                .toList()
            return Scanner(name, beacons)
        }
    }

    fun withRotation(rotation: Rotation) = LocalizedScanner(this, ScannerLocation(rotation, Position.origin))

    infix fun at(location: ScannerLocation) = LocalizedScanner(this, location)

    override fun toString() = name
}

data class Scored<out T>(val score: Int, val item: T)

data class LocalizedScanner(val scanner: Scanner, val location: ScannerLocation) {
    val rotatedBeacons = scanner.beaconsPosition.map { it.rotate(location.rotation) + location.position }
}

data class ScannerLocation(val rotation: Rotation, val position: Position) {
    companion object {
        val ref = ScannerLocation(Rotation(Angle.`0`, Angle.`0`, Angle.`0`), Position.origin)
    }
}

data class Position(val x: Int, val y: Int, val z: Int) {

    companion object {
        val origin = Position(0, 0, 0)
        fun from(vector: Vector<Int>) = Position(vector[0], vector[1], vector[2])
        fun parse(s: String) = s.split(",").let { (x, y, z) -> Position(x.toInt(), y.toInt(), z.toInt()) }
    }

    fun rotate(rotation: Rotation) = from(rotation.matrix * asVector())
    private fun asVector() = Vector(listOf(x, y, z))

    operator fun plus(other: Position) = Position(x + other.x, y + other.y, z + other.z)
    operator fun minus(other: Position) = Position(x - other.x, y - other.y, z - other.z)

    override fun toString() = "($x,$y,$z)"
}



