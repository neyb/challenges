package exercice1;

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

class TwoSum {
    @Test fun `sum 9 in (2,7,11,15) is (0,1)`() {
        expect(searchSumIn(listOf(2, 7, 11, 15), 9)).toEqual(0 to 1)
    }

    @Test fun `search sum of 6 in (3,2,4) is (1,2)`() {
        expect(searchSumIn(listOf(3, 2, 4), 6)).toEqual(1 to 2)
    }

    @Test fun `search sum of 6 in (3,3) is (0,1)`() {
        expect(searchSumIn(listOf(3, 3), 6)).toEqual(0 to 1)
    }
}


