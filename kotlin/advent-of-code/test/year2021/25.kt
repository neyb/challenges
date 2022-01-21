package year2021

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test
import year2021.day25.RegionState
import year2021.day25.part1


class `25` {

    val exampleRegion = RegionState.parse(
        """
            v...>>.vv>
            .vv>>.vv..
            >>.>v>...v
            >>v>>.>.v.
            v>v.vv.v..
            >.>>..v...
            .vv..>.>v.
            v.v..>>v.v
            ....v..v.>
        """.trimIndent().lines()
                                         )

    fun afterNIterations(nbIteration: Int) = (1..nbIteration).fold(exampleRegion) { exampleRegion, _ -> exampleRegion.next() }

    fun expectAfterNIteration(nbIteration: Int, expected: String) {
        val result = (1..nbIteration).fold(exampleRegion) { exampleRegion, _ -> exampleRegion.next() }
        val expectedRegion = RegionState.parse(expected.trimIndent().lines())
        expect(result).toEqual(expectedRegion)
    }

    @Test fun `part 1 - given test 1 iteration`() {
        expectAfterNIteration(
            1,
            """
                ....>.>v.>
                v.v>.>v.v.
                >v>>..>v..
                >>v>v>.>.v
                .>v.v...v.
                v>>.>vvv..
                ..v...>>..
                vv...>>vv.
                >.v.v..v.v
            """
                             )
    }

    @Test fun `part 1 - given test 20 iteration`() {
        expectAfterNIteration(
            20,
            """
                v>.....>>.
                >vv>.....v
                .>v>v.vv>>
                v>>>v.>v.>
                ....vv>v..
                .v.>>>vvv.
                ..v..>>vv.
                v.v...>>.v
                ..v.....v>
            """
                             )
    }

    @Test fun `part 1 - given test 57 iteration`() {
        expectAfterNIteration(
            57,
            """
                ..>>v>vv..
                ..v.>>vv..
                ..>>v>>vv.
                ..>>>>>vv.
                v......>vv
                v>v....>>v
                vvv.....>>
                >vv......>
                .>v.vv.v..
            """
                             )
    }

    @Test fun `part 1 - given test 58 iteration`() {
        expectAfterNIteration(
            58,
            """
                ..>>v>vv..
                ..v.>>vv..
                ..>>v>>vv.
                ..>>>>>vv.
                v......>vv
                v>v....>>v
                vvv.....>>
                >vv......>
                .>v.vv.v..
            """.trimIndent()
                             )
    }

    @Test fun `part 1 - 57 = 58th gen`(){
        val gen57 = afterNIterations(57)
        val gen58 = afterNIterations(58)
        expect(gen57).toEqual(gen58)
    }

    @Test fun `part 1 - given test`() {
        expect(part1(exampleRegion)).toEqual(58)
    }
}