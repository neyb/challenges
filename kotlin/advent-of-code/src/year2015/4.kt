package year2015.day4

import common.*
import java.util.stream.Stream

fun main() {
    println(run("00000"))
    println(run("000000"))
}

val run = { input:String ->
    Stream.iterate(0) { it + 1 }.parallel()
        .filter { md5("$input$it").startsWith("00000") }
        .findFirst().get()
}