package com.mahanrip.chimera

import android.media.AudioAttributes
import android.media.AudioFormat
import android.media.AudioTrack
import android.os.Process
import android.webkit.JavascriptInterface
import java.util.concurrent.ConcurrentLinkedQueue
import java.util.concurrent.atomic.AtomicBoolean
import kotlin.math.sqrt

class NativeAudio {
    companion object {
        init { System.loadLibrary("chimera_android") }
        @JvmStatic private external fun nativeCreate(rate: Int): Long
        @JvmStatic private external fun nativeDestroy(handle: Long)
        @JvmStatic private external fun nativeRender(handle: Long, output: FloatArray)
        @JvmStatic private external fun nativeCommand(handle: Long, json: String): String
    }

    private val active = AtomicBoolean(false)
    private val commands = ConcurrentLinkedQueue<String>()
    @Volatile private var handle = 0L
    @Volatile private var track: AudioTrack? = null
    @Volatile private var worker: Thread? = null
    @Volatile private var rms = 0f
    private val rate = 32000

    @JavascriptInterface fun sampleRate(): Int = rate
    @JavascriptInterface fun running(): Boolean = active.get()
    @JavascriptInterface fun level(): Float = rms

    @Synchronized @JavascriptInterface fun start(): Boolean {
        if (active.get()) return true
        val minBytes = AudioTrack.getMinBufferSize(rate, AudioFormat.CHANNEL_OUT_MONO, AudioFormat.ENCODING_PCM_FLOAT)
        if (minBytes <= 0) return false
        val audio = AudioTrack.Builder()
            .setAudioAttributes(AudioAttributes.Builder().setUsage(AudioAttributes.USAGE_MEDIA).setContentType(AudioAttributes.CONTENT_TYPE_MUSIC).build())
            .setAudioFormat(AudioFormat.Builder().setSampleRate(rate).setEncoding(AudioFormat.ENCODING_PCM_FLOAT).setChannelMask(AudioFormat.CHANNEL_OUT_MONO).build())
            .setTransferMode(AudioTrack.MODE_STREAM)
            .setBufferSizeInBytes(maxOf(minBytes * 2, 8192))
            .setPerformanceMode(AudioTrack.PERFORMANCE_MODE_LOW_LATENCY)
            .build()
        if (audio.state != AudioTrack.STATE_INITIALIZED) { audio.release(); return false }
        audio.setVolume(0.8f)
        val engine = nativeCreate(rate)
        if (engine == 0L) { audio.release(); return false }
        handle = engine
        track = audio
        active.set(true)
        audio.play()
        worker = Thread({ renderLoop(engine, audio) }, "CHIMERA-Audio").apply { start() }
        return true
    }

    private fun renderLoop(engine: Long, audio: AudioTrack) {
        Process.setThreadPriority(Process.THREAD_PRIORITY_AUDIO)
        val block = FloatArray(256)
        while (active.get() && handle == engine) {
            while (true) {
                val command = commands.poll() ?: break
                nativeCommand(engine, command)
            }
            nativeRender(engine, block)
            var sum = 0.0
            for (sample in block) sum += sample.toDouble() * sample.toDouble()
            rms = sqrt(sum / block.size).toFloat()
            var offset = 0
            while (offset < block.size && active.get()) {
                val written = audio.write(block, offset, block.size - offset, AudioTrack.WRITE_BLOCKING)
                if (written <= 0) break
                offset += written
            }
        }
    }

    @JavascriptInterface fun command(json: String): String {
        val current = handle
        if (!active.get() || current == 0L) return "{\"error\":\"audio is not started\"}"
        commands.offer(json)
        return "{\"queued\":true}"
    }

    @JavascriptInterface fun setVolume(value: Float) { track?.setVolume(value.coerceIn(0f, 1f)) }

    @Synchronized @JavascriptInterface fun stop() {
        if (!active.getAndSet(false)) return
        commands.clear()
        val audio = track
        try { audio?.pause(); audio?.flush(); audio?.stop() } catch (_: IllegalStateException) { }
        worker?.join()
        audio?.release()
        track = null
        worker = null
        val old = handle
        handle = 0L
        if (old != 0L) nativeDestroy(old)
    }
}
