require 'chunky_png'
require 'color'

module Jekyll
  module CssHelpers
    def as_rgba(color, opacity)
      red   = color[1..2].to_i(16)
      green = color[3..4].to_i(16)
      blue  = color[5..6].to_i(16)

      "rgba(#{red}, #{green}, #{blue}, #{opacity})"
    end

    def cache
      @@cache ||= Jekyll::Cache.new("ShelfHeaders")
    end

    def create_shelf_data_uri(color)
      hex_string = color.gsub(/#/, "")

      if not cache.key? hex_string
        rgb_color = Color::RGB.by_hex(color)
        hsl_color = rgb_color.to_hsl

        # These two RNGs serve two different purposes:
        #
        #    - The shape RNG creates the shape of the different shelves; we
        #      seed with a constant to ensure a consistent output on all pages.
        #
        #      In particular, as somebody navigates around the site, they should
        #      see the bookshelf changing colours, but it should never change
        #      shape -- that would be too jarring.
        #
        #    - The luminosity RNG chooses the light/dark of individual books
        #      on the shelves.  This is seeded based on the colour so it's
        #      consistent across runs, but is different for each colour so we
        #      get different patterns of light/dark.
        #
        shapes = Random.new(0)
        luminosities = Random.new(rgb_color.red * 256 * 256 + rgb_color.green * 256 + rgb_color.blue * 256)

        png = ChunkyPNG::Image.new(2000, 90, ChunkyPNG::Color::TRANSPARENT)

        x = 0

        while x < png.width
          shelf_width = shapes.rand(4..28)

          # Shelves go from 30px to 45px height, then 2x for retina displays.
          shelf_height = shapes.rand(60..90)

          shelf_color = create_random_colour_like(luminosities, hsl_color)

          png.rect(
            x, 0,
            x + shelf_width, shelf_height,
            ChunkyPNG::Color.rgba(0, 0, 0, 0),
            ChunkyPNG::Color.rgb(shelf_color.red.to_i, shelf_color.green.to_i, shelf_color.blue.to_i)
          )

          x += shelf_width
        end

        cache[hex_string] = png.to_data_url
      end

      cache[hex_string]
    end
  end
end

# Create a random colour that's similar to the given colour.
#
# All this does is modify the "lightness" parameter in HSL space.
# There are probably better ways to create similar colours within a
# given hue (colour is neither linear nor simple), but this creates
# good enough results.
#
# I don't remember how I picked all these constants -- I might have
# chosen them arbitrarily until I got something that looked good.
def create_random_colour_like(luminosities, hsl_color)
  puts hsl_color.lightness
  v = [hsl_color.lightness, 45].min

  min_lightness = [v * 3.0 / 4.0, 0].max
  max_lightness = [v * 4.0 / 3.0, 100].min

  Color::HSL::new(
    hsl_color.hue,
    hsl_color.saturation,
    luminosities.rand(min_lightness..max_lightness)
  ).to_rgb
end

Liquid::Template.register_filter(Jekyll::CssHelpers)
