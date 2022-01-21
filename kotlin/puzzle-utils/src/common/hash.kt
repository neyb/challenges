package common

import java.security.MessageDigest
import java.util.*

fun md5(s: String): String =
    MessageDigest.getInstance("MD5").digest(s.toByteArray())
        .let { HexFormat.of().formatHex(it) }