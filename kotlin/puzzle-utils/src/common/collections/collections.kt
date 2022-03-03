package common.collections

operator fun <T> List<T>.plus(tailElement: T): List<T> = FastSingleAppendList(this, tailElement)
