// Copyright 2019, Joren Van Onder (joren.vanonder@gmail.com)
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
extern crate clap;
use clap::{App, Arg};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

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

fn read_words(dictionary: &str) -> Result<HashSet<String>, io::Error> {
    let mut words = HashSet::new();
    let file = File::open(dictionary)?;

    for line in BufReader::new(file).lines() {
        words.insert(line?);
    }

    Ok(words)
}

fn main() {
    let layouts = create_layouts();
    let layout_names: Vec<&str> = layouts.keys().map(|n| n.as_str()).collect();
    let matches = App::new("Keyboard layout synonym finder")
        .version("1.0")
        .author("Joren Van Onder <joren.vanonder@gmail.com>")
        .arg(
            Arg::with_name("from")
                .short("f")
                .help("The configured keyboard layout")
                .takes_value(true)
                .possible_values(&layout_names)
                .required(true),
        )
        .arg(
            Arg::with_name("to")
                .short("t")
                .help("The keyboard layout in which is typed")
                .takes_value(true)
                .possible_values(&layout_names)
                .required(true),
        )
        .arg(
            Arg::with_name("dictionary")
                .short("d")
                .help("Path to dictionary to use")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let from = matches.value_of("from").unwrap();
    let to = matches.value_of("to").unwrap();

    let from = layouts.get(from).unwrap().keys.iter();
    let to = layouts.get(to).unwrap().keys.iter();
    let translator: HashMap<&char, &char> = from.zip(to).collect();

    let words = read_words(matches.value_of("dictionary").unwrap()).unwrap();

    for word in words.iter() {
        let mut invalid = false;
        let translated: String = word
            .chars()
            .map(|c| *translator.get(&c).unwrap_or_else(|| {
                invalid = true;
                &&' '
            }))
            .collect();

        if invalid {
            println!("{} was invalid", word);
            continue;
        }

        if words.contains(&translated) {
            println!("{} = {}", word, translated);
        }
    }
}
