use skim::prelude::{Cow, SkimItem};

pub enum Target {
    TERMINAL,
    DESKTOP,
}

pub struct Item {
    pub label: String,
    pub target: Target,
    pub cmd: String,
    pub description: String,
}

impl Item {
    pub fn new(label: String, target: Target, cmd: String, description: String) -> Self {
        Item {
            label,
            target,
            cmd,
            description,
        }
    }
    pub fn icon(&self) -> char {
        match &self.target {
            Target::DESKTOP => '',
            Target::TERMINAL => '',
        }
    }

    pub fn title(&self) -> String {
        format!("{} {}", &self.icon(), &self.label)
    }
}

impl SkimItem for Item {
    fn text(&self) -> Cow<str> {
        Cow::Owned(self.title())
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.cmd)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_desktop_launcher_item() {
        let item = Item::new(
            "Test".to_string(),
            Target::DESKTOP,
            "Test command".to_string(),
            "Test description".to_string(),
        );
        assert_eq!(item.label, "Test");
        assert!(match item.target {
            Target::DESKTOP => true,
            Target::TERMINAL => false,
        });
        assert_eq!(item.cmd, "Test command");
        assert_eq!(item.description, "Test description");
    }

    #[test]
    fn create_terminal_launcher_item() {
        let item = Item::new(
            "Test".to_string(),
            Target::TERMINAL,
            "Test command".to_string(),
            "Test description".to_string(),
        );
        assert_eq!(item.label, "Test");
        assert!(match item.target {
            Target::DESKTOP => false,
            Target::TERMINAL => true,
        });
        assert_eq!(item.cmd, "Test command");
        assert_eq!(item.description, "Test description");
    }
}
