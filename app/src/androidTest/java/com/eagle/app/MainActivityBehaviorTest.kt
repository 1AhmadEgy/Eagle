package com.eagle.app

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.rule.ActivityTestRule
import androidx.test.espresso.Espresso.onView
import androidx.test.espresso.matcher.ViewMatchers.withText
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class MainActivityBehaviorTest {

    @get:Rule
    val activityRule = ActivityTestRule(MainActivity::class.java)

    @Test
    fun launcherActivity_displaysTestLabLabel() {
        onView(withText("Eagle Test Lab")).check { view, noViewFoundException ->
            if (noViewFoundException != null) throw noViewFoundException
            check(view.isShown) { "Expected Eagle Test Lab label to be visible" }
        }
    }
}
