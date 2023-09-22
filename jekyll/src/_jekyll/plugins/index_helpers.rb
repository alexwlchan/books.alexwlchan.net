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
  end
end

Liquid::Template.register_filter(Jekyll::IndexHelpers)
