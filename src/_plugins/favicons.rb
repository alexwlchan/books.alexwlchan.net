# frozen_string_literal: true

require 'fileutils'
require 'chunky_png'
require 'tmpdir'

require_relative 'utils/favicons'

module Favicons
  class GenerateFavicons < Jekyll::Generator
    def generate(site)
      dst = site.config['destination']

      colors =
        site.posts.docs
            .map { |p| p.data.dig('book', 'cover', 'tint_color') }
            .compact
            .to_set

      colors << '191919'

      colors.each do |c|
        create_favicon("#{dst}/favicons", c)
      end
    end
  end
end
