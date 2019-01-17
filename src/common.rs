use log::info;
use std::{
    io::prelude::*,
    process::{Command, Stdio},
};

pub fn query_rofi(prompt: &str, options: Option<&[&str]>) -> Option<String> {
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

macro_rules! keys_to_strptr_vec {
    ( $e: expr ) => {
        $e.keys().map(|key| &key[..]).collect::<Vec<&str>>();
    };
}
