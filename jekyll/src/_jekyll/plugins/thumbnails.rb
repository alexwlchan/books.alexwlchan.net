require 'fileutils'
require 'rszr'

module Thumbnails
  class GenerateThumbnails < Jekyll::Generator
    def generate(site)
      source = site.config["source"]
      destination = site.config["destination"]

      Dir["#{source}/covers/**/*"].each { |cover_path|
        if File.directory? cover_path
          next
        end

        thumbnail_path = cover_path.gsub("#{source}/covers/", "#{destination}/thumbs/")

        if File.exist? thumbnail_path
          next
        end

        FileUtils.mkdir_p File.dirname(thumbnail_path)

        im = Rszr::Image.load(cover_path)
        im.resize!(110 * 2, 130 * 2, crop: false)

        im.save(thumbnail_path)
      }
    end
  end
end