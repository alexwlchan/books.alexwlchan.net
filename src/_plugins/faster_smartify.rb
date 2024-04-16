# frozen_string_literal: true

# This wraps Jekyll's built-in `smartify` plugin with a Jekyll Cache.
#
# This makes a significant impact on the speed of the site build,
# cutting it roughly in half!

module Jekyll
  module Filters
    alias old_smartify smartify

    def smartify(input)
      cache = Jekyll::Cache.new('Smartify')

      cache.getset(input) do
        old_smartify(input)
      end
    end
  end
end
