package year2021

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import common.graph.graph2d.*
import org.junit.jupiter.api.Test
import year2021.day11.OctopusMap2d
import year2021.day11.part1

class `11` {

    fun octopusMapOf(s: String) =
        OctopusMap2d(Map2d.parseLinesWithItem(s.trimIndent().lineSequence()) { it.digitToInt() })

    @Test fun `part 1 given test step 1`() {
        expect(
            part1(1)(
                octopusMapOf(
                    """
                        11111
                        19991
                        19191
                        19991
                        11111
                    """
                            )
                    )
              ).toEqual(9)
    }

    @Test fun `part 1 given test 2 step 1`() {
        expect(
            part1(1)(
                octopusMapOf(
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
            """
                            )
                    )
              ).toEqual(0)
    }

    @Test fun `part 1 given test 2 step 2`() {
        expect(
            part1(2)(
                octopusMapOf(
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
            """
                            )
                    )
              ).toEqual(35)
    }

    @Test fun `part 1 given test 2 step 100`() {
        expect(
            part1(100)(octopusMapOf(
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
            """)
                      )
              ).toEqual(1656)
    }

}