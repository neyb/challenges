fun main(args : Array<String>) {
    val input = generateSequence(::readLine)
    val lines = input.toList()

    run3(lines).map{ " ${it}" }.forEach(::print)

}

fun run3(lines:List<String>):List<Int>{
    val byChef: Map<String, List<Relation>> = lines.mapNotNull { Relation.parse(it) }
        .groupBy { it.chef }

    val result = mutableListOf<Int>()

    var current: List<String> = listOf("0")
    result.add(current.size)
    (2..10).forEach {
        current = current.flatMap { (byChef[it]?: listOf()).map { it.fils } }
        result.add(current.size)
    }

    return result
}



data class Relation(val chef:String, val fils:String){
    companion object {
        fun parse(s:String) = Regex("(\\w+) (\\w+)").matchEntire(s)?.let { Relation(it.groupValues[2], it.groupValues[1]) }
    }
}