package common

import kotlin.reflect.KClass

//inline fun <T> Sequence<T>.reduceOnly(test:(T) -> Boolean, reducer:(T,T)->) =;
inline fun <T : Any, ReduceResult : T, reified SubType : ReduceResult> Iterable<T>.joiningIfInstanceOf(
    subType: KClass<SubType>,
    crossinline reduce: (ReduceResult, SubType) -> ReduceResult
                                                                                                      )
        : List<T> = asSequence().joiningIfInstanceOf(subType, reduce).toList()

inline fun <T : Any, ReduceResult : T, reified SubType : ReduceResult> Sequence<T>.joiningIfInstanceOf(
    subType: KClass<SubType>,
    crossinline reduce: (ReduceResult, SubType) -> ReduceResult
                                                                                                      )
        : Sequence<T> {
    var joined: ReduceResult? = null

    return this
        .onEach { item -> if (item is SubType) joined = joined?.let { reduce(it, item) } ?: item }
        .filter { it !is SubType } + generateSequence { joined.also { joined = null } }
}

inline fun <T, reified SubType : T> Sequence<T>.flattenInstanceOf(crossinline flat: (SubType) -> Sequence<T>): Sequence<T> =
    flatMap { if (it is SubType) flat(it) else sequenceOf(it) }

inline fun <T : Any, reified SubType : T> Iterable<T>.flattenInstanceOf(
    subType: KClass<SubType>,
    crossinline flat: (SubType) -> Iterable<T>
                                                                       ): List<T> =
    flatMap { if (it is SubType) flat(it) else listOf(it) }

//Plus(
//                    expressions.filter { it !is Literal } +
//                            Literal(expressions.asSequence().filterIsInstance<Literal>().sumOf { it.value })
//                                  )