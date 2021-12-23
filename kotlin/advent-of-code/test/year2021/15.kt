package year2021

import year2021.day15.*

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Tag
import org.junit.jupiter.api.Test

class `15` {
    @Test fun `part1 given test`() {
        expect(
            part1(
                """
                    1163751742
                    1381373672
                    2136511328
                    3694931569
                    7463417111
                    1319128137
                    1359912421
                    3125421639
                    1293138521
                    2311944581
                """.trimIndent().lines()
                 )
              ).toEqual(40)
    }

    @Test fun `part2 given test`() {
        expect(
            part2(
                """
                    1163751742
                    1381373672
                    2136511328
                    3694931569
                    7463417111
                    1319128137
                    1359912421
                    3125421639
                    1293138521
                    2311944581
                """.trimIndent().lines()
                 )
              ).toEqual(315)
    }
}

