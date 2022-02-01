fun main() {
    val input = generateSequence(::readLine)
    val lines = input.toList()

    println(run(lines))
}

fun run(lines:List<String>):String {
    val nb = lines
        .drop(1)
        .mapNotNull { Regex("(\\d\\d):(\\d\\d)").matchEntire(it) }
        .count { it.groupValues[1].toInt().let { it >= 20 || it < 8 } }

    System.err.println("nb lines ${lines.size}")
    System.err.println("/2 ${lines.size / 2}")
    System.err.println("nb ${nb}")

    return if(nb >= (lines.size / 2)) "SUSPICIOUS"
    else "OK"
}