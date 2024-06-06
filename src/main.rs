use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read as _, Write as _},
};

use quick_xml::events::Event;
use walkdir::WalkDir;
//6b925f47fe5958318cf63084b4bb8331

/*
scraper-rs panic:

2024-06-05 20:52:47.724971 -04:00: [INFO] Downloaded: https://en.wikipedia.org/wiki/List_of_conflicts_related_to_the_Cold_War
2024-06-05 20:52:47.769910 -04:00: [INFO] Downloaded: https://en.wikipedia.org/wiki/Post-Soviet_conflicts
2024-06-05 20:52:47.814384 -04:00: [INFO] Downloaded: https://en.wikipedia.org/wiki/Reagan_Doctrine
2024-06-05 20:52:47.868346 -04:00: [INFO] Downloaded: https://en.wikipedia.org/wiki/Second_Cold_War
thread '<unnamed>' panicked at src/scraper.rs:195:31:
Failed to parse url: http://Proclamation%20of%20Malaysia | Error: invalid domain character
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at src/scraper.rs:334:10:
called `Result::unwrap()` on an `Err` value: Any { .. }
danielgallups@Daniels-MacBook-Pro pages %
*/

/*

build a reqwest client that will scrape every page's words, and then click on every link on the page and scrape those words, and so on
and will happen in parallel

*/
fn main() {
    println!("Hello, world!");

    let mut words = HashSet::new();

    for entry in WalkDir::new("wiki") {
        let Ok(entry) = entry else {
            println!("warn: BAD ENTRY");
            continue;
        };
        let Ok(file) = File::open(entry.path()) else {
            println!("warn: could not open file! {:?}", entry.path());
            continue;
        };

        let mut reader = quick_xml::Reader::from_reader(BufReader::new(file));
        let mut trash = Vec::new();
        loop {
            match reader.read_event_into(&mut trash) {
                Ok(Event::Text(text)) => {
                    let escaped_text = match text.unescape() {
                        Ok(t) => t,
                        Err(e) => {
                            println!("warn: could not unescape text! {:?}", e);
                            continue;
                        }
                    };

                    let text_words = escaped_text.split(' ');
                    for word in text_words {
                        if let Some((first, second)) = word.split_once(r#"",""#) {
                            words.insert(clean_str(first));
                            words.insert(clean_str(second));
                        } else {
                            words.insert(clean_str(word));
                        }
                    }
                }
                Ok(Event::Eof) | Err(_) => break,
                Ok(_) => {}
            }
        }
    }

    let mut arr_words: Vec<String> = words.into_iter().collect();
    arr_words.sort();

    println!("done");
    let word_list = arr_words
        .into_iter()
        .filter_map(|word| {
            if word.len() > 20 {
                None
            } else {
                Some(word + "\n")
            }
        })
        .collect::<String>();

    let Ok(mut file) = File::create("words.txt") else {
        panic!();
    };
    file.write_all(word_list.as_bytes()).unwrap();
}

fn clean_str(s: &str) -> String {
    let init = s
        .trim()
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
        .to_ascii_lowercase();

    init.chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        .collect()
}
