extern crate clap;
use clap::{App, Arg};
use std::collections::HashMap;

struct KbdLayout {
    name: String,
    keys: &'static [char],
}

fn create_layouts() -> HashMap<String, KbdLayout> {
    let mut layouts = HashMap::new();
    let dvorak = KbdLayout {
        name: "dvorak".to_string(),
        keys: &[
            '\'', ',', '.', 'p', 'y', 'f', 'g', 'c', 'r', 'l', 'a', 'o', 'e', 'u', 'i', 'd', 'h',
            't', 'n', 's', ';', 'q', 'j', 'k', 'x', 'b', 'm', 'w', 'v', 'z',
        ],
    };
    let qwerty = KbdLayout {
        name: "qwerty".to_string(),
        keys: &[
            'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j',
            'k', 'l', ';', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/',
        ],
    };

    layouts.insert(dvorak.name.clone(), dvorak);
    layouts.insert(qwerty.name.clone(), qwerty);
    layouts
}

#[cfg(test)]
mod test_layouts {
    use super::*;

    #[test]
    fn test_layout_length() {
        let layouts = create_layouts();

        for layout in layouts.values() {
            assert_eq!(layout.keys.len(), 30);
        }
    }
}

fn main() {
    let matches = App::new("Keyboard layout synonym finder")
        .version("1.0")
        .author("Joren Van Onder <joren.vanonder@gmail.com>")
        .arg(
            Arg::with_name("from")
                .short("f")
                .help("The configured keyboard layout")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("to")
                .short("t")
                .help("The keyboard layout in which is typed")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let from = matches.value_of("from");
    let layouts = create_layouts();
}
