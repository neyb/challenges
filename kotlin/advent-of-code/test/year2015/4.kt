package year2015

import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test
import year2015.day4.md5

class `4` {
    @Test fun `test md5`(){
        expect(md5("yzbqklnj")).toEqual("dd9391a66659d33f01cc20141ce540b8")
    }
}