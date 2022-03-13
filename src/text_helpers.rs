use phf::phf_map;
use regex::Regex;

pub fn markdown(s: &str) -> String {
    let parser = pulldown_cmark::Parser::new_ext(&s,  pulldown_cmark::Options::all());

    let mut body = String::new();
    pulldown_cmark::html::push_html(&mut body, parser);

    body
}

pub fn smartypants(s: &str) -> String {
    markdown(s).replace("<p>", "").replace("</p>", "")
}

pub fn star_rating(rating: usize) -> String {
    assert!(rating <= 5);
    format!("{}{}", "★".repeat(rating), "☆".repeat(5 - rating))
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
//     date_obj = datetime.datetime(
//         year=int(date_match.group("year")),
//         month=int(date_match.group("month")),
//         day=int(date_match.group("day") or "1"),
//     )
//
//     if date_match.group("day"):
//         return render_date(date_obj)
//     else:
//         return date_obj.strftime("%B %Y")