use std::str::FromStr;
pub struct PasswordRule { //10.10
    pub from: usize,
    pub to: usize,
    pub letter: char,
}

impl PasswordRule {
    pub fn is_valid(&self, s: &str) -> bool {
        let count = s
            .chars()
            .filter(|c| c == &self.letter)
            .count();
        self.from <= count && count <= self.to
    }
}

impl FromStr for PasswordRule {  // 10.53
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s
            .split(|c| c=='-' || c == ' ')
            .collect();
        let from: usize = splitted[0]
            .parse()
            .unwrap();
        let to: usize = splitted[1]
            .parse()
            .unwrap();
        let letter: char = splitted[2]
            .chars()
            .next()
            .unwrap(); 
        Ok(PasswordRule { from, to, letter })
    }
}

pub fn map_input_to_password(s: &str) -> (PasswordRule, String) {
    let splitted: Vec<&str> = s
        .split(':')
        .collect();
    let password_rule: PasswordRule = splitted[0]
        .parse()
        .unwrap();
    (password_rule, splitted[1].trim().to_string())
}
