# frozen_string_literal: true

require 'rszr'

require_relative 'css_helpers'

module Jekyll
  module IndexHelpers
    def all_reviews(pages)
      pages
        .filter { |p| p['url'] != '/reviews/' and p['url'] != '/' }
        .sort_by { |p| p['review']['date_read'].to_s }
        .reverse
    end

    def grouped_reviews(pages)
      all_reviews(pages)
        .group_by { |p| p['review']['date_read'].to_s[0..3] }
        .to_a
        .sort_by { |year, _| year }
        .reverse
    end

    def year_read(review)
      if review['date_read'].is_a? String
        review['date_read'][0..3]
      else
        review['date_read'].year.to_s
      end
    end

    def derived_cover_info(review_entry)
      year = if review_entry['review']['date_read'].is_a? Date
               review_entry['review']['date_read'].year
             else
               review_entry['review']['date_read'][0..3]
             end

      filename = review_entry['book']['cover']['name']

      image = Rszr::Image.load("src/covers/#{year}/#{filename}")

      { 'width' => image.width, 'height' => image.height }
    end

    def credit_line(book)
      create_credit_line(book)
    end
  end
end

def get_attribution_credit(contributors_by_role)
  # Only one contributor, an author
  #
  # e.g. "by J.K. Rowling"
  if contributors_by_role.keys == ['author'] && contributors_by_role['author'].length == 1
    author_name = contributors_by_role['author'][0]
    "by #{author_name}"

  # Only one contributor, an editor
  #
  # e.g. "edited by Michael DiBernardo"
  elsif contributors_by_role.keys == ['editor'] && contributors_by_role['editor'].length == 1
    editor_name = contributors_by_role['editor'][0]
    "edited by #{editor_name}"

  # A retold by contributor and no other writing credits
  elsif contributors_by_role.keys.sort == ['illustrator', 'retold by'] && contributors_by_role['retold by'].length == 1
    retold_by_name = contributors_by_role['retold by'][0]
    "retold by #{retold_by_name}"

  else
    '<unknown>'
  end
end

def create_credit_line(book)
  publication_year = book['publication_year']

  contributors_by_role = Hash.new([])

  book['contributors'].each do |c|
    role = c['role'].nil? ? 'author' : c['role']

    contributors_by_role[role] += [c['name']]
  end

  attribution_line = get_attribution_credit(contributors_by_role)

  "#{attribution_line} (#{publication_year})"
end

Liquid::Template.register_filter(Jekyll::IndexHelpers) if defined? Liquid
