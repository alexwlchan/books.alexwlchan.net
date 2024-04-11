# frozen_string_literal: true

require 'fileutils'

require_relative 'pillow/create_cover'

module Thumbnails
  class GenerateThumbnails < Jekyll::Generator
    def generate(site)
      source = site.config['source']
      destination = site.config['destination']

      Dir["#{source}/covers/**/*"].each do |cover_path|
        next if File.directory? cover_path

        create_cover({
                       'in_path' => cover_path,
                       'out_path' => cover_path.gsub("#{source}/covers/", "#{destination}/thumbs/"),
                       'max_width' => 110 * 2,
                       'max_height' => 130 * 2
                     })
      end
    end
  end
end
