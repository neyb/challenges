package year2021

import year2021.day18.Snailfish
import ch.tutteli.atrium.api.fluent.en_GB.its
import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

class `18` {

    val s = Snailfish.Companion::parse

    @Test fun `part1 parsing ex1`() {
        val snailfish = Snailfish.parse("[1,2]")
        expect(snailfish).toEqual(s("[1,2]"))
    }

    @Test fun `part1 parsing ex2`() {
        val snailfish = Snailfish.parse("[[1,2],3]")
        expect(snailfish).toEqual(s("[[1,2],3]"))
    }

    @Test fun `part1 magniture test1`() {
        val snailfish = s("[[1,2],[[3,4],5]]")
        expect(snailfish).its { magnitude }.toEqual(143)
    }

    @Test fun `part1 reduce test 1`() {
        val snailfish = s("[[[[[9,8],1],2],3],4]")
        expect(snailfish.reduce()).toEqual(s("[[[[0,9],2],3],4]"))
    }

    @Test fun `part1 reduce test 2`() {
        val snailfish = s("[[6,[5,[4,[3,2]]]],1]")
        expect(snailfish.reduce()).toEqual(s("[[6,[5,[7,0]]],3]"))
    }

    @Test fun `part1 sum test 1`() {
        val sum = s("[[[[4,3],4],4],[7,[[8,4],9]]]") + s("[1,1]")
        expect(sum).toEqual(s("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"))
    }

    @Test fun `part1 large example`() {
        val sum = """
            [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
            [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
            [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
            [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
            [7,[5,[[3,8],[1,4]]]]
            [[2,[2,2]],[8,[8,1]]]
            [2,9]
            [1,[[[9,3],9],[[9,0],[0,7]]]]
            [[[5,[7,4]],7],1]
            [[[[4,2],2],6],[8,7]]
        """.trimIndent().lines().map(s).reduce(Snailfish::plus)

        expect(sum).toEqual(s("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"))
    }

}

