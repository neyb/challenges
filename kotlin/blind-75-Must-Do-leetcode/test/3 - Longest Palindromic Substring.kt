package `longest-palindromic-substring`

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect

class SolutionTest {
    val solution = Solution()

    fun `longestPalindrome(babad) is bab`(s: String) {
        with(solution) {
            expect(longestPalindrome("babad")).toEqual("bab")
        }
    }
}