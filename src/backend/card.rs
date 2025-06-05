pub struct Basic {
    question: String,
    answer: String,
    hint: Option<String>,
}

pub struct Switch {
    question: String,
    answer: String,
    
    forward_hint: Option<String>,
    reverse_hint: Option<String>
}

pub enum Field {
    Statement(String),
    Blank(String)
}

pub struct Cloze {
    statement: Vec<Field>,
    show_options: bool
}

pub enum Card {
    Basic(Basic),
    Switch(Switch),
    Cloze(Cloze)
}
