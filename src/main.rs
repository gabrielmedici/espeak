use std::{ffi::c_char, i32, ptr::null};

#[repr(C)]
enum espeak_AUDIO_OUTPUT {
    /* PLAYBACK mode: plays the audio data, supplies events to the calling program*/
    AUDIO_OUTPUT_PLAYBACK,

    /* RETRIEVAL mode: supplies audio data and events to the calling program */
    AUDIO_OUTPUT_RETRIEVAL,

    /* SYNCHRONOUS mode: as RETRIEVAL but doesn't return until synthesis is completed */
    AUDIO_OUTPUT_SYNCHRONOUS,

    /* Synchronous playback */
    AUDIO_OUTPUT_SYNCH_PLAYBACK,
}

extern "C" {
    fn espeak_Initialize(
        output_type: espeak_AUDIO_OUTPUT,
        buf_length: i32,
        path: *const c_char,
        options: i32,
    ) -> i32;
}
fn main() {
    unsafe {
        espeak_Initialize(
            espeak_AUDIO_OUTPUT::AUDIO_OUTPUT_SYNCHRONOUS,
            0,
            null(),
            0x8000,
        );
    }
    println!("Hello, world!");
}
