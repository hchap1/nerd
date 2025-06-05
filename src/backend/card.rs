use super::filemanager::Serial;

pub const DELIM: &str = "<!>";
pub const VDELIM: &str = "<,>";
pub const FDELIM: &str = "<f>";

pub struct Basic {
    question: String,
    answer: String,
    hint: Option<String>,
}

impl Serial for Basic {
    fn serialize(&self) -> String {
        format!(
            "basic{DELIM}{}{DELIM}{}{DELIM}{}",
            self.question,
            self.answer,
            match self.hint {
                Some(hint) => hint.as_str(),
                None => "none"
            }
        )
    }

    fn deserialize(serial: String) -> Result<Self, ()> {
        let parts: Vec<String> = serial.split(DELIM).map(|x| x.to_string()).collect();
        
        if parts.len() != 4 {
            return Err(());
        }

        if parts[0].as_str() != "basic" {
            return Err(());
        }

        Ok(Self {
            question: parts[1],
            answer: parts[2],
            hint: match parts[3].as_str() {
                "none" => None,
                value => Some(value.to_string())
            }
        })
    }
}

pub struct Switch {
    question: String,
    answer: String,
    
    forward_hint: Option<String>,
    reverse_hint: Option<String>
}

impl Serial for Switch {
    fn serialize(&self) -> String {
        format!(
            "switch{DELIM}{}{DELIM}{}{DELIM}{}{DELIM}{}",
            self.question,
            self.answer,
            match self.forward_hint {
                Some(hint) => hint.as_str(),
                None => "none"
            },
            match self.reverse_hint {
                Some(hint) => hint.as_str(),
                None => "none"
            }
        )
    }

    fn deserialize(serial: String) -> Result<Self, ()> {
        let parts: Vec<String> = serial.split(DELIM).map(|x| x.to_string()).collect();
        
        if parts.len() != 5 {
            return Err(());
        }

        if parts[0].as_str() != "switch" {
            return Err(());
        }

        Ok(Self { 
            question: parts[1],
            answer: parts[2],
            forward_hint: match parts[3].as_str() {
                "none" => None,
                value => Some(value.to_string())
            },
            reverse_hint: match parts[4].as_str() {
                "none" => None,
                value => Some(value.to_string())
            }
        })
    }
}

pub enum Field {
    Statement(String),
    Blank(String)
}

impl Serial for Field {
    fn serialize(&self) -> String {
        let (tag, value) = match self {
            Field::Statement(value) => ("statement", value.as_str()),
            Field::Blank(value) => ("blank", value.as_str())
        };

        format!(
            "{}{FDELIM}{}",
            tag, value
        )
    }

    fn deserialize(serial: String) -> Result<Self, ()> {
        let parts: Vec<String> = serial.split(FDELIM).map(|x| x.to_string()).collect();
        
        if parts.len() != 2 {
            return Err(());
        }

        match parts[0].as_str() {
            "statement" => Ok(Self::Statement(parts[1])),
            "blank" => Ok(Self::Blank(parts[1])),
            _ => Err(())
        }
    }
}

pub struct Cloze {
    statement: Vec<Field>,
    show_options: bool
}

impl Serial for Cloze {
    fn serialize(&self) -> String {
        format!(
            "cloze{DELIM}{}{DELIM}{}",
            self.statement
                .iter()
                .map(|f| f.serialize())
                .collect::<Vec<String>>()
                .join(VDELIM),
            self.show_options.to_string()
        )
    }

    fn deserialize(serial: String) -> Result<Self, ()> {
        
    }
}

pub enum Card {
    Basic(Basic),
    Switch(Switch),
    Cloze(Cloze)
}

impl Serial for Card {
    fn serialize(&self) -> String {
        
    }

    fn deserialize(serial: String) -> Result<Self, ()> {

    }
}
