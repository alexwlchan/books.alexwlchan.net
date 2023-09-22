require 'date'

def render_date_read(date_string)
  if date_string.match(/^\d{4}$/)
    date_string
  elsif date_string.match(/^\d{4}-\d{2}$/)
    DateTime.parse("#{date_string}-01").strftime("%B %Y")
  else
    DateTime.parse(date_string).strftime("%-d %B %Y")
  end
end

module Jekyll
  module TextHelpers
    def book_description(book)
      if book["author"]
        "#{book["title"]}, by #{book["author"]}"
      else
        book["title"]
      end
    end

    def star_rating(rating)
      "★" * rating + "☆" * (5 - rating)
    end

    def review_description(review)
      if review["did_not_finish"]
        return ""
      end

      if review["date_read"] and review["rating"]
        "Read #{review["date_read"]}, #{star_rating(review["rating"])}"
      elsif review["date_read"]
        "Read #{review["date_read"]}"
      end
    end

    def date_read(date_string)
      render_date_read(date_string)
    end
  end
end

Liquid::Template.register_filter(Jekyll::TextHelpers) if defined? Liquid
