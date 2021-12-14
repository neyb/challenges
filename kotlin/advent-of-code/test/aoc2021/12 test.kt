package aoc2021.day12

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

class Aoc2021Day12Test{


    @Test fun `part2 given test count`() {
        expect(part2("""
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        """.trimIndent().lines())).toEqual(36)
    }
}