package year2021.day24

import common.*
import common.`object`.*
import java.util.*
import kotlin.math.max
import kotlin.math.min

fun main() = day(2021, 24, part1, part2) {
    readLines().map(Instruction::parse).let(::Alu)
}

val part1 = { alu: Alu ->
    val analyseResult = alu.analyze()
    val conditionforPotential = analyseResult.conditionforPotential(Operand.Variable.z, 0)
    (0..13).asSequence()
        .map { conditionforPotential.possibleRangeFor(it) }
        .fold(sequenceOf(emptyList<Int>())) { seq, range ->
            seq.flatMap { modelValues -> range.down().map { modelValues + it.toInt() } }
        }
        .map(::Model)
        .first { analyseResult.calc(it, Operand.Variable.z) == 0 }
}

val part2 = { alu: Alu ->
    val context = alu.analyze()
    val conditionforPotential = context.conditionforPotential(Operand.Variable.z, 0)
    (0..13).asSequence()
        .map { conditionforPotential.possibleRangeFor(it) }
        .fold(sequenceOf(emptyList<Int>())) { seq, range ->
            seq.flatMap { modelValues -> range.up().map { modelValues + it.toInt() } }
        }
        .map(::Model)
        .first { context.calc(it, Operand.Variable.z) == 0 }
}

class Alu(private val instructions: List<Instruction>) {
    //    fun analyze() = instructions.fold(AnalyzeContext.Leaf.default, AnalyzeContext::take)
    fun analyze() = instructions.fold(AnalyseResult.Leaf.default as AnalyseResult) { analyzeContext, instruction ->
        val result = analyzeContext.take(instruction)
        result
    }
}

interface Formatable {
    fun format(indent: Int = 0): String
}

data class Instruction(
    val settedVar: Operand.Variable,
    val type: Type,
    val operands: List<Operand>,
                      ) {
    companion object {
        fun parse(s: String) = s.split(" ").let { splitted ->
            Instruction(
                Operand.Variable.valueOf(splitted[1]),
                Type.valueOf(splitted[0]),
                splitted.asSequence().drop(1).map { Operand.parse(it) }.toList(),
                       )
        }
    }

    enum class Type { inp, add, mul, div, mod, eql }

    override fun toString() = "$settedVar = $type(${operands.joinToString()})"

}

sealed interface Operand { //    fun value(context: RunContext): Int

    companion object {
        fun parse(s: String) = when {
            s.matches(Regex("""-?\d+""")) -> Literal(s.toInt())
            s.matches(Regex("[wxyz]")) -> Variable.valueOf(s)
            else -> throw Exception("cannot parse operand $s")
        }
    }

    enum class Variable : Operand {
        w, x, y, z;
    }

    class Literal(val value: Int) : Operand {
        override fun toString() = value.toString()
    }
}

data class Model(private val inputs: List<Int>) {
    operator fun get(index: Int) = inputs[index]
    fun with(nextValue: Int) = Model(inputs + nextValue)

    override fun toString() = inputs.joinToString("")
}

fun <T : Any, O : Any> nullableBinaryOp(a: T?, b: T?, op: (T, T) -> O) = a?.let { b?.let { op(a, b) } }

interface AnalyseResult : Formatable {
    fun take(instruction: Instruction): AnalyseResult
    fun setting(variable: Operand.Variable, to: Expression.Numeric): AnalyseResult
    fun calc(model: Model, variable: Operand.Variable): Int
    fun conditionforPotential(variable: Operand.Variable, value: Int): Expression.Condition

    data class Leaf(val expressions: Map<Operand.Variable, Expression.Numeric>, val nextInputIndex: Int) :
        AnalyseResult {

        companion object {
            val default = Leaf(
                mapOf(
                    Operand.Variable.w to 0.e,
                    Operand.Variable.x to 0.e,
                    Operand.Variable.y to 0.e,
                    Operand.Variable.z to 0.e
                     ),
                0
                              )
        }


        override fun take(instruction: Instruction): AnalyseResult = when (instruction.type) {
            Instruction.Type.eql -> If(
                instruction.expression(0) eql instruction.expression(1),
                setting(instruction.settedVar, Expression.Numeric.Literal.one),
                setting(instruction.settedVar, Expression.Numeric.Literal.zero),
                                      ).simplify()
            else -> Leaf(
                expressions + (instruction.settedVar to expression(instruction)),
                this.nextInputIndex + if (instruction.type == Instruction.Type.inp) 1 else 0
                        )
        }

        override fun setting(variable: Operand.Variable, to: Expression.Numeric) =
            copy(expressions = expressions + (variable to to))

        override fun calc(model: Model, variable: Operand.Variable) = get(variable).calc(model)

        override fun conditionforPotential(variable: Operand.Variable, value: Int) = when {
            get(variable).estimation == value.r ->
                true.e
            value !in get(variable).estimation ->
                false.e
            else -> get(variable) eql value.e
        }.simplify()

        private fun expression(instruction: Instruction) = with(instruction) {
            when (instruction.type) {
                Instruction.Type.inp -> Expression.Numeric.Input(nextInputIndex)
                Instruction.Type.add -> expression(0) + expression(1)
                Instruction.Type.mul -> expression(0) * expression(1)
                Instruction.Type.div -> expression(0) / expression(1)
                Instruction.Type.mod -> expression(0) % expression(1) //                Instruction.Type.eql -> expression(0) eql expression(1)
                else -> throw Exception("eql should not be an exp")
            }
        }

        private fun Instruction.expression(index: Int) = operands[index].let { operand ->
            when (operand) {
                is Operand.Variable -> expressions.getValue(operand)
                is Operand.Literal -> Expression.Numeric.Literal(operand.value)
            }
        }

        override fun format(indent: Int) = """
            w=${expressions.getValue(Operand.Variable.w)}
            x=${expressions.getValue(Operand.Variable.x)}
            y=${expressions.getValue(Operand.Variable.y)}
            z=${expressions.getValue(Operand.Variable.z)}
        """.replaceIndent("  ".repeat(indent))

        private fun get(variable: Operand.Variable) = expressions.getOrDefault(variable, 0.e)
    }

    data class If(val condition: Expression.Condition, val ifTrue: AnalyseResult, val ifFalse: AnalyseResult) :
        AnalyseResult {
        override fun take(instruction: Instruction): AnalyseResult =
            If(condition, ifTrue.take(instruction), ifFalse.take(instruction)).simplify()

        override fun setting(variable: Operand.Variable, to: Expression.Numeric) =
            copy(ifTrue = ifTrue.setting(variable, to), ifFalse = ifFalse.setting(variable, to)).simplify()

        override fun calc(model: Model, variable: Operand.Variable) =
            if (condition.calc(model)) ifTrue.calc(model, variable)
            else ifFalse.calc(model, variable)

        fun simplify() = when {
            ifTrue == ifFalse -> ifTrue
            condition.estimation == true -> ifTrue
            condition.estimation == false -> ifFalse
            else -> this
        }

        override fun conditionforPotential(variable: Operand.Variable, value: Int) =
            (condition and ifTrue.conditionforPotential(variable, value)) or
                    (condition.not() and ifFalse.conditionforPotential(variable, value))

        override fun format(indent: Int) = """
if(${condition.format()}){
${ifTrue.format(indent + 1)}
} else {
${ifFalse.format(indent + 1)}
}
        """.replaceIndent("  ".repeat(indent))

    }
}


class Range private constructor(val min: Long, val max: Long) {
    companion object {
        fun at(v: Int) = at(v.toLong())
        fun at(v: Long) = Range(v, v)
        fun of(a: Int, b: Int) = of(a.toLong(), b.toLong())
        fun of(a: Long, b: Long) = if (a <= b) Range(a, b) else Range(b, a)
    }

    val single get() = min == max

    fun isSingle(value: Int) = min == max && min == value.toLong()
    fun mustEquals(other: Range) = single && other.single && min == other.min

    fun notCross(other: Range) = max < other.min || min > other.max

    infix fun intersectionWith(other: Range) =
        Range(max(min, other.min), min(max, other.max)).takeIf { it.min <= it.max }

    fun down() = (max downTo min).asSequence()
    fun up() = (min..max).asSequence()

    operator fun contains(v: Int) = v in min..max
    operator fun contains(v: Long) = v in min..max
    operator fun contains(other: Range) = other.min in this && other.max in this
    operator fun plus(other: Range) = Range((min + other.min), (max + other.max))
    operator fun times(other: Range) = Range((min * other.min), (max * other.max))
    operator fun div(other: Range) = of((min / other.max), (max / other.min))
    operator fun rem(other: Range) = of(0, other.max)

    override fun hashCode() = Objects.hash(min, max)
    override fun equals(other: Any?) = eq(other, { min }, { max })
    override fun toString() = "$min..$max"
}

operator fun Range?.contains(other: Range?) = nullableBinaryOp(this, other, Range::contains) ?: false

val Int.e
    get() = when (this) {
        0 -> Expression.Numeric.Literal.zero
        1 -> Expression.Numeric.Literal.one
        else -> Expression.Numeric.Literal(this)
    }
val Boolean.e get() = if (this) Expression.Condition.Always.True else Expression.Condition.Always.False
val Int.r get() = Range.at(this)
val IntRange.r get() = Range.of(first.toLong(), last.toLong())
val LongRange.r get() = Range.of(first, last)

interface Expression<T, Estimate> {
    fun calc(model: Model): T
    val estimation: Estimate
    fun simplify(): Expression<T, Estimate>

    interface Numeric : Expression<Int, Range> {
        override fun equals(other: Any?): Boolean

        fun useInput(inputIndex: Int): Boolean
        fun isolate(input: Input): (Numeric) -> Numeric = throw UnsupportedOperationException()

        override fun simplify(): Numeric
        operator fun plus(other: Numeric) = Plus(listOf(this, other)).simplify()
        operator fun times(other: Numeric) = Mult(listOf(this, other)).simplify()
        operator fun div(other: Numeric) = Div(this, other).simplify()
        operator fun rem(other: Numeric) = Mod(this, other).simplify()
        infix fun eql(other: Numeric) = Condition.Eq(this, other).simplify()

        data class Literal(val value: Int) : Numeric {
            override fun calc(model: Model) = value
            override val estimation = Range.at(value)
            override fun simplify() = this
            override fun useInput(inputIndex: Int) = false
            override fun toString() = value.toString()

            companion object {
                val zero = Literal(0)
                val one = Literal(1)
            }
        }

        data class Input(private val index: Int) : Numeric {
            override fun calc(model: Model) = model[index]
            override val estimation = Range.of(1, 9)
            override fun useInput(inputIndex: Int) = inputIndex == index
            override fun simplify() = this
            override fun isolate(input: Input): (Numeric) -> Numeric = { it }
            override fun toString() = "input($index)"
        }

        data class Plus(val expressions: List<Numeric>) : Numeric {
            override fun calc(model: Model) = expressions.sumOf { it.calc(model) }
            override val estimation = expressions.asSequence().map { it.estimation }.fold(0.r, Range::plus)

            override fun simplify(): Numeric = when {
                expressions.isEmpty() -> 0.e
                expressions.size == 1 -> expressions[0]
                expressions.any { it is Plus } -> Plus(expressions.flattenInstanceOf(Plus::class) { it.expressions }).simplify()
                expressions.any { it.estimation == 0.r } -> Plus(expressions.filter { it.estimation != 0.r }).simplify()
                expressions.count { it is Literal } > 1 -> Plus(
                    expressions.joiningIfInstanceOf(Literal::class) { l1, l2 ->
                        Literal(l1.value + l2.value)
                    }).simplify()
                else -> this
            }

            override fun useInput(inputIndex: Int) = expressions.any { it.useInput(inputIndex) }

            override fun isolate(input: Input): (Numeric) -> Numeric = if (expressions.contains(input)) {
                { numeric ->
                    Plus(mutableListOf(numeric).apply {
                        addAll((expressions - input).map { it * (-1).e })
                    }).simplify()
                }
            } else throw Exception("cannot isolate $input from $this")

            fun map(mapper: (Numeric) -> Numeric) = Plus(expressions.map(mapper))

            override fun toString() = expressions.joinToString(" + ") { "($it)" }
        }

        data class Mult(val expressions: List<Numeric>) : Numeric {
            override fun calc(model: Model) = expressions.map { it.calc(model) }.reduce(Int::times)
            override val estimation = expressions.asSequence().map { it.estimation }.fold(1.r, Range::times)
            override fun simplify(): Numeric = mergeLiterals().remove1s().let { mult ->
                when {
                    expressions.any { it.estimation == 0.r } -> Literal.zero
                    mult.expressions.isEmpty() -> Literal.one
                    mult.expressions.size == 1 -> mult.expressions[0]
                    mult.expressions.all { it is Literal } -> mult.expressions.fold(1) { result, e -> result * (e as Literal).value }
                        .let(::Literal)
                    else -> mult
                }
            }

            override fun useInput(inputIndex: Int) = expressions.any { it.useInput(inputIndex) }

            fun map(mapper: (Numeric) -> Numeric) = Mult(expressions.map(mapper))

            private fun remove1s() = Mult(expressions.filter { it.estimation != 1.r })

            private fun mergeLiterals() = Mult(
                (sequenceOf(expressions.asSequence().filterIsInstance<Literal>()
                                .fold(Literal.one) { a, b -> Literal(a.value * b.value) }) + expressions.asSequence()
                    .filter { it !is Literal }).toList()
                                              )

            override fun toString() = expressions.joinToString(" * ") { "($it)" }
        }

        data class Div(val a: Numeric, val b: Numeric) : Numeric {
            override fun calc(model: Model) = a.calc(model) / b.calc(model)

            override val estimation = a.estimation / b.estimation

            override fun simplify(): Numeric = when {
                b.estimation == 1.r -> a
                a == b -> Literal.one
                a is Plus -> Plus(a.expressions.map { (it / b) }).simplify()
                b is Literal -> when {
                    a.estimation in (0..b.value).r -> Literal.zero
                    a is Mult && a.expressions.any { it.estimation == b.estimation } -> Mult(a.expressions - a.expressions.first { it.estimation == b.estimation }).simplify()
                    a is Literal -> Literal(a.value / b.value)
                    else -> this
                }
                else -> this
            }

            override fun useInput(inputIndex: Int) = a.useInput(inputIndex) || b.useInput(inputIndex)

            override fun toString() = "($a) / ($b)"
        }

        data class Mod(val a: Numeric, val b: Numeric) : Numeric {
            override fun calc(model: Model) = a.calc(model) % b.calc(model)
            override val estimation = (0..b.estimation.max).r

            override fun simplify(): Numeric = simplify(true)

            private fun simplify(plus: Boolean): Numeric = when {
                a.estimation in (0..b.estimation.min).r -> a
                b is Literal -> when {
                    a is Literal -> Literal(a.value % b.value)
                    a is Mult -> if (a.expressions.any { it is Literal && it.value == b.value }) Literal.zero
                    else this
                    a is Plus && plus -> (Mod(Plus(a.expressions.map { (it % b).simplify() }), b)).simplify(false)
                    else -> this
                } //            a is If -> a.map { (it % b).simplify() }.simplify()
                //            b is If -> If(b.a, b.b, (b.ifEq % a).simplify(), (b.ifNe % a).simplify()).simplify()
                else -> this
            }

            override fun useInput(inputIndex: Int) = a.useInput(inputIndex) || b.useInput(inputIndex)

            override fun toString() = "($a) % ($b)"
        }
    }

    interface Condition : Expression<Boolean, Boolean?> {
        fun format(): String
        override fun simplify(): Condition
        fun possibleRangeFor(inputIndex: Int): Range = throw Exception("???")

        fun not() = Not(this)
        infix fun and(other: Condition) = And(listOf(this, other)).simplify()
        infix fun or(other: Condition) = Or(this, other).simplify()

        class Eq(val left: Numeric, val right: Numeric) : Condition {
            override val estimation
                get() = when {
                    left.estimation.mustEquals(right.estimation) -> true
                    left.estimation.notCross(right.estimation) -> false
                    else -> null
                }

            override fun simplify() = when {
                left.estimation.mustEquals(right.estimation) -> true.e
                left.estimation.notCross(right.estimation) -> false.e
                else -> this
            }

            override fun calc(model: Model) = left.calc(model) == right.calc(model)

            override fun possibleRangeFor(inputIndex: Int): Range =
                if (left.useInput(inputIndex) || right.useInput(inputIndex)) {
                    val withInput = sequenceOf(left, right).single { it.useInput(inputIndex) }
                    val withoutInput = sequenceOf(left, right).single { !it.useInput(inputIndex) }
                    val isolate = withInput.isolate(Numeric.Input(inputIndex))
                    Range.of(1, 9).intersectionWith(isolate(withoutInput).estimation)
                        ?: throw Exception("cannot resolve $this for Input($inputIndex)")
                } else {
                    (1..9).r
                }

            override fun format() = "$left == $right"
            override fun toString() = format()
        }

        class Always private constructor(val value: Boolean) : Condition {
            companion object {
                val True = Always(true)
                val False = Always(false)
            }

            override val estimation get() = value
            override fun simplify() = this
            override fun calc(model: Model) = value
            override fun format() = "$value"
            override fun toString() = format()
        }

        class And(val conditions: List<Condition>) : Condition {
            override val estimation = when {
                conditions.isEmpty() -> true
                conditions.any { it.estimation == false } -> false
                conditions.all { it.estimation == true } -> true
                else -> null
            }

            override fun simplify(): Condition = when {
                conditions.any { it.estimation == false } -> false.e
                conditions.any { it.estimation == true } -> And(conditions.filter { it.estimation != true }).simplify()
                conditions.isEmpty() -> true.e
                else -> this
            }

            override fun possibleRangeFor(inputIndex: Int): Range {
                return conditions.fold((1..9).r) { result, condition ->
                    (result intersectionWith condition.possibleRangeFor(inputIndex)) ?: (1..9).r
                }
            }

            override fun calc(model: Model) = conditions.fold(true) { res, cond -> res && cond.calc(model) }

            override fun format() = conditions.joinToString(" && ") { "(${it.format()})" }
            override fun toString() = format()
        }

        class Or(val left: Condition, val right: Condition) : Condition {
            override val estimation
                get() = when {
                    left.estimation == true -> right.estimation
                    right.estimation == true -> left.estimation
                    left.estimation == false && right.estimation == false -> false
                    else -> null
                }

            override fun simplify() = when {
                left.estimation == true -> true.e
                right.estimation == true -> true.e
                left.estimation == false -> right
                right.estimation == false -> left
                else -> this
            }

            override fun calc(model: Model) = left.calc(model) || right.calc(model)

            override fun format() = "(${left.format()}) || (${right.format()})"
            override fun toString() = format()
        }

        class Not(val condition: Condition) : Condition {
            override val estimation = condition.estimation?.let(Boolean::not)

            override fun simplify() = this

            override fun calc(model: Model) = !condition.calc(model)

            override fun format() = "!${condition.format()}"
            override fun toString() = format()
        }

    }

}