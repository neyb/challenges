fun main() {
    val input = generateSequence(::readLine)
    val lines = input.toList().count { it.matches(".*\\d{5}".toRegex())}
    println(lines)
}