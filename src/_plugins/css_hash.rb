# frozen_string_literal: true

# This returns the MD5 hash of the CSS file, which is included as
# a URL query parameter, so the cache will be invalidated whenever
# I change the CSS.

require 'digest'

module Jekyll
  class CssHashTag < Liquid::Tag
    def render(context)
      site = context.registers[:site]
      source = site.config['source']
      Digest::MD5.file("#{source}/static/style.css").hexdigest
    end
  end
end

Liquid::Template.register_tag('css_hash', Jekyll::CssHashTag)
