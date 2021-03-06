package year2021.day16

import common.*
import common.io.*
import kotlin.math.max
import kotlin.math.min

fun main() = run().forEach(::println)

val run = {
    day(2021, 16, part1, part2) {
        bufferedReader().use { reader ->
            PacketReader(reader.toSequence().filter { it != '\n' }.flatMap(hexaToBits)).readAllPackets()
        }
    }
}

val part1 = { packets: List<Packet> ->
    packets.asSequence()
        .flatMap { it.allPackets() }
        .fold(0) { versionSum, packet -> versionSum + packet.version }
}

val part2 = { packets: List<Packet> ->
    packets.fold(0L) { versionSum, packet -> versionSum + packet.value() }
}

val hexaToBits = { c: Char -> c.digitToInt(16).toString(2).padStart(4, '0').asSequence().map { it.digitToInt() } }

typealias Version = Short

sealed interface Packet {
    val version: Version
    val type: Short
    val subPackets: List<Packet>
    fun value(): Long

    fun allPackets(): Collection<Packet> = listOf(this) + subPackets.flatMap { it.allPackets() }
}

data class LiteralPacket(override val version: Version, val value: Long) : Packet {
    companion object {
        val type = 4.toShort()
        val subPackets = emptyList<Packet>()
    }

    override val type get() = Companion.type
    override val subPackets get() = Companion.subPackets
    override fun value() = value
}

data class OperatorPacket(
    override val version: Version,
    override val type: Short,
    override val subPackets: List<Packet>
                         ) : Packet {
    fun subValues() = subPackets.asSequence().map { it.value() }

    override fun value() = when (type.toInt()) {
        0 -> subValues().fold(0L, Long::plus)
        1 -> subValues().fold(1, Long::times)
        2 -> subValues().reduce(::min)
        3 -> subValues().reduce(::max)
        5 -> subPackets.map { it.value() }.let { (a, b) -> if (a > b) 1 else 0 }
        6 -> subPackets.map { it.value() }.let { (a, b) -> if (a < b) 1 else 0 }
        7 -> subPackets.map { it.value() }.let { (a, b) -> if (a == b) 1 else 0 }
        else -> throw Exception("unsupported type : $type")
    }
}

class PacketReader(input: Sequence<Int>) {
    private val iterator = input.iterator()

    private val seq = object : Sequence<Int> {
        override fun iterator() = iterator
    }

    private fun next() = seq.first()

    private fun readShort(length: Int): Short =
        seq.take(length).fold(0) { acc, bit -> acc * 2 + bit }.toShort()

    private fun readLiteralChain() = sequence {
        var stop = false
        while (!stop) {
            stop = next() == 0
            yield(readShort(4))
        }
    }.fold(0L) { acc, v -> acc * 16 + v }

    fun readPacket(): Packet {
        val version = readShort(3)
        val type = readShort(3)

        return when (type) {
            LiteralPacket.type -> LiteralPacket(version, readLiteralChain())
            else -> if (next() == 0) {
                val nbBits = readShort(15)
                val subPacketReader = PacketReader(seq.take(nbBits.toInt()))
                OperatorPacket(version, type, subPacketReader.readAllPackets())
            } else {
                val nbSubPackets = readShort(11)
                val subPackets = generateSequence { readPacket() }.take(nbSubPackets.toInt()).toList()
                OperatorPacket(version, type, subPackets)
            }
        }
    }

    fun readAllPackets(): List<Packet> = generateSequence {
        try {
            readPacket()
        } catch (e: NoSuchElementException) {
            null
        }
    }.toList()
}