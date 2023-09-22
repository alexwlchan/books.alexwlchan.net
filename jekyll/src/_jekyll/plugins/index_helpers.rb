require 'rszr'

require_relative 'css_helpers'

module Jekyll
  module IndexHelpers
    def grouped_reviews(pages)
      pages
        .filter { |p| p["url"] != "/reviews/" }
        .sort_by { |p| p["review"]["date_read"] }
        .reverse
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
  end
end

Liquid::Template.register_filter(Jekyll::IndexHelpers)
