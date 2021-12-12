package aoc2021.day11

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

class `11 test` {
    @Test fun `part 1 given test step 1`() {
        expect(
            part1(1)(
                """
                11111
                19991
                19191
                19991
                11111
            """.trimIndent().lines()
                    )
              ).toEqual(9)
    }

    @Test fun `part 1 given test 2 step 1`() {
        expect(
            part1(1)(
                """
                    5483143223
                    2745854711
                    5264556173
                    6141336146
                    6357385478
                    4167524645
                    2176841721
                    6882881134
                    4846848554
                    5283751526
            """.trimIndent().lines()
                    )
              ).toEqual(0)
    }

    @Test fun `part 1 given test 2 step 2`() {
        expect(
            part1(2)(
                """
                    5483143223
                    2745854711
                    5264556173
                    6141336146
                    6357385478
                    4167524645
                    2176841721
                    6882881134
                    4846848554
                    5283751526
            """.trimIndent().lines()
                    )
              ).toEqual(35)
    }

    @Test fun `part 1 given test 2 step 100`() {
        expect(
            part1(100)(
                """
                    5483143223
                    2745854711
                    5264556173
                    6141336146
                    6357385478
                    4167524645
                    2176841721
                    6882881134
                    4846848554
                    5283751526
            """.trimIndent().lines()
                    )
              ).toEqual(1656)
    }

}