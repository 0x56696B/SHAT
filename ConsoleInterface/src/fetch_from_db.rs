pub struct ChatPair {
    user: String,
    text: String,
}

impl ChatPair {}

const USER: String = "A_Fucking_Genius".to_string();
const TEXT: String = "I'M A FUCKING GENIUS!!!".to_string();

//TODO: Implement properly
pub fn fetch() -> Vec<ChatPair> {
    //For testing purposes, currently
    return vec![
        ChatPair {
            user: USER,
            text: TEXT,
        },
        ChatPair {
            user: USER,
            text: TEXT,
        },
        ChatPair {
            user: USER,
            text: TEXT,
        },
    ];
}
