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
    assert_eq!(split_keys("\\>"), Ok(vec!["\\>"]));
    assert_eq!(split_keys("\\<"), Ok(vec!["\\<"]));
    assert_eq!(split_keys("a\\<b"), Ok(vec!["a", "\\<", "b"]));

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
    assert_eq!(
        parse_key("\\<"),
        Ok(Key {
            name: KeyName::LessThan,
            modifiers: Modifiers::default(),
        })
    );
    assert_eq!(
        parse_key("<C-\\<>"),
        Ok(Key {
            name: KeyName::LessThan,
            modifiers: Modifiers {
                control: true,
                ..Default::default()
            }
        })
    );
    assert_eq!(
        parse_key("\\>"),
        Ok(Key {
            name: KeyName::GreaterThan,
            modifiers: Modifiers::default(),
        })
    );
    assert_eq!(
        parse_key("<C-\\>>"),
        Ok(Key {
            name: KeyName::GreaterThan,
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
