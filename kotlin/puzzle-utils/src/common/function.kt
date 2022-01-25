package common

fun <I, O> ((I) -> O).memoize() =
    with(mutableMapOf<I, O>()) { { input: I -> computeIfAbsent(input, this@memoize) } }
