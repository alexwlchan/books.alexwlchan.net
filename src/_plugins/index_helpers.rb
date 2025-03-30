# frozen_string_literal: true

require_relative 'css_helpers'

module Jekyll
  module IndexHelpers
    def all_reviews(pages)
      pages
        .sort_by { |p| "#{p['review']['date_read']}-#{p['review']['date_order']}" }
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

  elsif contributors_by_role.keys == ['editor'] && contributors_by_role['editor'].length == 2
    editor_name0 = contributors_by_role['editor'][0]
    editor_name1 = contributors_by_role['editor'][1]
    "edited by #{editor_name0} and #{editor_name1}"

  # A retold by contributor and no other writing credits
  elsif contributors_by_role.keys.sort == ['illustrator', 'retold by'] && contributors_by_role['retold by'].length == 1
    retold_by_name = contributors_by_role['retold by'][0]
    "retold by #{retold_by_name}"

  # Otherwise find the author
  elsif contributors_by_role.key?('author') && (contributors_by_role['author'].length == 1)
    author_name = contributors_by_role['author'][0]
    "by #{author_name}"

  elsif contributors_by_role.key?('author') && (contributors_by_role['author'].length == 2)
    author0 = contributors_by_role['author'][0]
    author1 = contributors_by_role['author'][1]
    "by #{author0} and #{author1}"

  else
    raise "Unknown attribution line: #{contributors_by_role}"
  end
end

def create_credit_line(book)
  publication_year = book['publication_year']

  contributors_by_role = Hash.new { [] }

  book['contributors'].each do |c|
    role = c['role'].nil? ? 'author' : c['role']

    contributors_by_role[role] += [c['name']]
  end

  attribution_line = get_attribution_credit(contributors_by_role)

  "#{attribution_line} (#{publication_year})"
end

Liquid::Template.register_filter(Jekyll::IndexHelpers) if defined? Liquid
