use inquire::{autocompletion::Replacement, Autocomplete, CustomUserError};

#[derive(Clone, Debug)]
pub struct CommandAutoComplete {
    commands: Vec<String>,
    lcp: String,
}

impl CommandAutoComplete {
    pub fn new() -> Self {
        Self {
            commands: vec![
                "clone", "init", "add", "mv", "restore", "rm", "bisect", "diff", "grep", "log",
                "show", "status", "branch", "commit", "merge", "rebase", "reset", "switch", "tag",
                "fetch", "pull", "push",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            lcp: String::new(),
        }
    }

    fn get_autocomplete(&mut self, input: &str) -> Vec<String> {
        let mut cmds = self.commands.clone();
        cmds.retain(|s| s.contains(input));
        cmds.sort_by(|a, b| {
            a.find(input)
                .get_or_insert(0)
                .cmp(&b.find(input).get_or_insert(0))
        });

        if cmds.len() == 1 {
            self.lcp = cmds[0].clone();
        }

        cmds
    }
}

impl Autocomplete for CommandAutoComplete {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        Ok(self.get_autocomplete(input))
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
        let _ = self.get_autocomplete(input);

        Ok(match highlighted_suggestion {
            Some(suggestion) => Replacement::Some(suggestion),
            None => match self.lcp.is_empty() {
                true => Replacement::None,
                false => Replacement::Some(self.lcp.clone()),
            },
        })
    }
}
