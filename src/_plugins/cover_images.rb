# frozen_string_literal: true

# Sort out the cover images for each book.
#
# I give the Markdown file and the cover image the same slug.
# For example, if the MD file is called `night-train.md`, then the
# cover image will be `night-train.jpg` or `night-train.png`.
#
# This plugin does three things:
#
#   - It creates the thumbnail image shown on /reviews/
#   - It creates the full-sized image shown on the individual review page
#   - It adds information about the cover image to the review object
#

require 'abbrev'

require_relative 'pillow/create_cover'
require_relative 'pillow/get_image_info'

Jekyll::Hooks.register :site, :post_read do |site|
  source = site.config['source']
  destination = site.config['destination']

  # Construct a hash that maps normalised name to original path,
  # e.g. if there's an image called src/covers/2023/white-fragility.jpg,
  # then we'd get a hash like
  #
  #     {"2023/white-fragility" => "src/covers/2023/white-fragility.jpg"}
  #
  cover_image_names =
    Dir.glob("#{source}/covers/**/*")
       .select { |e| File.file? e }
       .to_h do |f|
         year = File.basename(File.dirname(f))
         name = File.basename(f, '.*') # remove any extension
         ["#{year}/#{name}", f]
       end

  # Work out a unique prefix for each cover image, so we can generate
  # short filenames, e.g.
  #
  #     {"2023/white-fragility"=>"23/wh"}
  #
  cover_image_prefixes =
    Abbrev.abbrev(cover_image_names.keys)
          .group_by { |_, v| v }
          .transform_values { |v| v.flatten.min_by(&:length) }
          .transform_values { |v| v.gsub(/^20/, '') }

  # Now go through and attach the cover image path to each review, e.g.
  # the White Fragility post gets
  #
  #     {"name"   => "white-fragility.jpg",
  #      "prefix" => "23/wh",
  #      "width"  => 1525,
  #      "height" => 2338}
  #
  site.posts.docs.each do |post|
    year = File.basename(File.dirname(post.path))
    name = File.basename(post.path)
               .gsub(/^\d{4}-\d{2}-\d{2}-/, '')
               .gsub(/\.md$/, '')
    slug = "#{year}/#{name}"

    if cover_image_names.include? slug
      this_cover_path = cover_image_names[slug]

      post.data['book']['cover']['path'] = this_cover_path
      post.data['book']['cover']['name'] = File.basename(this_cover_path)
      post.data['book']['cover']['prefix'] = cover_image_prefixes[slug]

      info = get_single_image_info(this_cover_path)
      post.data['book']['cover']['width'] = info['width']
      post.data['book']['cover']['height'] = info['height']
    else
      puts "Can't find a cover image for #{post.path} (#{slug})"
      next
    end

    # Now go ahead and create a thumbnail for each post.
    #
    # This will be stored at /t/{prefix}, so it's a short path.
    cover = post.data['book']['cover']

    thumbnail_path = "/t/#{cover['prefix']}#{File.extname(cover['path'])}"

    create_cover({
                   'in_path' => cover['path'],
                   'out_path' => "#{destination}#{thumbnail_path}",
                   'max_width' => 110 * 2,
                   'max_height' => 130 * 2
                 })

    cover['thumbnail_path'] = thumbnail_path

    # Now go ahead and create a cover image for each post.
    #
    # This will be stored at /individual_covers/#{name}, because it's
    # only used on a single page and expressiveness is more useful
    # than brevity here.
    cover = post.data['book']['cover']

    create_cover({
                   'in_path' => cover['path'],
                   'out_path' => cover['path'].gsub("#{source}/covers/", "#{destination}/individual_covers/"),
                   'max_width' => 180 * 2,
                   'max_height' => 240 * 2
                 })
  end
end
