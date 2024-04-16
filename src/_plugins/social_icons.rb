# frozen_string_literal: true

require 'fileutils'

require_relative 'pillow/create_social_icon'

module SocialIcons
  class GenerateSocialIcons < Jekyll::Generator
    def generate(site)
      source = site.config['source']
      destination = site.config['destination']

      Dir["#{source}/covers/**/*"].each do |cover_path|
        next if File.directory? cover_path

        social_icon_path = cover_path.gsub("#{source}/covers/", "#{destination}/social_icons/")

        create_social_icon(cover_path, social_icon_path)
      end
    end
  end
end
