use regex::Regex;

pub fn slugify(s: &str) -> String {
    // Replace separating punctuation
    let punctuation_regex = Regex::new("[–—/:;,.]").unwrap();
    let s = punctuation_regex.replace_all(s, "-").to_string();

    // Best ASCII substitutions, lowercased
    let s = unidecode::unidecode(&s).to_lowercase();

    // Delete any other characters
    let non_ascii_regex = Regex::new("[^a-z0-9 -]").unwrap();
    let s = non_ascii_regex.replace_all(&s, "").to_string();

    // Convert spaces to hyphens
    let s = s.replace(" ", "-");

    // Condense repeated hyphens
    let repeated_hyphen_regex = Regex::new("-+").unwrap();
    let s = repeated_hyphen_regex.replace_all(&s, "-");

    s.to_string()
}

macro_rules! slugify_tests {
  ($($name:ident: $value:expr,)*) => {
    $(
      #[test]
      fn $name() {
        let (input, expected) = $value;
        assert_eq!(expected, slugify(input));
      }
    )*
  }
}

slugify_tests! {
  ascii_title_with_spaces: (
    "On the Origin of Species",
    "on-the-origin-of-species"
  ),
}
