#[derive(Clone, Debug, PartialEq)]
pub struct Keys(Vec<Key>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Key {
    pub modifiers: Modifiers,
    pub name: KeyName,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Modifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
}

macro_rules! define_key_name {
    ( $(
        $ident:ident $lower:literal $( $upper:literal )?
    ),* $(,)? ) =>{
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum KeyName {
            $( $ident ),*
        }

        impl KeyName {
            fn from_str(value: &str) -> Option<(Self, bool)> {
                Some(match value {
                    $(
                        $lower => (KeyName::$ident, false),
                        $(
                            $upper => (KeyName::$ident, true),
                        )?
                    )*
                    _ => return None,
                })
            }
        }
    };
}

define_key_name!(
    A "a" "A",
    B "b" "B",
    C "c" "C",
    D "d" "D",
    E "e" "E",
    F "f" "F",
    G "g" "G",
    H "h" "H",
    I "i" "I",
    J "j" "J",
    K "k" "K",
    L "l" "L",
    M "m" "M",
    N "n" "N",
    O "o" "O",
    P "p" "P",
    Q "q" "Q",
    R "r" "R",
    S "s" "S",
    T "t" "T",
    U "u" "U",
    V "v" "V",
    W "w" "W",
    X "x" "X",
    Y "y" "Y",
    Z "z" "Z",
    Number0 "0",
    Number1 "1",
    Number2 "2",
    Number3 "3",
    Number4 "4",
    Number5 "5",
    Number6 "6",
    Number7 "7",
    Number8 "8",
    Number9 "9",
    Bang "!",
    At "@",
    Pound "#",
    Dollar "$",
    Percent "%",
    Carrot "^",
    Ampersand "&",
    Star "*",
    ParenthesisLeft "(",
    ParenthesisRight ")",
    BracketLeft "[",
    BracketRight "]",
    BraceLeft "{",
    BraceRight "}",
    Backtick "`",
    Tilde "~",
    Equals "=",
    Underscore "_",
    Plus "+",
    ForwardSlash "/",
    Backslash "\\",
    Question "?",
    Pipe "|",
    SingleQuote "'",
    DoubleQuote "\"",
    Comma ",",
    Period ".",
    Colon ":",
    Semicolon ";",
    Dash "\\-",
    LessThan "\\<",
    GreaterThan "\\>",
);

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Missing key name")]
    NoKeyName,
    #[error("Invalid key name `{0}`")]
    InvalidKeyName(String),
    #[error("Invalid key modifier `{0}`")]
    InvalidKeyModifier(String),
    #[error("Unexpected open of modifier group (`<`)")]
    UnexpectedGroupOpen,
    #[error("Unexpected close of modifier group (`>`)")]
    UnexpectedGroupClose,
    #[error("Unclosed modifier group (missing `>` at end)")]
    UnexpectedEnd,
    #[error("Modifier group must be include modifer and key name, not `{0}`")]
    IncompleteGroup(String),
}

pub fn parse_keys(input: &str) -> Result<Keys, Error> {
    let mut keys = Vec::new();
    for key in split_keys(input)? {
        keys.push(parse_key(&key)?);
    }
    Ok(Keys(keys))
}

pub fn parse_key(input: &str) -> Result<Key, Error> {
    if input.starts_with('<') && input.ends_with('>') {
        let mut chars = input.chars();
        chars.next();
        chars.next_back();
        parse_key_with_modifier(chars.as_str())
    } else {
        parse_key_no_modifier(input)
    }
}

fn parse_key_no_modifier(input: &str) -> Result<Key, Error> {
    let Some((name, shift)) = KeyName::from_str(input) else {
        return Err(Error::InvalidKeyName(input.to_string()));
    };

    let modifiers = Modifiers {
        shift,
        ..Default::default()
    };

    Ok(Key { modifiers, name })
}

fn parse_key_with_modifier(input: &str) -> Result<Key, Error> {
    let modifier_strings = split_modifiers(input)?;
    if modifier_strings.len() < 2 {
        return Err(Error::IncompleteGroup(input.to_string()));
    }
    let mut modifier_strings = modifier_strings.into_iter();

    let Some(name) = modifier_strings.next_back() else {
        return Err(Error::NoKeyName);
    };

    let Some((name, shift)) = KeyName::from_str(name) else {
        return Err(Error::InvalidKeyName(name.to_string()));
    };

    let mut control = false;
    let mut alt = false;

    for modifier in modifier_strings {
        match modifier {
            "C" => control = true,
            "M" => alt = true,
            _ => return Err(Error::InvalidKeyModifier(modifier.to_string())),
        };
    }

    let modifiers = Modifiers {
        shift,
        control,
        alt,
    };

    Ok(Key { modifiers, name })
}

fn split_modifiers(input: &str) -> Result<Vec<&str>, Error> {
    let mut keys: Vec<&str> = Vec::new();
    let mut start = 0;
    let mut is_escaped = false;

    for (i, ch) in input.char_indices() {
        if is_escaped {
            is_escaped = false;
            continue;
        }
        if ch == '\\' {
            is_escaped = true;
        } else if ch == '-' {
            if start != i {
                keys.push(&input[start..i]);
            }
            start = i + 1;
        }
    }

    if start < input.len() {
        if is_escaped {
            panic!("cannot escape end of group");
        }
        keys.push(&input[start..]);
    }

    Ok(keys)
}

fn split_keys(input: &str) -> Result<Vec<&str>, Error> {
    let mut keys: Vec<&str> = Vec::new();
    let mut start = 0;
    let mut is_group = false;

    for (mut i, ch) in input.char_indices() {
        match (is_group, ch) {
            // Mismatched group delimeters
            (true, '<') => return Err(Error::UnexpectedGroupOpen),
            (false, '>') => return Err(Error::UnexpectedGroupClose),

            // Open group
            (_, '<') => is_group = true,
            // Close group
            (_, '>') => {
                is_group = false;
                i += 1;
            }

            // Any character inside group - do not push key yet
            (true, _) => continue,

            _ => (),
        }

        // Push key based on index
        if start != i {
            keys.push(&input[start..i]);
            start = i;
        }
    }

    // Push last key
    if start < input.len() {
        // Missing closing delimeter
        if is_group {
            return Err(Error::UnexpectedEnd);
        }
        keys.push(&input[start..]);
    }

    Ok(keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_keys_works() {
        assert_eq!(split_keys(""), Ok(vec![]));
        assert_eq!(split_keys("a"), Ok(vec!["a"]));
        assert_eq!(split_keys("ab"), Ok(vec!["a", "b"]));
        assert_eq!(split_keys("<C-a>"), Ok(vec!["<C-a>"]));
        assert_eq!(split_keys("<C-a>b"), Ok(vec!["<C-a>", "b"]));
        assert_eq!(split_keys("b<C-a>"), Ok(vec!["b", "<C-a>"]));
        assert_eq!(split_keys("<C-a><C-b>"), Ok(vec!["<C-a>", "<C-b>"]));

        assert_eq!(split_keys("<a"), Err(Error::UnexpectedEnd));
        assert_eq!(split_keys("a<C-a><"), Err(Error::UnexpectedEnd));
        assert_eq!(split_keys("<C-<a>"), Err(Error::UnexpectedGroupOpen));
        assert_eq!(split_keys("C-<<a>"), Err(Error::UnexpectedGroupOpen));
        assert_eq!(split_keys("a>"), Err(Error::UnexpectedGroupClose));
        assert_eq!(split_keys("<C-a>>"), Err(Error::UnexpectedGroupClose));
    }

    #[test]
    fn split_modifiers_works() {
        assert_eq!(split_modifiers("a"), Ok(vec!["a"]));
        assert_eq!(split_modifiers("ab"), Ok(vec!["ab"]));
        assert_eq!(split_modifiers("C-a"), Ok(vec!["C", "a"]));
        assert_eq!(split_modifiers("C-M-a"), Ok(vec!["C", "M", "a"]));
        assert_eq!(split_modifiers("\\a"), Ok(vec!["\\a"]));
        assert_eq!(split_modifiers("\\-"), Ok(vec!["\\-"]));
        assert_eq!(split_modifiers("C-\\-"), Ok(vec!["C", "\\-"]));
        assert_eq!(split_modifiers("--"), Ok(vec![]));
        assert_eq!(split_modifiers("-a"), Ok(vec!["a"]));
        assert_eq!(split_modifiers("--a"), Ok(vec!["a"]));
        assert_eq!(split_modifiers("C--"), Ok(vec!["C"]));

        // assert_eq!(split_modifiers("C-\\"), Err(Error::));
    }

    #[test]
    fn parse_keys_works() {
        assert_eq!(
            parse_keys("a"),
            Ok(Keys(vec![Key {
                name: KeyName::A,
                modifiers: Modifiers::default(),
            }])),
        );
        assert_eq!(
            parse_keys("aa"),
            Ok(Keys(vec![
                Key {
                    name: KeyName::A,
                    modifiers: Modifiers::default(),
                },
                Key {
                    name: KeyName::A,
                    modifiers: Modifiers::default(),
                },
            ])),
        );
        assert_eq!(
            parse_keys("ab"),
            Ok(Keys(vec![
                Key {
                    name: KeyName::A,
                    modifiers: Modifiers::default(),
                },
                Key {
                    name: KeyName::B,
                    modifiers: Modifiers::default(),
                },
            ])),
        );
        assert_eq!(
            parse_keys("<C-a>"),
            Ok(Keys(vec![Key {
                name: KeyName::A,
                modifiers: Modifiers {
                    control: true,
                    ..Default::default()
                },
            }])),
        );
        assert_eq!(
            parse_keys("a<C-a>b"),
            Ok(Keys(vec![
                Key {
                    name: KeyName::A,
                    modifiers: Modifiers::default(),
                },
                Key {
                    name: KeyName::A,
                    modifiers: Modifiers {
                        control: true,
                        ..Default::default()
                    },
                },
                Key {
                    name: KeyName::B,
                    modifiers: Modifiers::default(),
                },
            ])),
        );
        assert_eq!(
            parse_keys("<M-a><C-a>B<M-C-B>"),
            Ok(Keys(vec![
                Key {
                    name: KeyName::A,
                    modifiers: Modifiers {
                        alt: true,
                        ..Default::default()
                    },
                },
                Key {
                    name: KeyName::A,
                    modifiers: Modifiers {
                        control: true,
                        ..Default::default()
                    },
                },
                Key {
                    name: KeyName::B,
                    modifiers: Modifiers {
                        shift: true,
                        ..Default::default()
                    },
                },
                Key {
                    name: KeyName::B,
                    modifiers: Modifiers {
                        shift: true,
                        control: true,
                        alt: true,
                        ..Default::default()
                    },
                },
            ])),
        );

        assert_eq!(parse_keys("<C-<a>>"), Err(Error::UnexpectedGroupOpen));
        assert_eq!(parse_keys("a>b"), Err(Error::UnexpectedGroupClose));
        assert_eq!(parse_keys("<C-"), Err(Error::UnexpectedEnd));
        assert_eq!(
            parse_keys("<C>"),
            Err(Error::IncompleteGroup("C".to_string()))
        );
        assert_eq!(
            parse_keys("<Ca>"),
            Err(Error::IncompleteGroup("Ca".to_string()))
        );

        assert_eq!(
            parse_keys("<C->"),
            Err(Error::IncompleteGroup("C-".to_string()))
        );
        assert_eq!(
            parse_keys("<C-->"),
            Err(Error::IncompleteGroup("C--".to_string()))
        );
        assert_eq!(
            parse_keys("<-a>"),
            Err(Error::IncompleteGroup("-a".to_string()))
        );
        assert_eq!(
            parse_keys("<--a>"),
            Err(Error::IncompleteGroup("--a".to_string()))
        );
    }

    #[test]
    fn parse_key_works() {
        assert_eq!(
            parse_key("a"),
            Ok(Key {
                name: KeyName::A,
                modifiers: Modifiers::default(),
            })
        );
        assert_eq!(
            parse_key("b"),
            Ok(Key {
                name: KeyName::B,
                modifiers: Modifiers::default(),
            })
        );
        assert_eq!(
            parse_key("A"),
            Ok(Key {
                name: KeyName::A,
                modifiers: Modifiers {
                    shift: true,
                    ..Default::default()
                },
            })
        );
        assert_eq!(
            parse_key("<C-a>"),
            Ok(Key {
                name: KeyName::A,
                modifiers: Modifiers {
                    control: true,
                    ..Default::default()
                },
            })
        );
        assert_eq!(
            parse_key("<C-B>"),
            Ok(Key {
                name: KeyName::B,
                modifiers: Modifiers {
                    shift: true,
                    control: true,
                    ..Default::default()
                },
            })
        );
        assert_eq!(
            parse_key("<M-a>"),
            Ok(Key {
                name: KeyName::A,
                modifiers: Modifiers {
                    alt: true,
                    ..Default::default()
                },
            })
        );
        assert_eq!(
            parse_key("<M-A>"),
            Ok(Key {
                name: KeyName::A,
                modifiers: Modifiers {
                    shift: true,
                    alt: true,
                    ..Default::default()
                },
            })
        );
        assert_eq!(
            parse_key("<C-M-b>"),
            Ok(Key {
                name: KeyName::B,
                modifiers: Modifiers {
                    control: true,
                    alt: true,
                    ..Default::default()
                },
            })
        );
        assert_eq!(
            parse_key("<M-C-a>"),
            Ok(Key {
                name: KeyName::A,
                modifiers: Modifiers {
                    control: true,
                    alt: true,
                    ..Default::default()
                },
            })
        );
        assert_eq!(
            parse_key("<C-M-A>"),
            Ok(Key {
                name: KeyName::A,
                modifiers: Modifiers {
                    shift: true,
                    control: true,
                    alt: true,
                    ..Default::default()
                },
            })
        );

        assert_eq!(
            parse_key("!"),
            Ok(Key {
                name: KeyName::Bang,
                modifiers: Modifiers::default(),
            })
        );
        assert_eq!(
            parse_key("<C-!>"),
            Ok(Key {
                name: KeyName::Bang,
                modifiers: Modifiers {
                    control: true,
                    ..Default::default()
                },
            })
        );

        assert_eq!(
            parse_key("\\<"),
            Ok(Key {
                name: KeyName::LessThan,
                modifiers: Modifiers::default(),
            })
        );
        assert_eq!(
            parse_key("\\-"),
            Ok(Key {
                name: KeyName::Dash,
                modifiers: Modifiers::default(),
            })
        );
        assert_eq!(
            parse_key("<C-\\->"),
            Ok(Key {
                name: KeyName::Dash,
                modifiers: Modifiers {
                    control: true,
                    ..Default::default()
                }
            })
        );

        assert_eq!(parse_key("<"), Err(Error::InvalidKeyName("<".to_string())));
        assert_eq!(parse_key(">"), Err(Error::InvalidKeyName(">".to_string())));
        assert_eq!(parse_key("-"), Err(Error::InvalidKeyName("-".to_string())));

        assert_eq!(
            parse_key("C-"),
            Err(Error::InvalidKeyName("C-".to_string()))
        );
        assert_eq!(
            parse_key("C--"),
            Err(Error::InvalidKeyName("C--".to_string()))
        );
        assert_eq!(
            parse_key("-a"),
            Err(Error::InvalidKeyName("-a".to_string()))
        );
        assert_eq!(
            parse_key("--a"),
            Err(Error::InvalidKeyName("--a".to_string()))
        );

        assert_eq!(
            parse_key("<C->"),
            Err(Error::IncompleteGroup("C-".to_string()))
        );
        assert_eq!(
            parse_key("<C-->"),
            Err(Error::IncompleteGroup("C--".to_string()))
        );
        assert_eq!(
            parse_key("<-a>"),
            Err(Error::IncompleteGroup("-a".to_string()))
        );
        assert_eq!(
            parse_key("<--a>"),
            Err(Error::IncompleteGroup("--a".to_string()))
        );
    }
}
