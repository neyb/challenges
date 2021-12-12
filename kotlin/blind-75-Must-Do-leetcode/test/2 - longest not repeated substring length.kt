package exercice2

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

class LengthOfLongestSubstring {
    @Test fun `Longest Substring Without Repeating Characters of abcabcbb is 3`(){
        expect(lengthOfLongestSubstring("abcabcbb")).toEqual(3)
    }
}