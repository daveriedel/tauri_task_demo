package com.plugin.dialer
import android.app.Activity
import android.content.Intent
import android.net.Uri
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin

@InvokeArg
internal class OpenArgs {
    lateinit var tel: String
}

@TauriPlugin
class DialerPlugin(
    private val activity: Activity,
) : Plugin(activity) {
    @Command
    fun dial(invoke: Invoke) {
        try {
            val args = invoke.parseArgs(OpenArgs::class.java)
            val number = args.tel

            if (number.isNullOrEmpty()) {
                invoke.reject("Missing or invalid phone number")
                return
            }

            val intent =
                Intent(Intent.ACTION_DIAL).apply {
                    data = Uri.parse("tel:$number")
                }

            activity.startActivity(intent)
            invoke.resolve(null)
        } catch (e: Exception) {
            invoke.reject("Failed to launch dialer: ${e.message}")
        }
    }
}
