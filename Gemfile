# frozen_string_literal: true

source 'https://rubygems.org'

gem 'chunky_png', '~> 1.4'
gem 'color', '~> 1.8'
gem 'html-proofer', '~> 5'
gem 'jekyll', '~> 4'
gem 'jekyll-include-cache', '~> 0.2'
gem 'nokogiri', '~> 1.18'

# These two gems are here because of warnings that they were loaded
# from the standard library, but won't be included in Ruby 3.4.0.
gem 'abbrev', '~> 0.1.2'
gem 'logger', '~> 1'

group :lint, optional: true do
  gem 'json-schema', '~> 5'
  gem 'rubocop', '~> 1.77'
end

group :test, optional: true do
  gem 'test-unit'
end
