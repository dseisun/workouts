use std::{time::Duration, thread};

use tts;

fn main() {
    env_logger::init();
    let mut tts = tts::Tts::new(tts::Backends::AvFoundation).unwrap();
    dbg!(tts.normal_volume());
    dbg!(tts.supported_features());
    let voices = tts.voices().unwrap();
    
    for (i, voice) in voices.iter().enumerate() {
        println!("Voice #: {} Voice: {:?}", i, &voice);
        tts.set_voice(voice);
        tts.speak("This is a test", false).unwrap();
        thread::sleep(Duration::from_secs(3));
    }

    let voice = &voices[33];
    dbg!(voice);
    dbg!(tts.set_voice(voice).unwrap());
    dbg!(tts.voice());
    
    // tts.set_volume(1_f32);
    
    let res = tts.speak("This is a test", false).unwrap();
    thread::sleep(Duration::from_secs(5));
    

}