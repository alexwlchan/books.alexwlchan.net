# frozen_string_literal: true

require 'fileutils'

require_relative 'pillow/create_cover'

module IndividualCovers
  class GenerateIndividualCovers < Jekyll::Generator
    def generate(site)
      source = site.config['source']
      destination = site.config['destination']

      Dir["#{source}/covers/**/*"].each do |in_path|
        next if File.directory? in_path

        create_cover({
                       'in_path' => in_path,
                       'out_path' => in_path.gsub("#{source}/covers/", "#{destination}/individual_covers/"),
                       'max_width' => 180 * 2,
                       'max_height' => 240 * 2
                     })
      end
    end
  end
end
