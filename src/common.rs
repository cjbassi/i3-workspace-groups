use log::info;
use std::io::prelude::*;
use std::process::{Command, Stdio};

pub fn query_rofi(prompt: &str, options: Option<Vec<String>>) -> Option<String> {
    let input = match options {
        Some(x) => x.join("\n"),
        None => "".to_owned(),
    };
    let input_bytes = input.as_bytes();
    let args = ["-dmenu", "-p", prompt, "-i"];
    info!("Running command: `rofi {}`", args.join(" "));
    let process = Command::new("rofi")
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("could not execute rofi command");
    process
        .stdin
        .expect("could not connect to rofi stdin")
        .write_all(input_bytes)
        .expect("could not write to rofi stdin");
    let mut s = String::new();
    process
        .stdout
        .expect("could not connect to rofi stdou")
        .read_to_string(&mut s)
        .expect("could not read from rofi stdin");
    s = s.trim().to_owned();
    if s == "" {
        return None;
    }
    Some(s)
}

// https://stackoverflow.com/questions/51344951/how-do-you-unwrap-a-result-on-ok-or-return-from-the-function-on-err
macro_rules! unwrap_option_or_return {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return,
        }
    };
}
