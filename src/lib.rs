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
    for key in split_keyss(input)? {
        keys.push(parse_key(&key)?);
    }
    Ok(Keys(keys))
}

pub fn parse_key(input: &str) -> Result<Key, Error> {
    let mut modifier_strings = input.split('-');
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

fn split_keyss(input: &str) -> Result<Vec<String>, Error> {
    let mut key_strings: Vec<String> = Vec::new();
    let mut escaped = false;
    let mut key_string_current: Option<String> = None;
    for ch in input.chars() {
        if escaped {
            escaped = false;
            key_strings.push(ch.to_string());
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '<' => {
                if key_string_current.is_some() {
                    return Err(Error::UnexpectedGroupOpen);
                }
                key_string_current = Some(String::new());
            }
            '>' => {
                if let Some(current) = key_string_current {
                    if !current.contains('-') {
                        return Err(Error::IncompleteGroup(current));
                    }
                    key_strings.push(current);
                    key_string_current = None;
                } else {
                    return Err(Error::UnexpectedGroupClose);
                }
            }
            _ => {
                if let Some(current) = &mut key_string_current {
                    current.push(ch);
                } else {
                    key_strings.push(ch.to_string())
                }
            }
        }
    }
    if key_string_current.is_some() {
        return Err(Error::UnexpectedEnd);
    }

    Ok(key_strings)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            Err(Error::InvalidKeyName("".to_string()))
        );
        assert_eq!(
            parse_keys("<C-->"),
            Err(Error::InvalidKeyName("".to_string()))
        );
        assert_eq!(
            parse_keys("<-a>"),
            Err(Error::InvalidKeyModifier("".to_string()))
        );
        assert_eq!(
            parse_keys("<--a>"),
            Err(Error::InvalidKeyModifier("".to_string()))
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
            parse_key("C-a"),
            Ok(Key {
                name: KeyName::A,
                modifiers: Modifiers {
                    control: true,
                    ..Default::default()
                },
            })
        );
        assert_eq!(
            parse_key("C-B"),
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
            parse_key("M-a"),
            Ok(Key {
                name: KeyName::A,
                modifiers: Modifiers {
                    alt: true,
                    ..Default::default()
                },
            })
        );
        assert_eq!(
            parse_key("M-A"),
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
            parse_key("C-M-b"),
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
            parse_key("M-C-a"),
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
            parse_key("C-M-A"),
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
            parse_key("C-!"),
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
        // assert_eq!(
        //     parse_key("\\-"),
        //     Ok(Key {
        //         name: KeyName::Dash,
        //         modifiers: Modifiers::default(),
        //     })
        // );

        assert_eq!(parse_key("<"), Err(Error::InvalidKeyName("<".to_string())));
        assert_eq!(parse_key(">"), Err(Error::InvalidKeyName(">".to_string())));
        assert_eq!(parse_key("-"), Err(Error::InvalidKeyName("".to_string())));

        assert_eq!(parse_key("C-"), Err(Error::InvalidKeyName("".to_string())));
        assert_eq!(parse_key("C--"), Err(Error::InvalidKeyName("".to_string())));
        assert_eq!(
            parse_key("-a"),
            Err(Error::InvalidKeyModifier("".to_string()))
        );
        assert_eq!(
            parse_key("--a"),
            Err(Error::InvalidKeyModifier("".to_string()))
        );
    }
}
