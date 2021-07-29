use std::{
    fs::OpenOptions,
    io::{self},
    io::{Read, Write},
};
const CHAT_ENDING: &'static str = "_chat.txt";

// Push the typed text to a db with the friend you've been chatting with
//TODO: Refactor. Db shouldn't know about username and chat_friend.
// Format the text property at core.rs
pub fn push(text: &String, username: &String, chat_friend: &String) -> Result<(), io::Error> {
    let chat_path = format!("{}{}", chat_friend, CHAT_ENDING);
    let chat_file = OpenOptions::new().append(true).create(true).open(chat_path);

    match chat_file.is_ok() {
        true => {
            let mut file = chat_file.unwrap();
            let write_result = file.write_all(format!("{}: {}\n", username, text).as_bytes());
            write_result
        }
        false => Err(chat_file.unwrap_err()),
    }
}

pub fn fetch(chat_friend: &String) -> Result<String, io::Error> {
    let chat_path = format!("{}{}", chat_friend, CHAT_ENDING);
    let chat_file = OpenOptions::new().append(false).read(true).open(chat_path);

    match chat_file.is_ok() {
        true => {
            let mut file = chat_file.unwrap();
            let mut content = Default::default();
            let _write_result = file.read_to_end(&mut content);
            Ok(String::from_utf8(content).unwrap())
        }
        false => Err(chat_file.unwrap_err()),
    }
}
