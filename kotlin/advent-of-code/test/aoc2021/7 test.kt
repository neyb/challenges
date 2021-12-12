package aoc2021.day7

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

class AdventOfCode2021Day7Test {

    @Test fun `part 1 given test`() {
        expect(part1(listOf("16,1,2,0,4,2,7,1,2,14"))).toEqual(37)
    }

    @Test fun `part 2 given test`() {
        expect(part2(listOf("16,1,2,0,4,2,7,1,2,14"))).toEqual(168)
    }
}

