package year2021

import ch.tutteli.atrium.api.fluent.en_GB.toContainExactly
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Tag
import org.junit.jupiter.api.Test

@Tag("slow")
class Results {
    @Test fun day1() {
        expect(year2021.day1.run().toList()).toContainExactly(1583, 1627)
    }

    @Test fun day2() {
        expect(year2021.day2.run().toList()).toContainExactly(1855814, 1845455714)
    }

    @Test fun day3() {
        expect(year2021.day3.run().toList()).toContainExactly(3895776, 7928162)
    }

    @Test fun day4() {
        expect(year2021.day4.run().toList()).toContainExactly(35670, 22704)
    }

    @Test fun day5() {
        expect(year2021.day5.run().toList()).toContainExactly(8350, 19374)
    }

    @Test fun day6() {
        expect(year2021.day6.run().toList()).toContainExactly(352151, 1601616884019)
    }

    @Test fun day7() {
        expect(year2021.day7.run().toList()).toContainExactly(344297, 97164301)
    }

    @Test fun day8() {
        expect(year2021.day8.run().toList()).toContainExactly(445, 1043101)
    }

    @Test fun day9() {
        expect(year2021.day9.run().toList()).toContainExactly(600, 987840)
    }

    @Test fun day10() {
        expect(year2021.day10.run().toList()).toContainExactly(387363, 4330777059)
    }

    @Test fun day11() {
        expect(year2021.day11.run().toList()).toContainExactly(1705, 265)
    }

    @Test fun day12() {
        expect(year2021.day12.run().toList()).toContainExactly(4011, 108035)
    }

    @Test fun day13() {
        expect(year2021.day13.run().toList()).toContainExactly(827, """
            XXXX  XX  X  X X  X XXX  XXXX  XX  XXX 
            X    X  X X  X X X  X  X X    X  X X  X
            XXX  X  X XXXX XX   X  X XXX  X    X  X
            X    XXXX X  X X X  XXX  X    X    XXX 
            X    X  X X  X X X  X X  X    X  X X   
            XXXX X  X X  X X  X X  X XXXX  XX  X   
        """.trimIndent())
    }

    @Test fun day14() {
        expect(year2021.day14.run().toList()).toContainExactly(2988, 3572761917024)
    }

    @Tag("very-slow")
    @Test fun day15() {
        expect(year2021.day15.run().toList()).toContainExactly(456, 2831)
    }

    @Test fun day16() {
        expect(year2021.day16.run().toList()).toContainExactly(1007, 834151779165)
    }

    @Test fun day17() {
        expect(year2021.day17.run().toList()).toContainExactly(9870, 5523)
    }

    @Test fun day18() {
        expect(year2021.day18.run().toList()).toContainExactly(4057, 4683)
    }

    @Test fun day19() {
        expect(year2021.day19.run().toList()).toContainExactly(483, 14804)
    }

    @Tag("very-slow")
    @Test fun day20() {
        expect(year2021.day20.run().toList()).toContainExactly(5291, 16665)
    }

    @Test fun day21() {
        expect(year2021.day21.run().toList()).toContainExactly(757770, 712381680443927)
    }

    @Test fun day22() {
        expect(year2021.day22.run().toList()).toContainExactly(556501, 1217140271559773)
    }

    @Tag("very-slow")
    @Test fun day23() {
        expect(year2021.day23.run().toList()).toContainExactly(14546, 42308)
    }

    @Test fun day24() {
        expect(year2021.day24.run().map { it.toString() }.toList()).toContainExactly("92928914999991", "91811211611981")
    }

    @Test fun day25() {
        expect(year2021.day25.run().toList()).toContainExactly(563)
    }


}