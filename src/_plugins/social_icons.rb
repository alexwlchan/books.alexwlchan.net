# frozen_string_literal: true

require 'fileutils'

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

def create_social_icon(cover_path, social_icon_path)
  return if File.exist? social_icon_path

  require 'vips'

  im = Vips::Image.new_from_file cover_path

  # Resize the image to match the target width
  scale = [480.to_f / im.width, 480.to_f / im.height].min
  resized = im.resize(scale)

  # Create the parent directory, if it doesn't exist already
  FileUtils.mkdir_p File.dirname(social_icon_path)

  # Create a white background image
  white_background = Vips::Image.new_from_array([[255]], 1, 1)
  if im.bands == 3
    white_background = white_background
                       .bandjoin([255, 255])
  end
  white_background = white_background
                     .cast(im.format) # Match image format
  white_background = white_background
                     .embed(0, 0, 480, 480, extend: :white)

  # Calculate top-left coordinates to centre the resized image
  left = ((white_background.width - resized.width) / 2.0).floor
  top = ((white_background.height - resized.height) / 2.0).floor

  # Composite resized image onto background
  padded = white_background.insert(resized, left, top)

  padded.write_to_file social_icon_path
end
