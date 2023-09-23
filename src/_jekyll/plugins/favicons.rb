# frozen_string_literal: true

require 'fileutils'
require 'chunky_png'
require 'shell/executer'

module Favicons
  class GenerateFavicons < Jekyll::Generator
    def generate(site)
      source = site.config["source"]
      destination = site.config["destination"]

      colors =
        site.pages
          .map { |p| p.data.dig("book", "cover", "tint_color") }
          .filter { |c| not c.nil? }
          .to_set

      FileUtils.mkdir_p "#{destination}/favicons"

      mask16 = ChunkyPNG::Image.from_file("#{source}/static/favicon_16.png")
      mask32 = ChunkyPNG::Image.from_file("#{source}/static/favicon_32.png")

      colors.each do |color|
        hex_string = color.gsub(/#/, "")
        rgb_color = Color::RGB.by_hex(color)

        ico_path = "#{destination}/favicons/#{hex_string}.ico"
        png_path = "#{destination}/favicons/#{hex_string}.png"

        if File.exist? ico_path
          next
        end

        Dir.mktmpdir do |tmp_dir|
          Dir.chdir(tmp_dir) do
            img16 = ChunkyPNG::Image.new(16, 16, ChunkyPNG::Color::TRANSPARENT)

            for x in 0..15 do
              for y in 0..15 do
                img16[x, y] = ChunkyPNG::Color.rgba(
                  rgb_color.red.to_i,
                  rgb_color.green.to_i,
                  rgb_color.blue.to_i,
                  mask16[x, y]
                )
              end
            end

            img16.save("favicon-16x16.png")

            img32 = ChunkyPNG::Image.new(32, 32, ChunkyPNG::Color::TRANSPARENT)

            for x in 0..31 do
              for y in 0..31 do
                img32[x, y] = ChunkyPNG::Color.rgba(
                  rgb_color.red.to_i,
                  rgb_color.green.to_i,
                  rgb_color.blue.to_i,
                  mask32[x, y]
                )
              end
            end

            img32.save("favicon-32x32.png")

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
