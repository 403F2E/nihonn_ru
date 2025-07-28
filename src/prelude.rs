use thiserror::Error;

#[derive(Error, Debug)]
pub enum LogApp<'a> {
    #[error("I wish u learned somthing 🙂")]
    GoodBye,
    #[error("An error occured when reaching to the api")]
    ErrorApi,
    #[error("Command Error: {0}")]
    ErrorCommand(&'a str),
    #[error("An error has occured when trying to peak: \n{0}")]
    ErrorSpeak(Box<dyn std::error::Error>),
}

// all the available commands
pub const QUIT: [&str; 2] = ["exit", "e"];
pub const CLEAR: [&str; 2] = ["clear", "c"];
pub const HELP: [&str; 3] = ["help", "h", "?"];
pub const WORD: [&str; 2] = ["search", "s"];
pub const PLAY: [&str; 2] = ["play", "p"];
pub const EXPLAIN: [&str; 2] = ["explain", "x"];
pub const READING: [&str; 2] = ["reads", "r"];
pub const DEFINITION: [&str; 2] = ["define", "d"];

// all way to use for all commands
pub const USE_WORD: &str = "search [or s] (WORD)";
pub const USE_PLAY: &str = "The correct form is : 
\tplay [or p] (NUMBER: number of the wanted word. or. WORD: a word of your chose to get played)
                            examples:
                            \tplay 1
                            \tplay 林檎";
pub const USE_EXPLAIN: &str = "The correct form is : 
\texplain [or x] (NUMBER,NUMBER,...: a number or more of the wanted explanations. or. all: to show all the explanations)
                        \texplain all
                        \tx 1,2,3";
pub const USE_DEFINITION: &str = "The correct form is : 
\tdefine [or d] (NUMBER,NUMBER,...: a number or more of the wanted defined. or. all: to show all the definitions)
                        \td all
                        \tdefine 1,2,3 all";
pub const USE_READING: &str = "The correct form is : 
\treads [or r] (NUMBER,NUMBER,...: a number or more of the wanted defined. or. all: to show all the definitions)
                        \tr all
                        \treads 1,2,3 all";
