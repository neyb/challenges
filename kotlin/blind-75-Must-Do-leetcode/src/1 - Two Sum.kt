package exercice1;

fun twoSum(nums: IntArray, target: Int): IntArray {
    return searchSumIn(nums.toList(), target)?.let { intArrayOf(it.first, it.second) }!!
}

fun searchSumIn(input: List<Int>, searchedSum: Int): Pair<Int, Int>? {

    val complement = mutableMapOf<Int, Int>()
    val foundSecondIndex = input.asSequence()
        .withIndex()
        .indexOfFirst { (index, value) ->
            complement.contains(value).also {
                if (!it) complement[searchedSum - value] = index
            }
        }

    return if (foundSecondIndex >= 0) complement[input[foundSecondIndex]]!! to foundSecondIndex else null
}