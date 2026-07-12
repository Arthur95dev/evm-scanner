use std::fmt::Display;

#[derive(Debug)]
pub enum Severity {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub struct Finding {
    pub severity: Severity,
    pub title: String,
    pub offset: usize,
}

impl Display for Finding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. Offset: {}", self.title, self.offset)
    }
}