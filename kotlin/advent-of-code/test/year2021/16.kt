package year2021

import year2021.day16.*

import ch.tutteli.atrium.api.fluent.en_GB.its
import ch.tutteli.atrium.api.fluent.en_GB.toBeAnInstanceOf
import ch.tutteli.atrium.api.fluent.en_GB.toContainExactly
import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

class `16` {
    @Test fun `part1 given test binary - 110100101111111000101000`() {
        val readPacket = ofBinary("110100101111111000101000").readPacket()
        expect(readPacket).toEqual(LiteralPacket(6, 2021))
    }

    @Test fun `part1 given test binary - 00111000000000000110111101000101001010010001001000000000`() {
        val readPacket = ofBinary("00111000000000000110111101000101001010010001001000000000").readPacket()
        expect(readPacket)
            .its({ version }) { toEqual(1) }
            .its({ type }) { toEqual(6) }
            .its({ subPackets }) {
                toContainExactly(
                    { toBeAnInstanceOf<LiteralPacket> { its { value }.toEqual(10) } },
                    { toBeAnInstanceOf<LiteralPacket> { its { value }.toEqual(20) } },
                                )
            }
    }

    @Test fun `part1 given test binary - 11101110000000001101010000001100100000100011000001100000`() {
        val readPacket = ofBinary("11101110000000001101010000001100100000100011000001100000").readPacket()
        expect(readPacket)
            .its({ version }) { toEqual(7) }
            .its({ type }) { toEqual(3) }
            .its({ subPackets }) {
                toContainExactly(
                    { toBeAnInstanceOf<LiteralPacket> { its { value }.toEqual(1) } },
                    { toBeAnInstanceOf<LiteralPacket> { its { value }.toEqual(2) } },
                    { toBeAnInstanceOf<LiteralPacket> { its { value }.toEqual(3) } },
                                )
            }
    }

    @Test fun `part1 given test - 8A004A801A8002F478`() {
        expect(part1(PacketReader("8A004A801A8002F478".asSequence().flatMap(hexaToBits)).readAllPackets())).toEqual(16)
    }

    private fun ofBinary(s: String) = PacketReader(s.asSequence().map { it.digitToInt() })
}

