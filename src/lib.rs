pub trait Style {
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
    fn magenta(&self) -> String;
    fn cyan(&self) -> String;
}

impl Style for &str {
    fn red(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", self)
    }
    fn green(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", self)
    }
    fn yellow(&self) -> String {
        format!("\x1b[33m{}\x1b[0m", self)
    }
    fn blue(&self) -> String {
        format!("\x1b[34m{}\x1b[0m", self)
    }
    fn magenta(&self) -> String {
        format!("\x1b[35m{}\x1b[0m", self)
    }
    fn cyan(&self) -> String {
        format!("\x1b[36m{}\x1b[0m", self)
    }
}

impl Style for String {
    fn red(&self) -> String { self.as_str().red() }
    fn green(&self) -> String { self.as_str().green() }
    fn yellow(&self) -> String { self.as_str().yellow() }
    fn blue(&self) -> String { self.as_str().blue() }
    fn magenta(&self) -> String { self.as_str().magenta() }
    fn cyan(&self) -> String { self.as_str().cyan() }
}
