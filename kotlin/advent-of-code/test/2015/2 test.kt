package `2015`.day2

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

class Day2Test {
    @Test fun `given part 1 test`(){
        expect(part1(listOf("2x3x4"))).toEqual(58)
    }
    @Test fun `given part 1 test 2`(){
        expect(part1(listOf("1x1x10"))).toEqual(43)
    }
}

