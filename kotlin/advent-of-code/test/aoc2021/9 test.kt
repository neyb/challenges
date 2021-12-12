package aoc2021.day9

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

class AdventOfCode2021Day9Test {
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

