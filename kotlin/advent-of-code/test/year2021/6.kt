package year2021

import year2021.day6.*

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

class `6` {
    @Test fun `part1 given 18 days test`() {
        expect(populationAfter(18)(listOf("3,4,3,1,2"))).toEqual(26)
    }

    @Test fun `part1 given 80 days test`() {
        expect(populationAfter(80)(listOf("3,4,3,1,2"))).toEqual(5934)
    }

    @Test fun `part1 given 256 days test`() {
        expect(populationAfter(256)(listOf("3,4,3,1,2"))).toEqual(26984457539)
    }
}