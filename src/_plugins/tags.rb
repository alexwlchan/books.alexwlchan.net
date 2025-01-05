# frozen_string_literal: true

# This plugin creates an index of all the tags I'm using, and works
# out a unique prefix for each.

require 'abbrev'

Jekyll::Hooks.register :site, :post_read do |site|
  # This hook runs before the site is built, and adds the following fields
  # to the `site` object:
  #
  #   - `tag_names` is a list of all the tags used, e.g. ["fiction", "romance"]
  #
  #   - `tag_prefixes` is a map of unique prefixes for each tag name,
  #     e.g. {"fiction" => "f", "romance" => "ro"}
  #
  site.data['tag_names'] =
    site.posts.docs
        .flat_map { |doc| doc.data['book']['tags'] }
        .uniq

  site.data['tag_prefixes'] =
    Abbrev.abbrev(site.data['tag_names'])
          .group_by { |_, v| v }
          .transform_values { |v| v.flatten.min_by(&:length) }
end
