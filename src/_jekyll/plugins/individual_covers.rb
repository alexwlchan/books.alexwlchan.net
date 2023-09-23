# frozen_string_literal: true

require 'fileutils'
require 'rszr'

module IndividualCovers
  class GenerateIndividualCovers < Jekyll::Generator
    def generate(site)
      source = site.config['source']
      destination = site.config['destination']

      Dir["#{source}/covers/**/*"].each do |src_path|
        next if File.directory? src_path

        dst_path = src_path.gsub("#{source}/covers/", "#{destination}/individual_covers/")

        next if File.exist? dst_path

        FileUtils.mkdir_p File.dirname(dst_path)

        im = Rszr::Image.load(src_path)
        im.resize!(180 * 2, 240 * 2, crop: false)

        im.save(dst_path)
      end
    end
  end
end
