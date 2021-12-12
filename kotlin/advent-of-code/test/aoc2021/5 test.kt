package aoc2021.day5

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

class Day5Test {
    @Test fun `part 2 given test`() {
        expect(
            commonRun { it.pointsPart2 }(
                """
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2
        """.trimIndent().split("\n")
                                        )
              ).toEqual(12)
    }
}