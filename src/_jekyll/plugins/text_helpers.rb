# frozen_string_literal: true

require 'date'

def render_date_read(date_string)
  if date_string.instance_of? Date
    date_string.strftime('%-d %B %Y')
  elsif date_string.match(/^\d{4}$/)
    date_string
  elsif date_string.match(/^\d{4}-\d{2}$/)
    DateTime.parse("#{date_string}-01").strftime('%B %Y')
  else
    DateTime.parse(date_string).strftime('%-d %B %Y')
  end
end

module Jekyll
  module TextHelpers
    def book_description(book)
      if book['author']
        "#{book['title']}, by #{book['author']}"
      else
        book['title']
      end
    end

    def star_rating(rating)
      ('★' * rating) + ('☆' * (5 - rating))
    end

    def spread_star_rating(rating)
      ('★ ' * rating) + ('☆ ' * (5 - rating))
    end

    def review_description(review)
      return '' if review['did_not_finish']

      if review['date_read'] && review['rating']
        "Read #{review['date_read']}, #{star_rating(review['rating'])}"
      elsif review['date_read']
        "Read #{review['date_read']}"
      end
    end

    def date_read(date_string)
      render_date_read(date_string)
    end

    def get_dimensions(cover_dimensions)
      if (cover_dimensions['width'] / 110.0) > (cover_dimensions['height'] / 130.0)
        width = '110'
        height = (cover_dimensions['height'] * 110 / cover_dimensions['width']).round
      else
        width = (cover_dimensions['width'] * 130 / cover_dimensions['height']).round
        height = '130'
      end

      "width: #{width}px; height: #{height}px;"
    end

    def is_plural(contributor_name)
      match = contributor_name =~ / and |, | & | with /
      !match.nil?
    end
  end
end

Liquid::Template.register_filter(Jekyll::TextHelpers) if defined? Liquid
