package `2015`.day3

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

class Day3Test{
    @Test fun `part2 given test 1`(){
        expect(part2(listOf("^v"))).toEqual(3)
    }
    @Test fun `part2 given test 2`(){
        expect(part2(listOf("^>v<"))).toEqual(3)
    }
}

