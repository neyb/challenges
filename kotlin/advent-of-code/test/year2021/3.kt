package year2021

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import year2021.day3.*
import org.junit.jupiter.api.Test

class `3` {
    @Test fun `part 2 given test`() {
        val input = listOf(
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
                          )

        expect(part2(input)).toEqual(230)
    }
}