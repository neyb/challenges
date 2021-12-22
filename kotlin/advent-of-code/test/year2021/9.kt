package year2021

import year2021.day9.*

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

class `9` {
    @Test fun `part1 givent test`() {
        expect(
            part1(
                """
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        """.trimIndent().lines()
                 )
              ).toEqual(15)
    }

    @Test fun `part2 givent test`() {
        expect(
            part2(
                """
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        """.trimIndent().lines()
                 )
              ).toEqual(1134)
    }
}

