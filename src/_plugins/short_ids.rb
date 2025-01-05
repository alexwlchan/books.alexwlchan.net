# frozen_string_literal: true

# This plugin gives each review a short ID.
#
# These short IDs are used on /reviews/ in place of the full URL, so
# I can reduce the page weight.

require 'abbrev'

def create_slug(post)
  year = File.basename(File.dirname(post.path)).gsub(/^20/, '')
  name = File.basename(post.path)
             .gsub(/^\d{4}-\d{2}-\d{2}-/, '')
             .gsub(/\.md$/, '')
  slug = "#{year}-#{name}"
end

Jekyll::Hooks.register :site, :post_read do |site|
  post_ids =
    Abbrev.abbrev(site.posts.docs.map { |post| create_slug(post) })
          .group_by { |_, v| v }
          .transform_values { |v| v.flatten.min_by(&:length) }

  site.posts.docs.each do |post|
    post.data['short_id'] = post_ids[create_slug(post)]
  end
end
