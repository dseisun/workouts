use std::{process::{Command, Output}, io::{self, Write}, thread, time::Duration, fmt::format};
//Os specific trait
pub(crate) trait Speak {
    //Implement this method however you want text to speech to work on a given operating system
    fn tts(&self, text: &str);
    fn write_to_shell(&self, output: &Output) {
        io::stdout().write_all(&output.stdout).unwrap();    
        io::stderr().write_all(&output.stderr).unwrap();    
    }
}

pub(crate) struct OSXSpeak;
impl Speak for OSXSpeak {
    fn tts(&self, text: &str) {
        let full_command = format!("echo {text} | say");
        let command_resp = Command::new("bash")
        .arg("-c")
        .arg(full_command)
        .output().unwrap();
        self.write_to_shell(&command_resp);
    }
}

//TODO There a way to allow the program to still run if there's no TTS? Make it configurable?
pub(crate) struct NotImplSpeak;
impl Speak for NotImplSpeak {
    fn tts(&self, text: &str) {
        panic!("TTS not implemented for OS")
    }
}

//TODO Write tests

pub(crate) fn get_speaker() -> Box<dyn Speak> {
    if cfg!(target_os = "macos") {
        Box::new(OSXSpeak)
    } else {
        Box::new(NotImplSpeak)
    }
}