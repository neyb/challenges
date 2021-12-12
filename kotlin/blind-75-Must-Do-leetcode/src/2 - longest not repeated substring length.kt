package exercice2


fun lengthOfLongestSubstring(s: String): Int {
    class ActualState(
        var fromIndex: Int = 0,
        var toIndex: Int = -1,
        val lastIndexByChar: Array<Int?> = arrayOfNulls(256),
//        val lastIndexByChar: MutableMap<Char, Int> = mutableMapOf()
                     ) {

        val size get() = toIndex - fromIndex + 1

        fun take(nextChar: Int): ActualState {
            val previouslyPresentIndex = lastIndexByChar[nextChar]
            toIndex += 1
            lastIndexByChar[nextChar] = toIndex

            if (previouslyPresentIndex != null && previouslyPresentIndex >= fromIndex) {
                fromIndex = previouslyPresentIndex + 1
            }

            return this;
        }


    }

    return s.asSequence()
        .fold(ActualState() to 0) { (state, longestThatFar), char ->
            state.take(char.code) to
                    Math.max(longestThatFar, state.size)
        }.second


}


