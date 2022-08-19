use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use inquire::{error::CustomUserError, Text};

use crate::models;
use crate::render_html::get_reviews;

fn get_tags() -> HashMap<String, usize> {
    println!("@@AWLC Calling get_tags()!!!!");

    let root = Path::new("reviews");

    let mut tally: HashMap<String, usize> = HashMap::new();

    for rev in get_reviews(&root).unwrap().iter() {
        match &rev.book.tags {
            None => (),
            Some(tags) => {
                for t in tags {
                    tally.entry(t.into()).and_modify(|count| *count += 1).or_insert(1);
                }
            }
        };
    }

    tally
}

fn suggester(tags: &HashMap<String, usize>, val: &str) -> Result<Vec<String>, CustomUserError> {
    let tags_set: HashSet<String> = tags.keys().cloned().collect();

    // What tags have already been used?  Tags can only be selected
    // once, so we don't want to suggest a tag already in the input.
    let used_tags: Vec<String> = val.split_whitespace().into_iter().map(|s| s.to_string()).collect();
    let used_tags: HashSet<String> = HashSet::from_iter(used_tags);
    let mut available_tags: Vec<String> = tags_set.difference(&used_tags).cloned().collect();
    available_tags.sort_by(|a, b| {
        let count_a = tags.get(a).unwrap();
        let count_b = tags.get(b).unwrap();
        count_b.partial_cmp(count_a).unwrap()
    });

    // What's the latest tag the user is typing?  i.e. what are we trying
    // to autocomplete on this tag.
    let this_tag = val.split_whitespace().last();

    let prefix = match this_tag {
        None => val,
        Some(t) => &val[..(val.len() - t.len())],
    };

    Ok(available_tags
        .iter()
        // Note: this will filter to all the matching tags if the user
        // is midway through matching a tag (e.g. "adventure ac" -> "action"),
        // but will also display *all* the tags on the initial prompt.
        //
        // If there are lots of tags, that might be unwieldy.
        .filter(|s| match this_tag {
            None => true,
            Some(t) => s.contains(&t),
        })
        // Note: the prefix may be empty if the user hasn't typed
        // anything yet.
        .map(|s| {
            if prefix.is_empty() {
                format!("{} ", s)
            } else {
                format!("{} {} ", prefix.trim_end(), s)
            }
        })
        .collect())
}

fn completer(tags: &HashMap<String, usize>, val: &str) -> Result<Option<String>, CustomUserError> {
    let suggestions = suggester(tags, val)?;

    if suggestions.len() == 1 {
        Ok(Some(suggestions[0].clone()))
    } else {
        Ok(None)
    }
}
//
// #[cfg(test)]
// mod tests {
//     use crate::suggester;
//
//     #[test]
//     fn it_offers_all_options_initially() {
//         let result = suggester("");
//         assert_eq!(
//             result.unwrap(),
//             vec!["adventure ", "fiction ", "mystery ", "romance ", "scifi "]
//         );
//     }
//
//     #[test]
//     fn it_offers_all_options_with_a_matching_substring() {
//         let result = suggester("s");
//         assert_eq!(result.unwrap(), vec!["mystery ", "scifi "]);
//     }
//
//     #[test]
//     fn it_only_offers_unused_options() {
//         let result = suggester("scifi s");
//         assert_eq!(result.unwrap(), vec!["scifi mystery "]);
//     }
//
//     #[test]
//     fn it_offers_no_options_if_no_matches() {
//         let result = suggester("scifi z");
//         assert_eq!(result.unwrap().len(), 0);
//
//         let result = suggester("z");
//         assert_eq!(result.unwrap().len(), 0);
//     }
// }

pub fn get_tag_value_input(question: &str) -> Vec<String> {
    let tags = get_tags();

    println!("@@AWLC tags = {:?}", tags);

    let answer = Text::new(question)
        .with_suggester(&|val| suggester(&tags, val))
        .with_completer(&|val| completer(&tags, val))
        .prompt()
        .unwrap();

    answer.split_whitespace().into_iter().map(|s| s.to_string()).collect()
}

fn save_review(review: models::Review) -> () {
    let front_matter = models::FrontMatter {
        book: review.book,
        review: review.review.unwrap(),
    };

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(false)
        .open(&review.path)
        .unwrap();
    file.write_all(serde_yaml::to_string(&front_matter).unwrap().as_bytes())
        .unwrap();
    file.write("---".as_bytes()).unwrap();
    file.write(review.text.as_bytes()).unwrap();
}

fn update_tags_on_review(rev: models::Review) -> () {
    let question = format!("What are the tags for \"{}\", by {} ({})", rev.book.title, rev.book.author.as_ref().unwrap_or(&"<none>".to_string()), rev.book.publication_year);
    let tags = get_tag_value_input(&question);

    let updated_book = models::Book {
        tags: Some(tags),
        ..rev.book
    };

    let updated_review = models::Review {

        book: updated_book,
// |               ^^^ expected struct `Review`, found `&Review`
        review: rev.review,
        text: rev.text,
        slug: rev.slug,
        path: rev.path,
        derived_cover_info: rev.derived_cover_info,
        // ..rev
    };

    save_review(updated_review);
}

pub fn backfill_tags() -> () {
    let root = Path::new("reviews");

    for rev in get_reviews(&root).unwrap() {
        if rev.book.tags.is_some() {
            continue;
        }

        update_tags_on_review(rev);

        break;
    }
}
