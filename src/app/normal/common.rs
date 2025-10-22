#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputPart {
    #[default]
    Key,
    Value,
}
