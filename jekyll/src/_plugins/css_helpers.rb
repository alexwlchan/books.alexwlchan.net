module Jekyll
  module CssHelpers
    def as_rgba(color, opacity)
      red   = color[1..2].to_i(16)
      green = color[3..4].to_i(16)
      blue  = color[5..6].to_i(16)

      "rgba(#{red}, #{green}, #{blue}, #{opacity})"
    end
  end
end

Liquid::Template.register_filter(Jekyll::CssHelpers)
