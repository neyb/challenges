import ch.tutteli.atrium.api.fluent.en_GB.toEqual
import ch.tutteli.atrium.api.verbs.expect
import org.junit.jupiter.api.Test

internal class _2KtTest {

    @Test
    fun run_1() {
        val l = listOf("5", "20:04", "20:23", "08:00", "09:15", "13:00")
        expect(run(l)).toEqual("OK")
    }
}