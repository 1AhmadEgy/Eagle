package com.eagle.app

import android.app.Activity
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class SmokeTest {
    @Test
    fun mainActivityRemainsAndroidEntryPoint() {
        assertTrue(
            "MainActivity must remain an Android Activity",
            Activity::class.java.isAssignableFrom(MainActivity::class.java)
        )
        assertEquals(
            "com.eagle.app.MainActivity",
            MainActivity::class.java.name
        )
    }
}
