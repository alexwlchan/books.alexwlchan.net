# frozen_string_literal: true

# This plugin adds a default role of "author" to any contributor that
# doesn't have an explicit role set.

Jekyll::Hooks.register :site, :post_read do |site|
  site.posts.docs.each do |post|
    post.data['book']['contributors'].each do |contributor|
      unless contributor.key?('role')
        contributor['role'] = 'author'
      end
    end
  end
end
