# frozen_string_literal: true

require 'fileutils'
require 'chunky_png'
require 'tmpdir'

require_relative 'pillow/create_ico_image'

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
        site.posts.docs
            .map { |p| p.data.dig('book', 'cover', 'tint_color') }
            .filter { |c| !c.nil? }
            .to_set

      colors << '191919'

      FileUtils.mkdir_p "#{dst}/favicons"

      colors.each do |c|
        hex_string = c.gsub('#', '')

        ico_path = "#{dst}/favicons/#{hex_string}.ico"
        png_path = "#{dst}/favicons/#{hex_string}.png"

        next if (File.exist? ico_path) && (File.exist? png_path)

        image16 = ChunkyPNG::Image.from_file("#{src}/static/favicon_16.png")
        image32 = ChunkyPNG::Image.from_file("#{src}/static/favicon_32.png")

        Dir.mktmpdir do |tmp_dir|
          fill_color = Color::RGB.by_hex(c)

          colorise_image(image16, fill_color)
          image16.save("#{tmp_dir}/favicon-16x16.png", :best_compression)

          colorise_image(image32, fill_color)
          image32.save("#{tmp_dir}/favicon-32x32.png", :best_compression)

          create_ico_image(
            "#{tmp_dir}/favicon-16x16.png",
            "#{tmp_dir}/favicon-32x32.png",
            "#{tmp_dir}/favicon.ico"
          )

          FileUtils.mv "#{tmp_dir}/favicon-32x32.png", png_path
          FileUtils.mv "#{tmp_dir}/favicon.ico", ico_path
        end
      end
    end
  end
end
