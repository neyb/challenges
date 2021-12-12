import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

internal class _5KtTest {

    @Test fun `hash of BigBoss is 1548960877`(){
        assertEquals(1548960877, "BigBoss".hash())
    }

    @Test fun `run5 on BigBoss`(){
        val result = run5("BigBoss")
        assertEquals(result.hash(), "BigBoss".hash())
        assert(result.startsWith("alban"))
    }

    @Test fun `tryResolveWith hashdiff=33, nbChar=1 should return "!" (33, )`() {
        assertEquals("!", tryResolveWith(1, 33))
    }

    @Test fun `tryResolveWith hashdiff=33*31*33, nbChar=1 should return "!!" (33, 33)`() {
        assertEquals("!!", tryResolveWith(2, 33*31+33))
            }

}