require 'rszr'

require_relative 'css_helpers'

module Jekyll
  module IndexHelpers
    def all_reviews(pages)
      pages
        .filter { |p| p["url"] != "/reviews/" and p["url"] != "/" }
        .sort_by { |p| p["review"]["date_read"] }
        .reverse
    end

    def grouped_reviews(pages)
      all_reviews(pages)
        .group_by { |p| p["review"]["date_read"][0..3] }
        .to_a
        .sort_by { |year, _| year }
        .reverse
    end

    def year_read(review)
      review["date_read"][0..3]
    end

    def derived_cover_info(review_entry)
      year = review_entry["review"]["date_read"][0..3]
      filename = review_entry["book"]["cover"]["name"]

      image = Rszr::Image.load("src/covers/#{year}/#{filename}")

      return { "width" => image.width, "height" => image.height }
    end

    def credit_line(book)
      create_credit_line(book)
    end
  end
end

def create_credit_line(book)
  contributors = book["contributors"]
  publication_year = book["publication_year"]

  if contributors.length == 1 && contributors[0]["label"].nil?
    author = contributors[0]["name"]
    "by #{author} (#{publication_year})"
  end
end

Liquid::Template.register_filter(Jekyll::IndexHelpers) if defined? Liquid
