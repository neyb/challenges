package common.io

import java.io.Closeable
import java.io.Reader

fun Reader.toSequence() = ReaderSequence(this)

class ReaderSequence(private val reader: Reader) : Sequence<Char>, Closeable by reader {
    private val iterator = object : Iterator<Char> {
        var next = reader.read()

        override fun hasNext() = next >= 0

        override fun next() = (next.toChar()).also {
            next = reader.read()
        }
    }

    override fun iterator() = iterator
}