# frozen_string_literal: true

Jekyll::Hooks.register :site, :post_read do |site|
  # Adds a default role of "author" to any contributor that doesn't have
  # an explicit role set.
  site.posts.docs.each do |post|
    post.data['book']['contributors'].each do |contributor|
      unless contributor.key?('role')
        contributor['role'] = 'author'
      end
    end
  end

  # This hook runs before the site is built, and adds the following fields
  # to the `site` object:
  #
  #   - `author_names` is a list of all the authors used,
  #     e.g. ["Tim Smith", "Ursula K Le Guin"]
  #
  #   - `author_ids` is a map of numeric IDs that can be used to refer to
  #     authors in the data attributes
  #     e.g. {"Tim Smith" => 1, "Ursula K Le Guin" => 2}
  #
  site.data['author_names'] =
    site.posts.docs
        .flat_map { |doc| doc.data['book']['contributors'] }
        .filter { |c| c['role'] == 'author' }
        .map { |c| c['name'] }
        .uniq

  site.data['author_ids'] =
    site.data['author_names'].map.with_index.to_h
end
