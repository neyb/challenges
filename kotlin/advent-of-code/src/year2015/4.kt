package year2015.day4

import common.*
import java.util.stream.Stream

fun main() = run().forEach(::println)
val run = { sequenceOf(withInput("00000"), withInput("000000")) }

val withInput = { input: String ->
    Stream.iterate(0) { it + 1 }.parallel()
        .filter { md5("$input$it").startsWith("00000") }
        .findFirst().get()
}