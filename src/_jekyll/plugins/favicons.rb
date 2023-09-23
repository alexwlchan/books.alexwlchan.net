# frozen_string_literal: true

require 'fileutils'
require 'chunky_png'
require 'shell/executer'

# Given a ChunkyPNG image with grayscale pixels and a tint colour, create
# a colourised version of that image.
def colorise_image(image, tint_colour)
  0.upto(image.width - 1) do |x|
    0.upto(image.height - 1) do |y|
      image.set_pixel(
        x, y,
        ChunkyPNG::Color.rgba(
          tint_colour.red.to_i,
          tint_colour.green.to_i,
          tint_colour.blue.to_i,
          image.get_pixel(x, y)
        )
      )
    end
  end
end

module Favicons
  class GenerateFavicons < Jekyll::Generator
    def generate(site)
      src = site.config['source']
      dst = site.config['destination']

      colors =
        site.pages
            .map { |p| p.data.dig('book', 'cover', 'tint_color') }
            .filter { |c| !c.nil? }
            .to_set

      FileUtils.mkdir_p "#{dst}/favicons"

      colors.each do |c|
        hex_string = c.gsub('#', '')

        ico_path = "#{dst}/favicons/#{hex_string}.ico"
        png_path = "#{dst}/favicons/#{hex_string}.png"

        next if (File.exist? ico_path) && (File.exist? png_path)

        image16 = ChunkyPNG::Image.from_file("#{src}/static/favicon_16.png")
        image32 = ChunkyPNG::Image.from_file("#{src}/static/favicon_32.png")

        Dir.mktmpdir do |tmp_dir|
          Dir.chdir(tmp_dir) do
            fill_colour = Color::RGB.by_hex(c)

            colorise_image(image16, fill_colour)
            image16.save('favicon-16x16.png', :best_compression)

            colorise_image(image32, fill_colour)
            image32.save('favicon-32x32.png', :best_compression)

            # Create an ICO favicon by packing the two PNG images.
            # See https://superuser.com/a/1012535/243137
            Shell.execute!('convert favicon-16x16.png favicon-32x32.png favicon.ico')
          end

          FileUtils.mv "#{tmp_dir}/favicon-32x32.png", png_path
          FileUtils.mv "#{tmp_dir}/favicon.ico", ico_path
        end
      end
    end
  end
end
