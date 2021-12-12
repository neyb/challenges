import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

internal class _4KtTest {

    @Test
    fun run4() {
        assertEquals(

            run4(
                """5 4
        |11 22 33 44 55
        |1 3
        |0 1
        |2 2
        |2 4
        """.trimMargin("|").split("\n")
                ),
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"
                    )
    }

    @Test fun `rangeXorFor 11 22 33 44 55 for 0 0 is 11`() = assertEquals(rangeXorFor(listOf(11,22,33,44,55))(InclRange(0, 0)), 11)

    @Test fun `rangeXorFor 11 22 33 44 55 for 1 3 is 27`() = assertEquals(rangeXorFor(listOf(11,22,33,44,55))(InclRange(1, 3)), 27)
    @Test fun `rangeXorFor 11 22 33 44 55 for 0 1 is 29`() = assertEquals(rangeXorFor(listOf(11,22,33,44,55))(InclRange(0,1)), 29)
    @Test fun `rangeXorFor 11 22 33 44 55 for 2 2 is 33`() = assertEquals(rangeXorFor(listOf(11,22,33,44,55))(InclRange(2, 2)), 33)
    @Test fun `rangeXorFor 11 22 33 44 55 for 2 4 is 58`() = assertEquals(rangeXorFor(listOf(11,22,33,44,55))(InclRange(2, 4)), 58)
}