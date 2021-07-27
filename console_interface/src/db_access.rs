pub struct ChatPair {
    user: String,
    text: String,
}

impl ChatPair {}

const USER: &str = "A_Fucking_Genius";
const TEXT: &str = "I'M A FUCKING GENIUS!!!";

//TODO: Implement properly
pub fn fetch() -> Vec<ChatPair> {
    //For testing purposes, currently
    return vec![
        ChatPair {
            user: USER.to_string(),
            text: TEXT.to_string(),
        },
        ChatPair {
            user: USER.to_string(),
            text: TEXT.to_string(),
        },
        ChatPair {
            user: USER.to_string(),
            text: TEXT.to_string(),
        },
    ];
}

//Return code of execution
pub fn send(input: &str) -> Result<(), i32> {
    Ok(())
}
