#[cfg(feature = "ggez")]
mod ggez;
#[cfg(test)]
mod tests;

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
        $ident:ident,
        $($lower:literal)?,
        $($upper:literal)?,
        $($ggez:ident)?
    );* $(;)? ) =>{
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum KeyName {
            $( $ident ),*
        }

        impl KeyName {
            pub fn from_str(value: &str) -> Option<(Self, bool)> {
                Some(match value {
                    $(
                        $( $lower => (KeyName::$ident, false), )?
                        $( $upper => (KeyName::$ident, true), )?
                    )*
                    _ => return None,
                })
            }
        }

        #[cfg(feature = "ggez")]
        mod ggez_key_name {
            use ggez::input::keyboard::KeyCode;
            use super::KeyName;
            impl TryFrom<KeyCode> for KeyName {
                type Error = ();
                fn try_from(keycode: KeyCode) -> Result<Self, Self::Error> {
                    Ok(match keycode {
                        $(
                            $( KeyCode::$ggez => KeyName::$ident, )?
                        )*
                        _ => return Err(()),
                    })
                }
            }
        }
    };
}

define_key_name!(
    A,            "a", "A", A;
    B,            "b", "B", B;
    C,            "c", "C", C;
    D,            "d", "D", D;
    E,            "e", "E", E;
    F,            "f", "F", F;
    G,            "g", "G", G;
    H,            "h", "H", H;
    I,            "i", "I", I;
    J,            "j", "J", J;
    K,            "k", "K", K;
    L,            "l", "L", L;
    M,            "m", "M", M;
    N,            "n", "N", N;
    O,            "o", "O", O;
    P,            "p", "P", P;
    Q,            "q", "Q", Q;
    R,            "r", "R", R;
    S,            "s", "S", S;
    T,            "t", "T", T;
    U,            "u", "U", U;
    V,            "v", "V", V;
    W,            "w", "W", W;
    X,            "x", "X", X;
    Y,            "y", "Y", Y;
    Z,            "z", "Z", Z;
    Number0,      "0",    , Key0;
    Number1,      "1",    , Key1;
    Number2,      "2",    , Key2;
    Number3,      "3",    , Key3;
    Number4,      "4",    , Key4;
    Number5,      "5",    , Key5;
    Number6,      "6",    , Key6;
    Number7,      "7",    , Key7;
    Number8,      "8",    , Key8;
    Number9,      "9",    , Key9;
    Bang,         "!",    , ;
    At,           "@",    , ;
    Pound,        "#",    , ;
    Dollar,       "$",    , ;
    Percent,      "%",    , ;
    Carrot,       "^",    , ;
    Ampersand,    "&",    , ;
    Star,         "*",    , ;
    ParenLeft,    "(",    , ;
    ParenRight,   ")",    , ;
    BracketLeft,  "[",    , ;
    BracketRight, "]",    , ;
    BraceLeft,    "{",    , ;
    BraceRight,   "}",    , ;
    Backtick,     "`",    , ;
    Tilde,        "~",    , ;
    Equals,       "=",    , ;
    Underscore,   "_",    , ;
    Plus,         "+",    , ;
    ForwardSlash, "/",    , ;
    Backslash,    "\\",   , ;
    Question,     "?",    , ;
    Pipe,         "|",    , ;
    SingleQuote,  "'",    , ;
    DoubleQuote,  "\"",   , ;
    Comma,        ",",    , ;
    Period,       ".",    , ;
    Colon,        ":",    , ;
    Semicolon,    ";",    , ;
    Dash,         "\\-",  , ;
    LessThan,     "\\<",  , ;
    GreaterThan,  "\\>",  , ;
    Space,        ,       , Space;
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
    let mut is_escaped = false;
    let mut is_group = false;

    for (mut i, ch) in input.char_indices() {
        if is_escaped {
            is_escaped = false;
            continue;
        }
        if ch == '\\' {
            is_escaped = true;
        }
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
