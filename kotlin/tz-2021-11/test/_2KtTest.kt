import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

internal class _2KtTest {

    @Test
    fun run_1() {
        val l = listOf("5", "20:04", "20:23", "08:00", "09:15", "13:00")
        assertEquals(run(l), "OK")
    }
}