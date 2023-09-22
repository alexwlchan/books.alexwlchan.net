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
  end
end

Liquid::Template.register_filter(Jekyll::TextHelpers)
