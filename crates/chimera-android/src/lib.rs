//! Android JNI host for the shared CHIMERA Rust engine.
//! Audio rendering is pulled by Android's AudioTrack thread, never by WebView/JavaScript.
use jni::JNIEnv;
use jni::objects::{JClass, JFloatArray, JString};
use jni::sys::{jint, jlong, jstring};
use serde_json::Value;
use std::ffi::c_void;

unsafe extern "C" {
    fn engine_new(sample_rate: f64) -> *mut c_void;
    fn engine_free(handle: *mut c_void);
    fn engine_render(handle: *mut c_void, frames: usize) -> *const f32;
    fn engine_set_gate(handle: *mut c_void, level: f64);
    fn engine_set_body(handle: *mut c_void, female: f64, child: f64);
    fn engine_stop_speech(handle: *mut c_void);
    fn engine_release(handle: *mut c_void, tag: f64);
    fn engine_values(handle: *mut c_void) -> *mut f64;
    fn engine_set_artic(handle: *mut c_void, tip_free: u32);
    fn engine_set_source(handle: *mut c_void);
    fn engine_records(handle: *mut c_void) -> *mut u8;
    fn engine_schedule(handle: *mut c_void, count: usize, lead: f64, replace: u32) -> f64;
    fn engine_set_human(handle: *mut c_void);
    fn engine_set_creature(handle: *mut c_void, ptr: *const u8, len: usize) -> u32;
    fn engine_play_call(handle: *mut c_void, index: usize, pitch: f64, amp: f64);
    fn engine_stop_call(handle: *mut c_void);
    fn engine_set_beat(
        handle: *mut c_void,
        has_steps: u32,
        count: usize,
        bpm: f64,
        swing: f64,
        playing: u32,
    );
    fn engine_beat_hit(handle: *mut c_void);
    fn chimera_result_ptr() -> *const u8;
    fn chimera_result_len() -> usize;
    fn engine_creature_param(handle: *mut c_void, index: usize, value: f64);
}

fn handle(raw: jlong) -> *mut c_void {
    raw as *mut c_void
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_mahanrip_chimera_NativeAudio_nativeCreate(
    _env: JNIEnv,
    _class: JClass,
    rate: jint,
) -> jlong {
    unsafe { engine_new(rate.max(16000) as f64) as jlong }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_mahanrip_chimera_NativeAudio_nativeDestroy(
    _env: JNIEnv,
    _class: JClass,
    raw: jlong,
) {
    if raw != 0 {
        unsafe {
            engine_free(handle(raw));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_mahanrip_chimera_NativeAudio_nativeRender(
    env: JNIEnv,
    _class: JClass,
    raw: jlong,
    output: JFloatArray,
) {
    if raw == 0 {
        return;
    }
    let Ok(n) = env.get_array_length(&output) else {
        return;
    };
    let ptr = unsafe { engine_render(handle(raw), n as usize) };
    if ptr.is_null() {
        return;
    }
    let samples = unsafe { std::slice::from_raw_parts(ptr, n as usize) };
    let _ = env.set_float_array_region(&output, 0, samples);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_mahanrip_chimera_NativeAudio_nativeCommand(
    mut env: JNIEnv,
    _class: JClass,
    raw: jlong,
    input: JString,
) -> jstring {
    let result = (|| -> Result<String, String> {
        if raw == 0 {
            return Err("native engine is not running".into());
        }
        let text: String = env.get_string(&input).map_err(|e| e.to_string())?.into();
        let msg: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        let h = handle(raw);
        let kind = msg.get("type").and_then(Value::as_str).unwrap_or("");
        unsafe {
            match kind {
                "gate" => engine_set_gate(h, msg["level"].as_f64().unwrap_or(0.0).clamp(0.0, 1.0)),
                "body" => engine_set_body(
                    h,
                    msg["body"]["female"].as_f64().unwrap_or(0.0),
                    msg["body"]["child"].as_f64().unwrap_or(0.0),
                ),
                "stopSpeech" => engine_stop_speech(h),
                "release" => engine_release(h, msg["tag"].as_f64().unwrap_or(0.0)),
                "artic" => {
                    let values = std::slice::from_raw_parts_mut(engine_values(h), 32);
                    values[..17].fill(f64::NAN);
                    let keys = [
                        "jaw",
                        "tbX",
                        "tbY",
                        "ttX",
                        "ttY",
                        "ttCurl",
                        "root",
                        "lipAp",
                        "lipProt",
                        "lipWidth",
                        "labiodental",
                        "velum",
                        "larynx",
                        "epilarynx",
                        "ventricular",
                        "lateral",
                        "dorsum",
                    ];
                    for (i, key) in keys.iter().enumerate() {
                        if let Some(v) = msg["values"][*key].as_f64() {
                            values[i] = v;
                        }
                    }
                    let tip = match msg.get("tipFree").and_then(Value::as_bool) {
                        Some(true) => 1,
                        Some(false) => 0,
                        None => 2,
                    };
                    engine_set_artic(h, tip);
                }
                "source" => {
                    let values = std::slice::from_raw_parts_mut(engine_values(h), 32);
                    values[0..12].fill(f64::NAN);
                    let keys = [
                        "f0",
                        "glide",
                        "pressure",
                        "breathiness",
                        "tension",
                        "fry",
                        "roughness",
                        "vibratoDepth",
                        "vibratoRate",
                        "drift",
                        "voiced",
                        "brightness",
                    ];
                    for (i, key) in keys.iter().enumerate() {
                        if let Some(v) = msg["values"][*key].as_f64() {
                            values[i] = v;
                        } else if let Some(v) = msg["values"][*key].as_bool() {
                            values[i] = if v { 1.0 } else { 0.0 };
                        }
                    }
                    engine_set_source(h);
                }
                "schedule" => {
                    let gestures: Vec<chimera::control::score::Gesture> =
                        serde_json::from_value(msg["gestures"].clone())
                            .map_err(|e| e.to_string())?;
                    let n = gestures.len().min(4000);
                    let records = std::slice::from_raw_parts_mut(
                        engine_records(h),
                        n * chimera_wasm::GESTURE_RECORD,
                    );
                    for (i, gesture) in gestures.iter().take(n).enumerate() {
                        chimera_wasm::write_gesture(
                            gesture,
                            &mut records[i * chimera_wasm::GESTURE_RECORD
                                ..(i + 1) * chimera_wasm::GESTURE_RECORD],
                        );
                    }
                    let lead = msg["lead"].as_f64().unwrap_or(0.03);
                    let off = engine_schedule(
                        h,
                        n,
                        lead,
                        if msg["replace"].as_bool().unwrap_or(false) {
                            1
                        } else {
                            0
                        },
                    );
                    return Ok(serde_json::json!({"ok":true,"offset":off}).to_string());
                }
                "creature" => {
                    let id = msg["id"].as_str().unwrap_or("human");
                    if id == "human" {
                        engine_set_human(h);
                    } else {
                        let bytes = text.as_bytes();
                        let status = engine_set_creature(h, bytes.as_ptr(), bytes.len());
                        if status != 0 {
                            return Err("could not select creature".into());
                        }
                    }
                }
                "cparams" => {
                    let len = chimera_result_len();
                    let ptr = chimera_result_ptr();
                    if !ptr.is_null() && len > 0 {
                        let bytes = std::slice::from_raw_parts(ptr, len);
                        if let Ok(info) = serde_json::from_slice::<Value>(bytes) {
                            if let Some(values) = msg["values"].as_object() {
                                if let Some(params) = info["params"].as_array() {
                                    for (key, value) in values {
                                        if let Some(index) =
                                            params.iter().position(|v| v.as_str() == Some(key))
                                        {
                                            if let Some(number) = value.as_f64() {
                                                engine_creature_param(h, index, number);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                "call" => {
                    let len = chimera_result_len();
                    let ptr = chimera_result_ptr();
                    if !ptr.is_null() && len > 0 {
                        let bytes = std::slice::from_raw_parts(ptr, len);
                        if let Ok(info) = serde_json::from_slice::<Value>(bytes) {
                            let id = msg["id"].as_str().unwrap_or("");
                            if let Some(index) = info["calls"]
                                .as_array()
                                .and_then(|a| a.iter().position(|v| v.as_str() == Some(id)))
                            {
                                engine_play_call(
                                    h,
                                    index,
                                    msg["pitch"].as_f64().unwrap_or(f64::NAN),
                                    msg["amp"].as_f64().unwrap_or(f64::NAN),
                                );
                            }
                        }
                    }
                }
                "beat" => {
                    let steps = msg.get("steps").and_then(Value::as_array);
                    let count = steps.map_or(0, |a| a.len().min(128));
                    if let Some(items) = steps {
                        let records = std::slice::from_raw_parts_mut(
                            engine_records(h),
                            count * chimera_wasm::STEP_RECORD,
                        );
                        for i in 0..count {
                            let row = &mut records[i * chimera_wasm::STEP_RECORD
                                ..(i + 1) * chimera_wasm::STEP_RECORD];
                            row.fill(0);
                            if items[i].is_null() {
                                row[0] = 255;
                                continue;
                            }
                            let id = items[i]["id"].as_str().unwrap_or("").as_bytes();
                            let len = id.len().min(24);
                            row[0] = len as u8;
                            row[1..1 + len].copy_from_slice(&id[..len]);
                            let note = items[i]["note"].as_f64().unwrap_or(f64::NAN);
                            let accent = items[i]["accent"].as_f64().unwrap_or(f64::NAN);
                            row[32..40].copy_from_slice(&note.to_le_bytes());
                            row[40..48].copy_from_slice(&accent.to_le_bytes());
                        }
                    }
                    engine_set_beat(
                        h,
                        if steps.is_some() { 1 } else { 0 },
                        count,
                        msg["bpm"].as_f64().unwrap_or(f64::NAN),
                        msg["swing"].as_f64().unwrap_or(f64::NAN),
                        match msg.get("playing").and_then(Value::as_bool) {
                            Some(true) => 1,
                            Some(false) => 0,
                            None => 2,
                        },
                    );
                }
                "beatHit" => {
                    let records = std::slice::from_raw_parts_mut(
                        engine_records(h),
                        chimera_wasm::STEP_RECORD,
                    );
                    records.fill(0);
                    let id = msg["id"].as_str().unwrap_or("").as_bytes();
                    let len = id.len().min(24);
                    records[0] = len as u8;
                    records[1..1 + len].copy_from_slice(&id[..len]);
                    records[32..40]
                        .copy_from_slice(&msg["note"].as_f64().unwrap_or(f64::NAN).to_le_bytes());
                    records[40..48]
                        .copy_from_slice(&msg["accent"].as_f64().unwrap_or(f64::NAN).to_le_bytes());
                    engine_beat_hit(h);
                }
                "stopCall" => engine_stop_call(h),
                _ => return Err(format!("unsupported native command: {kind}")),
            }
        }
        Ok("{\"ok\":true}".into())
    })();
    let out = result.unwrap_or_else(|e| serde_json::json!({"error":e}).to_string());
    env.new_string(out)
        .map(JString::into_raw)
        .unwrap_or(std::ptr::null_mut())
}
