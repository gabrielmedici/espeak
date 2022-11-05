fn main() {
    let files = [
        "src/speak_lib.cpp",
        "src/compiledict.cpp",
        "src/dictionary.cpp",
        "src/intonation.cpp",
        "src/readclause.cpp",
        "src/setlengths.cpp",
        "src/numbers.cpp",
        "src/synth_mbrola.cpp",
        "src/synthdata.cpp",
        "src/synthesize.cpp",
        "src/translate.cpp",
        //"src/mbrowrap.cpp",
        "src/tr_languages.cpp",
        "src/voices.cpp",
        "src/wavegen.cpp",
        "src/phonemelist.cpp",
        "src/espeak_command.cpp",
        "src/event.cpp",
        "src/fifo.cpp",
        "src/wave.cpp",
        "src/debug.cpp",
        "src/klatt.cpp",
        "src/sonic.cpp",
    ];

    cc::Build::new()
        .cpp(true)
        .files(files)
        //.file("foo.c")
        //.shared_flag(true)
        .include("src")
        .define("PLATFORM_WINDOWS", Some("1"))
        .compile("espeak-1");
}
