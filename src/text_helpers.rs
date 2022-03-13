use phf::phf_map;
use regex::Regex;

pub fn markdown(s: &str) -> String {
    let parser = pulldown_cmark::Parser::new_ext(&s,  pulldown_cmark::Options::all());

    let mut body = String::new();
    pulldown_cmark::html::push_html(&mut body, parser);

    body
}

pub fn smartypants(s: &str) -> String {
    markdown(s).replace("<p>", "").replace("</p>", "").trim().to_string()
}

pub fn star_rating(rating: usize) -> String {
    assert!(rating <= 5);
    format!("{}{}", "★".repeat(rating), "☆".repeat(5 - rating))
}

pub fn spread_star_rating(rating: usize) -> String {
    assert!(rating <= 5);
    format!("{}{}", "★ ".repeat(rating), "☆ ".repeat(5 - rating))
}

static MONTHS: phf::Map<&'static str, &'static str> = phf_map! {
    "01" => "January",
    "02" => "February",
    "03" => "March",
    "04" => "April",
    "05" => "May",
    "06" => "June",
    "07" => "July",
    "08" => "August",
    "09" => "September",
    "10" => "October",
    "11" => "November",
    "12" => "December",
};

pub fn render_date(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<year>\d{4})-(?P<month>\d{2})(?:-(?P<day>\d{2}))?$").unwrap();
    }

    let date_match = (*RE).captures(s).unwrap();

    let year = date_match.name("year").unwrap().as_str();
    let month = MONTHS[date_match.name("month").unwrap().as_str()];

    match date_match.name("day") {
        Some(d) => format!("{} {} {}", d.as_str().trim_start_matches('0'), month, year),
        None    => format!("{} {}", month, year),
    }
}

#[cfg(test)]
mod tests {
    use crate::text_helpers::*;

    #[test]
    fn test_star_rating() {
        assert_eq!(star_rating(1), "★☆☆☆☆");
        assert_eq!(star_rating(2), "★★☆☆☆");
        assert_eq!(star_rating(3), "★★★☆☆");
        assert_eq!(star_rating(4), "★★★★☆");
        assert_eq!(star_rating(5), "★★★★★");
    }

    #[test]
    fn test_render_date() {
        assert_eq!(render_date("2002-01"), "January 2002");
        assert_eq!(render_date("2003-05-01"), "1 May 2003");
        assert_eq!(render_date("2004-06-17"), "17 June 2004");
    }
}

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
