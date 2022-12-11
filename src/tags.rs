use std::collections::HashSet;
use std::path::Path;

use inquire::autocompletion::Autocomplete;
use inquire::autocompletion::Replacement;
use inquire::error::CustomUserError;

use crate::render_html;

/// Returns a set of all tags used in any review.
pub fn get_used_tags(root: &Path) -> HashSet<String> {
    let mut used_tags = HashSet::new();

    render_html::get_reviews(root)
        .unwrap_or(vec![])
        .iter()
        .for_each(|rv| {
            for t in rv.book.tags.to_owned().unwrap_or(vec![]) {
                used_tags.insert(t);
            }
        });

    used_tags
}

#[derive(Clone)]
pub struct TagCompleter {
    tags: Vec<String>,
    suggestions: Vec<String>,
    prefix: String,
}

impl TagCompleter {
    pub fn new(tags: Vec<String>) -> Self {
        Self {
            tags: tags.clone(),
            suggestions: tags,
            prefix: "".to_owned(),
        }
    }
}

fn update_based_on_input(tag_completer: &mut TagCompleter, input: &str) -> () {
    if input.is_empty() {
        tag_completer.prefix.clear();
        tag_completer.suggestions = tag_completer.tags.to_vec();
        tag_completer.suggestions.sort();
        return ();
    }

    // What tags have already been used?  Tags can only be selected
    // once, so we don't want to suggest a tag already in the input.
    let input_tags = input.split_whitespace();

    let used_tags: HashSet<&str> = HashSet::from_iter(input_tags);

    // What's the latest tag the user is typing?  i.e. what are we trying
    // to autocomplete on this tag.
    let last_char_is_space = input.chars().last().unwrap().is_whitespace();
    let this_tag = if last_char_is_space {
        None
    } else {
        input.split_whitespace().last()
    };

    tag_completer.prefix = if last_char_is_space {
        input.to_string()
    } else if let Some(tag) = this_tag {
        input[..(input.len() - tag.len())].to_string()
    } else {
        unreachable!();
    };

    tag_completer.suggestions = tag_completer
        .tags
        .iter()
        .filter(|s| !used_tags.contains(s.as_str()))
        // Note: this will filter to all the matching tags if the user
        // is midway through matching a tag (e.g. "adventure ac" -> "action"),
        // but will also display *all* the tags on the initial prompt.
        //
        // If there are lots of tags, that might be unwieldy.
        .filter(|s| match this_tag {
            None => true,
            Some(t) => s.contains(&t),
        })
        .take(15)
        .map(|s| s.to_owned())
        .collect();

    tag_completer.suggestions.sort();
}

impl Autocomplete for TagCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        update_based_on_input(self, input);
        Ok(self.suggestions.iter().map(|s| s.to_string()).collect())
    }

    fn get_completion(
        &mut self,
        input: &str,
        selected_suggestion: Option<String>,
    ) -> Result<inquire::autocompletion::Replacement, CustomUserError> {
        update_based_on_input(self, input);

        let completion = match selected_suggestion {
            None => self.suggestions.first().map(|s| s.to_owned()),
            Some(suggestion) => Some(suggestion),
        };

        let completion = match completion {
            Some(c) => match self.prefix.is_empty() {
                true => Replacement::Some(format!("{} ", c)),
                false => {
                    let separator = if self.prefix.chars().last().unwrap().is_whitespace() {
                        ""
                    } else {
                        " "
                    };
                    Replacement::Some(format!("{}{}{} ", self.prefix, separator, c))
                }
            },
            None => Replacement::None,
        };

        Ok(completion)
    }
}

#[cfg(test)]
mod tests {
    use inquire::Autocomplete;

    use crate::tags::TagCompleter;

    #[test]
    fn it_offers_all_options_initially() {
        let mut ac = TagCompleter::new(vec!["adventure", "action", "mystery", "romance", "scifi"]);

        let suggestions = ac.get_suggestions("").unwrap();

        assert_eq!(
            suggestions,
            vec!["adventure", "action", "mystery", "romance", "scifi"]
        );
    }

    #[test]
    fn it_offers_all_options_with_a_matching_substring() {
        let mut ac = TagCompleter::new(vec!["adventure", "action", "mystery", "romance", "scifi"]);

        let suggestions = ac.get_suggestions("s").unwrap();

        assert_eq!(suggestions, vec!["mystery", "scifi"]);
    }

    #[test]
    fn it_only_offers_unused_options() {
        let mut ac = TagCompleter::new(vec!["adventure", "action", "mystery", "romance", "scifi"]);

        let suggestions = ac.get_suggestions("scifi s").unwrap();

        assert_eq!(suggestions, vec!["mystery"]);
    }

    #[test]
    fn it_offers_no_options_if_no_matches() {
        let mut ac = TagCompleter::new(vec!["adventure", "action", "mystery", "romance", "scifi"]);

        let suggestions = ac.get_suggestions("scifi z").unwrap();

        assert_eq!(suggestions.len(), 0);

        let mut ac = TagCompleter::new(vec!["adventure", "action", "mystery", "romance", "scifi"]);

        let suggestions = ac.get_suggestions("z").unwrap();

        assert_eq!(suggestions.len(), 0);
    }
}
