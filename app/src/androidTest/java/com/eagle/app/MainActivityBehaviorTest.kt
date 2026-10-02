package com.eagle.app

import androidx.test.core.app.ActivityScenario
import androidx.test.espresso.Espresso.onView
import androidx.test.espresso.matcher.ViewMatchers.withText
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class MainActivityBehaviorTest {

    @Test
    fun launcherActivity_displaysTestLabLabel() {
        ActivityScenario.launch(MainActivity::class.java).use {
            onView(withText("Eagle Test Lab")).check { view, noViewFoundException ->
                if (noViewFoundException != null) throw noViewFoundException
                check(view.isShown) { "Expected Eagle Test Lab label to be visible" }
            }
        }
    }
}
