use crate::env_file::parser::env_file;
use combine::error::StringStreamError;
use combine::Parser;
use std::collections::HashMap;
use std::str::FromStr;

mod parser;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EnvFile(Vec<EnvFileRow>);

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EnvFileRow {
    Empty,
    CommentOnly(String),
    Declaration(EnvDeclaration),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EnvDeclaration {
    pub name: String,
    pub value: Option<String>,
}

impl EnvFile {
    pub fn env(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for (name, value) in self
            .0
            .iter()
            .flat_map(|row| match row {
                EnvFileRow::Declaration(declaration) => Some(declaration),
                _ => None,
            })
            .flat_map(|declaration| match declaration {
                EnvDeclaration {
                    name,
                    value: Some(value),
                } => Some((name, value)),
                _ => None,
            })
        {
            map.insert(name.clone(), value.clone());
        }
        map
    }

    pub fn stream(&self) -> Vec<EnvFileRow> {
        self.0.clone()
    }

    pub fn apply_assign(&mut self, map: &HashMap<String, String>, r#override: bool) -> () {
        let mut map = map.clone();
        for row in self.0.iter_mut() {
            match row {
                EnvFileRow::Declaration(declaration) => {
                    if map.contains_key(declaration.name.as_str()) {
                        if r#override || declaration.value.is_none() {
                            declaration.value = map.remove(declaration.name.as_str().clone());
                        } else {
                            map.remove(declaration.name.as_str());
                        }
                    }
                }
                _ => (),
            }
        }
        if !map.is_empty() {
            self.0.push(EnvFileRow::Empty);
            self.0
                .push(EnvFileRow::CommentOnly("Additional values".to_string()));
            for (k, v) in map {
                self.0.push(EnvFileRow::Declaration(EnvDeclaration {
                    name: k,
                    value: Some(v),
                }))
            }
        }
    }

    #[inline]
    pub fn apply(&self, map: &HashMap<String, String>, r#override: bool) -> Self {
        let mut clone = self.clone();
        clone.apply_assign(map, r#override);
        clone
    }
}

impl FromStr for EnvFile {
    type Err = StringStreamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = env_file().parse(s).map(|(result, _)| result)?;
        while !result.0.is_empty() && result.0.last() == Some(&EnvFileRow::Empty) {
            result.0.pop();
        }
        Ok(result)
    }
}

impl ToString for EnvFile {
    fn to_string(&self) -> String {
        let mut result: String = self
            .0
            .iter()
            .map(|row| match row {
                EnvFileRow::Empty => "".to_string(),
                EnvFileRow::CommentOnly(comment) => format!("# {}", comment),
                EnvFileRow::Declaration(declaration) => match declaration {
                    EnvDeclaration {
                        name,
                        value: Some(default),
                    } => format!("{}={}", name, default),
                    EnvDeclaration {
                        name,
                        value: _,
                    } => format!("{}=", name),
                },
            })
            .collect::<Vec<_>>()
            .join("\n");
        while result.chars().last() == Some('\n') {
            result.pop();
        }
        if !result.is_empty() {
            result += "\n"
        }
        result
    }
}

#[test]
fn test_env_file_serialization() {
    let input = EnvFile(vec![
        EnvFileRow::CommentOnly("An example .env file for test".to_string()),
        EnvFileRow::Empty,
        EnvFileRow::Declaration(EnvDeclaration {
            name: "ABC".to_string(),
            value: None,
        }),
        EnvFileRow::Declaration(EnvDeclaration {
            name: "ABC".to_string(),
            value: Some("123".to_string()),
        }),
        EnvFileRow::Declaration(EnvDeclaration {
            name: "ABC".to_string(),
            value: Some("123".to_string()),
        }),
        EnvFileRow::Declaration(EnvDeclaration {
            name: "ABC".to_string(),
            value: None,
        }),
    ]);

    println!("{:?}", input.to_string());
    assert_eq!(EnvFile::from_str(input.to_string().as_str()), Ok(input));
}

#[test]
fn test_env_file_hash() {
    let input = EnvFile(vec![
        EnvFileRow::CommentOnly("An example .env file for test".to_string()),
        EnvFileRow::Empty,
        EnvFileRow::Declaration(EnvDeclaration {
            name: "ABC".to_string(),
            value: None,
        }),
        EnvFileRow::Declaration(EnvDeclaration {
            name: "BCD".to_string(),
            value: Some("123".to_string()),
        }),
        EnvFileRow::Declaration(EnvDeclaration {
            name: "CDE".to_string(),
            value: Some("123".to_string()),
        }),
        EnvFileRow::Declaration(EnvDeclaration {
            name: "DEF".to_string(),
            value: None,
        }),
    ]);
    let output = [("BCD", "123"), ("CDE", "123")]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<String, String>>();

    assert_eq!(input.env(), output);
}

#[test]
fn test_env_file_apply() {
    let mut input = EnvFile(vec![
        EnvFileRow::CommentOnly("An example .env file for test".to_string()),
        EnvFileRow::Empty,
        EnvFileRow::Declaration(EnvDeclaration {
            name: "ABC".to_string(),
            value: None,
        }),
        EnvFileRow::Declaration(EnvDeclaration {
            name: "BCD".to_string(),
            value: Some("123".to_string()),
        }),
        EnvFileRow::Declaration(EnvDeclaration {
            name: "CDE".to_string(),
            value: Some("123".to_string()),
        }),
        EnvFileRow::Declaration(EnvDeclaration {
            name: "DEF".to_string(),
            value: None,
        }),
    ]);
    let map = [
        ("ABC", "123"),
        ("CDE", "456"),
        ("DEF", "123"),
        ("EFG", "789"),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect::<HashMap<String, String>>();
    let output_1 = r#"# An example .env file for test

ABC=123
BCD=123
CDE=456
DEF=123

# Additional values
EFG=789
"#;
    let output_2 = r#"# An example .env file for test

ABC=123
BCD=123
CDE=123
DEF=123

# Additional values
EFG=789
"#;

    assert_eq!(input.apply(&map, true).to_string(), output_1.to_string());
    input.apply_assign(&map, false);
    assert_eq!(input.to_string(), output_2.to_string());
}
