#![allow(dead_code)]

use rust_gpiozero::*;
use std::fmt::{Formatter, Display, self};
use std::io::{stdin, Error};

// use std::thread::sleep;
// use std::time::Duration;
use std::time::Instant;

const MORSE_CODE_ALPHABET: [(char, &'static str); 55] = 
[
    ('a', ".-"),
    ('b', "-..."),
    ('c', "-.-."),
    ('d', "-.."),
    ('e', "."),
    ('f', "..-."),
    ('g', "--."),
    ('h', "...."),
    ('i', ".."),
    ('j', ".---"),
    ('k', "-.-"),
    ('l', ".-.."),
    ('m', "--"),
    ('n', "-."),
    ('o', "---"),
    ('p', ".--."),
    ('q', "--.-"),
    ('r', ".-."),
    ('s', "..."),
    ('t', "-"),
    ('u', "..-"),
    ('v', "...-"),
    ('w', ".--"),
    ('x', "-..-"),
    ('y', "-.--"),
    ('z', "--.."),
    ('1', ".----"),
    ('2', "..---"),
    ('3', "...--"),
    ('4', "....-"),
    ('5', "....."),
    ('6', "-...."),
    ('7', "--..."),
    ('8', "---.."),
    ('9', "----."),
    ('0', "-----"),
    ('.', ".-.-.-"),
    (',', "--..--"),
    ('?', "..--.."),
    ('\'', ".----."),
    ('!', "-.-.--"),
    ('/', "-..-."),
    ('(', "-.--."),
    (')', "-.--.-"),
    ('&', ".-..."),
    (':', "---..."),
    (';', "-.-.-."),
    ('=', "-...-"),
    ('+', ".-.-."),
    ('-', "-....-"),
    ('_', "..--.-"),
    ('"', ".-..-."),
    ('$', "...-..-"),
    ('@', ".--.-."),
    (' ', "/"),
];

#[derive(Clone, Copy)]
enum Morse {
    /// Character dot
    Dot,
    /// Character dash
    Dash,
    /// Character (letter) separator
    Space,
    /// Word (space) separator
    Slash,
    /// No-op
    None,
}

impl Morse {
    fn to_char(&self) -> char {
        match self {
            Morse::Dot => '.',
            Morse::Dash => '-',
            Morse::Space => ' ',
            Morse::Slash => '/',
            Morse::None => '|',
        }
    }
}

impl Display for Morse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Morse::Dot => write!(f, "."),
            &Morse::Dash => write!(f, "-"),
            &Morse::Space => write!(f, " "),
            &Morse::Slash => write!(f, "/"),
            &Morse::None => write!(f, "|"),
        }
    }
}

const DOT_LENGTH: u128 = 100;
const DASH_LENGTH: u128 = DOT_LENGTH * 3;
const DOT_DASH_DELAY: u128 = DOT_LENGTH;
const LETTER_DELAY: u128 = DOT_LENGTH * 3;
const WORD_DELAY: u128 = DOT_LENGTH * 7;
const DELAY_ERROR: u128 = DOT_LENGTH / 2;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    println!("Starting...\n- Type in text to turn into Morse Code\n- Type `quit()` to quit\n");
    println!(".... . .-.. .-.. --- / .-- --- .-. .-.. -..");
    let led = LED::new(17);
    let button = Button::new(2);

    let res = tokio::task::spawn_blocking(|| -> Result<(), Error> {
        let mut buffer = String::new();
        loop {
            let quit = stdin().read_line(&mut buffer)?;
            println!("Exiting...");
            if quit.to_string().contains("quit()") {
                return Ok(());
            }
        }
    });

    let mut time_active = Instant::now();
    let mut time_inactive = Instant::now();

    // String of Morse Code received so far
    let mut morse_paragraph = String::new();
    loop {
        if button.is_active() {
            if !led.is_active() {
                led.on();
                let morse = translate_inactive_into_morse(time_inactive);
                append_morse_to_paragraph(&mut morse_paragraph, morse);
                let text = morse_string_to_text(&morse_paragraph);
                write_to_terminal(&morse_paragraph, &text);
                time_active = Instant::now();
            }
        } else {
            if led.is_active() {
                led.off();
                let morse = translate_active_into_morse(time_active);
                append_morse_to_paragraph(&mut morse_paragraph, morse);
                let text = morse_string_to_text(&morse_paragraph);
                write_to_terminal(&morse_paragraph, &text);
                time_inactive = Instant::now();
            }
        }
        if res.is_finished() {
            println!("Done");
            led.close();
            return Ok(());
        }
    }
}

fn translate_inactive_into_morse(time: Instant) -> Morse {
    let time_inactive = time.elapsed().as_millis();
    if time_inactive > WORD_DELAY - DELAY_ERROR {
        Morse::Slash
    } else if time_inactive > LETTER_DELAY - DELAY_ERROR {
        Morse::Space
    } else {
        Morse::None
    }
}

fn translate_active_into_morse(time: Instant) -> Morse {
    let time_active = time.elapsed().as_millis();
    if time_active > DASH_LENGTH - DELAY_ERROR {
        Morse::Dash
    } else if time_active > DOT_LENGTH - DELAY_ERROR {
        Morse::Dot
    } else {
        Morse::None
    }
}

fn append_morse_to_paragraph(morse_paragraph: &mut String, morse: Morse) {
    match morse {
        Morse::None => (),
        Morse::Slash => morse_paragraph.push_str(" / "),
        _ => {
            morse_paragraph.push(morse.to_char());
        }
    }
}

/// Returns the translation for the given Morse Code
/// 
/// Example Morse Code String:
/// ```markdown
/// .... . .-.. .-.. --- / .-- --- .-. .-.. -..
/// ```
fn morse_string_to_text(morse_string: &str) -> String {
    let characters = morse_string.split_whitespace();
    let mut text = String::new();
    for character in characters {
        if let Some((letter, _)) = MORSE_CODE_ALPHABET.iter().find(|(_, c)| c == &character) {
            text.push(*letter);
        }
    }
    text
}



/// Writes the given string to the terminal, without newlines
/// Does not append the string, but overwrites the previous string
fn write_to_terminal(morse_paragraph: &str, text_paragraph: &str) {
    std::process::Command::new("clear").status().unwrap();
    println!("{}", morse_paragraph);
    println!("{}", text_paragraph);
}
