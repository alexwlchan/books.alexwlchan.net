#!/usr/bin/env ruby
# frozen_string_literal: true

require 'date'
require 'pathname'
require 'open3'

print "What's the title of the book? "
title = gets.chomp
print "What's the slug of the MD file? "
slug = gets.chomp
print 'Who wrote the book? '
author = gets.chomp
print 'When was it published? '
publication_year = gets.chomp

today = Date.today
year = today.year.to_s

out_path = Pathname.new('src/_posts') + year + "#{DateTime.now.strftime('%Y-%m-%d')}-#{slug}.md"

out_path.dirname.mkpath # Create the directory if it doesn't exist

File.write(out_path, <<~HEREDOC)
  ---
  layout: review
  book:
    contributors:
      - name: #{author}
    cover:
      tint_color: "???"
    title: "#{title}"
    publication_year: #{publication_year}
    tags:
      - ???
    isbn13: "???"
  review:
    date_read: ???
    format: ???
    rating: ???
  ---
HEREDOC

# Use system call to open the file with the default application
system('open', out_path.to_s)
