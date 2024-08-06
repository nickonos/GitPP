use inquire::Text;

pub fn handle_commit_message(message: Option<String>) -> Option<String> {
    let message_prompt = Text::new("What is your commit message?");

    match message {
        None => {
            let msg = message_prompt.prompt();

            match msg {
                Ok(str) => {
                    if str.len() < 1 {
                        return handle_commit_message(message);
                    }

                    Some(str)
                }
                Err(_) => None,
            }
        }
        Some(str) => Some(str),
    }
}
