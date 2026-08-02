use std::{
    char,
    io::{self, BufRead},
    iter::{FromIterator, Peekable},
    str::CharIndices,
};

// TODO (json-tokenize): implement per the lesson description.

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }

        let mut t = Tokinzer::new(&l);
        t.tokenize();
    }
}

pub struct Tokinzer<'a> {
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Tokinzer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokinzer {
            chars: input.char_indices().peekable(),
        }
    }

    fn peek(&mut self) -> Option<(usize, char)> {
        self.chars.peek().copied()
    }

    fn next(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }

    fn skip_whitespace(&mut self) {
        while let Some((_, ch)) = self.peek() {
            if ch.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    pub fn tokenize(&mut self) {
        while let Some((idx, ch)) = self.peek() {
            match ch {
                '{' | '}' | '[' | ']' | ':' | ',' => {
                    println!("PUNCT {}", ch);
                    self.next();
                }
                '"' => {
                    match self.read_string() {
                        Ok(string) => { println!("{}", string); },
                        Err(_) => { 
                            println!("ERR");
                            return;
                        }
                    }
                    
                }
                ch if ch.is_ascii_digit() || ch == '-' => {
                    println!("NUMBER {}", self.read_number());
                }
                ch if ch.is_whitespace() => {
                    self.skip_whitespace();
                }
                't' | 'f' | 'n' => {
                    let value = self.read_value();
                    match value.as_str() {
                        "true" => {
                            println!("TRUE {}", value);
                        }
                        "false" => {
                            println!("FALSE {}", value);
                        }
                        "null" => {
                            println!("NULL {}", value);
                        }
                        _ => {
                            println!("ERR invalid token {}", value);
                        }
                    }
                }
                _ => {
                    println!("ERR unexpected character '{}' at position {}", ch, idx);
                    return;
                }
            }
        }
    }

    fn read_string(&mut self) -> Result<String, TokinzerError> {
        self.next();

        let mut result = String::new();
        let mut closing_quote_seen = false;

        while let Some((idx, ch)) = self.next() {
            if ch == '\\' {
                match self.next().unwrap().1 {
                    'b' => result.push(char::from(0x08)),
                    'f' => result.push(char::from(0x0C)),
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '"' => result.push('"'),
                    '\\' => result.push('\\'),
                    '/' => result.push('/'),
                    'u' => {
                        let x = self.get_unicode_unit();
                    }

                    _ => {
                        return Err(TokinzerError::InvalidToken(ch, idx))
                    },
                }
            } else if ch == '"' {
                closing_quote_seen = true;
                break;
            } else {
                result.push(ch);
            }
        }

        if closing_quote_seen {
        Ok(result)
        }
        else {
            Err(TokinzerError::StringNotTerminated)
        }
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();

        while let Some((_, ch)) = self.peek() {
            if ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == '+' || ch == 'e' || ch == 'E'
            {
                number.push(self.next().unwrap().1);
            } else {
                break;
            }
        }

        number
    }

    fn read_value(&mut self) -> String {
        let mut value = String::new();

        while let Some((_, ch)) = self.peek() {
            if ch.is_ascii_alphabetic() {
                value.push(self.next().unwrap().1);
            } else {
                break;
            }
        }

        value
    }

    fn get_unicode_unit(&mut self) -> u16 {
        let mut unicode = self.chars.by_ref().take(4).map(|(_, ch)| ch);

        if unicode.all(|ch| ch.is_ascii_hexdigit()) {
            return String::from_iter(unicode).parse::<u16>().expect("unable to parse u16");
        }
        
        0
    }
}

enum TokinzerError {
    InvalidToken(char, usize),
    StringNotTerminated,
}