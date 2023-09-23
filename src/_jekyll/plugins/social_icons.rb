# frozen_string_literal: true

require 'fileutils'
require 'rszr'

module SocialIcons
  class GenerateSocialIcons < Jekyll::Generator
    def generate(site)
      source = site.config['source']
      destination = site.config['destination']

      Dir["#{source}/covers/**/*"].each do |cover_path|
        next if File.directory? cover_path

        social_icon_path = cover_path.gsub("#{source}/covers/", "#{destination}/social_icons/")

        next if File.exist? social_icon_path

        FileUtils.mkdir_p File.dirname(social_icon_path)

        im = Rszr::Image.load(cover_path)

        background = Rszr::Image.new(480, 480, background: Rszr::Color::White)

        if im.width > im.height
          # landscape orientation
          im.resize!(480, :auto)
          background.blend!(im, 0, (480 - im.height) / 2)

        else
          # portrait orientation
          im.resize!(:auto, 480)
          background.blend!(im, (480 - im.width) / 2, 0)

        end
        background.save(social_icon_path)
      end
    end
  end
end
