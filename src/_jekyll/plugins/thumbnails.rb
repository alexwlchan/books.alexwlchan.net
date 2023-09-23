# frozen_string_literal: true

require 'fileutils'
require 'rszr'

module Thumbnails
  class GenerateThumbnails < Jekyll::Generator
    def generate(site)
      source = site.config['source']
      destination = site.config['destination']

      Dir["#{source}/covers/**/*"].each do |cover_path|
        next if File.directory? cover_path

        thumbnail_path = cover_path.gsub("#{source}/covers/", "#{destination}/thumbs/")

        next if File.exist? thumbnail_path

        FileUtils.mkdir_p File.dirname(thumbnail_path)

        im = Rszr::Image.load(cover_path)
        im.resize!(110 * 2, 130 * 2, crop: false)

        im.save(thumbnail_path)
      end
    end
  end
end
