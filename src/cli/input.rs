#[derive(Debug)]
pub enum Error {
    TryFromStr(String),
}

pub enum Input {
    Quit,
    Help,
    Revert,
    Clear,
    Top(usize),
    Pattern(String),
}

impl TryFrom<&str> for Input {
    type Error = self::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ":h" | ":help" => return Ok(Input::Help),
            ":q" | ":quit" => return Ok(Input::Quit),
            ":r" | ":revert" => return Ok(Input::Revert),
            ":c" | ":clear" => return Ok(Input::Clear),
            _ => (),
        }

        let regex = regex::Regex::new(r"^:top\s+(\d+)$").unwrap();
        if let Some(caps) = regex.captures(value) {
            let cap = caps.get(1).map_or("", |m| m.as_str());
            if let Ok(n) = cap.parse::<usize>() {
                return Ok(Input::Top(n));
            }
        }

        let regex =
            regex::Regex::new(r"^(?:#[a-z]|\?[a-z]|![a-z])(?:\s(?:#[a-z]|\?[a-z]|![a-z]))*$")
                .unwrap();
        if regex.is_match(value) {
            return Ok(Input::Pattern(value.to_owned()));
        }

        Err(Error::TryFromStr(value.to_owned()))
    }
}
