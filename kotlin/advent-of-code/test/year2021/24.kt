package year2021

import ch.tutteli.atrium.api.fluent.en_GB.its
import ch.tutteli.atrium.api.fluent.en_GB.toBeAnInstanceOf
import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test
import year2021.day24.*

class `24` {

    fun expressions(
        w: Expression.Numeric = 0.e,
        x: Expression.Numeric = 0.e,
        y: Expression.Numeric = 0.e,
        z: Expression.Numeric = 0.e
                   ) =
        mapOf(Operand.Variable.w to w, Operand.Variable.x to x, Operand.Variable.y to y, Operand.Variable.z to z)

    @Test fun `mul(1,25)`() {
        expect(1.e * 25.e).toBeAnInstanceOf<Expression.Numeric.Literal> { its { value }.toEqual(25) }
    }

    @Test fun `add(z, y)`() {
        val z = 26.e * (Expression.Numeric.Input(0) + 11.e)
        val y = Expression.Numeric.Input(1) + 11.e
        val result = z + y
        val expected = Expression.Numeric.Plus(listOf(z, y))
        expect(result)
            .its { toString() }.toEqual("((26) * ((input(0)) + (11))) + (input(1)) + (11)")
    }

    @Test fun `z = add(z, y)`() {
        val z = 26.e * (Expression.Numeric.Input(0) + 11.e)
        val y = Expression.Numeric.Input(1) + 11.e
        val context = AnalyseResult.Leaf(expressions(y = y, z = z), 2)

        val instruction =
            Instruction(Operand.Variable.z, Instruction.Type.add, listOf(Operand.Variable.z, Operand.Variable.y))

        val result = context.take(instruction)
        expect(result).toBeAnInstanceOf<AnalyseResult.Leaf> {
            its { expressions[Operand.Variable.z] }.toEqual(z + y)
        }
    }

    @Test fun `(26xinput(1)) div 26 = )input(1`() {
        val expression = (26.e * Expression.Numeric.Input(1)) / 26.e
        expect(expression).toEqual(Expression.Numeric.Input(1))
    }

}