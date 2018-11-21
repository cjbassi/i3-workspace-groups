use log::info;
use std::io::prelude::*;
use std::process::{Command, Stdio};

pub fn query_rofi(prompt: &str, options: Option<Vec<String>>) -> Option<String> {
    let input = match options {
        Some(x) => x.join("\n"),
        None => String::new(),
    };
    let args = ["-dmenu", "-p", prompt, "-i"];
    info!("Running command: `rofi {}`", args.join(" "));
    let child = Command::new("rofi")
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start rofi command");
    child
        .stdin
        .expect("failed to connect to rofi stdin")
        .write_all(input.as_bytes())
        .expect("faild to write to rofi stdin");
    let mut output = String::new();
    child
        .stdout
        .expect("failed to connect to rofi stdout")
        .read_to_string(&mut output)
        .expect("failed to read from rofi stdout");
    output = output.trim().to_owned();
    match output.as_ref() {
        "" => None,
        _ => Some(output),
    }
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
