package com.plugin.filepicker

import android.app.Activity
import android.content.Intent
import android.net.Uri
import androidx.activity.result.ActivityResult
import app.tauri.annotation.ActivityCallback
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@InvokeArg
class PingArgs {
    var value: String? = null
}

@TauriPlugin
class FilepickerPlugin(
    private val activity: Activity,
) : Plugin(activity) {
    private val implementation = Filepicker()

    @Command
    fun get_file_picker(invoke: Invoke) {
        val intent = Intent().setType("*/*").setAction(Intent.ACTION_GET_CONTENT)
        intent.putExtra(Intent.EXTRA_ALLOW_MULTIPLE, true)
        startActivityForResult(invoke, intent, "filePickerResult")
    }

    @ActivityCallback
    fun filePickerResult(invoke: Invoke, result: ActivityResult) {
        try {
            when (result.resultCode) {
                Activity.RESULT_OK -> {
                    val callResult = createPickFilesResult(result.data)
                    invoke.resolve(callResult)
                }
                Activity.RESULT_CANCELED -> invoke.reject("File picker cancelled")
                else -> invoke.reject("Failed to pick files")
            }
        } catch (ex: java.lang.Exception) {
            val message = ex.message ?: "Failed to read file pick result"
            invoke.reject(message)
        }
    }

    private fun createPickFilesResult(data: Intent?): JSObject {
        val callResult = JSObject()
        if (data == null) {
            callResult.put("files", null)
            return callResult
        }
        val uris: MutableList<String?> = ArrayList()
        if (data.clipData == null) {
            val uri: Uri? = data.data
            uris.add(uri?.toString())
        } else {
            for (i in 0 until data.clipData!!.itemCount) {
                val uri: Uri = data.clipData!!.getItemAt(i).uri
                uris.add(uri.toString())
            }
        }
        callResult.put("files", JSArray.from(uris.toTypedArray()))
        return callResult
    }



    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)

        val ret = JSObject()
        ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve(ret)
    }
}
