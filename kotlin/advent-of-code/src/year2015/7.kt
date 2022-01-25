package year2015.day7

import common.*
import year2015.day7.Expression.Literal
import year2015.day7.Expression.Variable

fun main() = day(2015, 7, part1, part2) {
    useLines { it.map { Instruction.parse(it) }.toList() }
}

val part1 = { instructions: List<Instruction> ->
    instructions.fold(Context.init, Context::take)["a"]
}

val part2 = { instructions: List<Instruction> ->
    val context = instructions.fold(Context.init, Context::take)
    context.take(Instruction(Literal(context["a"]), Variable("b")))["a"]
}

class Instruction(val expr: Expression, val assignTo: Variable) {
    companion object {
        private val r = Regex("""(?<expr>.*) -> (?<var>.*)""")
        fun parse(s: String) =
            r.matchEntire(s)!!.groupValues.let { (_, expr, variableName) ->
                Instruction(Expression.parse(expr), Variable(variableName))
            }
    }
}


interface Expression {
    fun calc(context: Context): Int

    companion object {
        private val binaryOpPattern = Regex("""(?<a>.+) (?<op>\w+) (?<b>.+)""")
        private val unaryOpPattern = Regex("""(?<op>\w+) (?<a>.+)""")
        private val variablePattern = Regex("""[a-z]+""")
        private val literalPattern = Regex("""\d+""")
        fun parse(s: String): Expression {
            fun Regex.expr(vararg names: String, get: (List<String>) -> Expression): Expression =
                matchEntire(s)!!.groups.let { groups -> get(names.map { groups[it]!!.value }) }
            return when {
                binaryOpPattern.matches(s) -> binaryOpPattern.expr("op", "a", "b") { (op, a, b) ->
                    when (op) {
                        "AND" -> And(parse(a), parse(b))
                        "OR" -> Or(parse(a), parse(b))
                        "LSHIFT" -> LShift(parse(a), parse(b))
                        "RSHIFT" -> RShift(parse(a), parse(b))
                        else -> throw Exception("unsupported binary operator $op in $s")
                    }
                }
                unaryOpPattern.matches(s) -> unaryOpPattern.expr("op", "a") { (op, a) ->
                    when (op) {
                        "NOT" -> Not(parse(a))
                        else -> throw Exception("unsupported unary operator $op in $s")
                    }
                }
                variablePattern.matches(s) -> Variable(s)
                literalPattern.matches(s) -> Literal(s.toInt())
                else -> throw Exception("no expression pattern match \"$s\"")
            }
        }
    }

    data class Variable(val name: String) : Expression {
        override fun calc(context: Context) = context[this]
    }

    class Literal(val value: Int) : Expression {
        override fun calc(context: Context) = value
    }

    class And(val a: Expression, val b: Expression) : Expression {
        override fun calc(context: Context) = with(context) { a() and b() }
    }

    class LShift(val a: Expression, val b: Expression) : Expression {
        override fun calc(context: Context) = with(context) { a() shl b() }
    }

    class RShift(val a: Expression, val b: Expression) : Expression {
        override fun calc(context: Context) = with(context) { a() shr b() }
    }

    class Not(val a: Expression) : Expression {
        override fun calc(context: Context) = with(context) { a().inv() }
    }

    class Or(val a: Expression, val b: Expression) : Expression {
        override fun calc(context: Context) = with(context) { a() or b() }
    }

    class Cached(val e: Expression) : Expression by e {
        private val cache = mutableMapOf<Context, Int>()
        override fun calc(context: Context) = cache.computeIfAbsent(context, e::calc)
    }
}

class Context private constructor(private val variables: Map<Variable, Expression>) {
    companion object {
        val init = Context(emptyMap())
    }

    operator fun get(variableName: String): Int = get(Variable(variableName))
    operator fun get(variable: Variable): Int = getExpression(variable)()
    private fun getExpression(v: Variable): Expression = variables.getValue(v)
    fun take(instruction: Instruction) =
        Context(variables + (instruction.assignTo to Expression.Cached(instruction.expr)))

    operator fun Expression.invoke() = calc(this@Context)
}

