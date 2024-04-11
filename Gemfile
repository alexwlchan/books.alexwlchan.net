source 'https://rubygems.org'

group :build, optional: true do
  gem 'chunky_png', '~> 1.4'
  gem 'color', '~> 1.8'
  gem 'html-proofer', '~> 5'
  gem 'jekyll', '~> 4'
  gem 'jekyll-include-cache', '~> 0.2'
  gem 'nokogiri', '~> 1.16'

  # These two gems are here because of warings that they were loaded
  # from the standard library, but won't be included in Ruby 3.4.0.
  gem 'base64', '~> 0.2'
  gem 'csv', '~> 3'
end

group :lint, optional: true do
  gem 'json-schema', '~> 4'
  gem 'rubocop', '~> 1.63'
end

group :test, optional: true do
  gem 'test-unit'
end
