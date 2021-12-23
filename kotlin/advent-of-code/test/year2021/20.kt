package year2021

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Tag
import org.junit.jupiter.api.Test
import year2021.day20.Input
import year2021.day20.part1
import year2021.day20.part2

class `20` {

    val givenInput = """
            ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
            #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
            .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
            .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
            .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
            ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
            ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

            #..#.
            #....
            ##..#
            ..#..
            ..###
        """.trimIndent().lines().let(Input::parse)

    @Test fun `part1 - given test`() {
        val count = part1(givenInput)
        expect(count).toEqual(35)
    }

    @Tag("slow")
    @Test fun `part2 - given test`() {
        val count = part2(givenInput)
        expect(count).toEqual(3351)
    }
}