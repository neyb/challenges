package common.`object`

import java.util.*

fun hash(vararg vs: Any?) = Objects.hash(*vs)
inline fun <reified T> T.eq(other: Any?, vararg fs: T.() -> Any?): Boolean =
    if (other is T) fs.fold(true) { acc, f -> acc && other.f() == f() }
    else false