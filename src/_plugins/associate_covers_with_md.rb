# frozen_string_literal: true

# This plugin adds the name of the cover file to the front matter.
#
# I try to name the text file and image file consistently,
# e.g. if the text of the review if 'again-again.md', then the name
# of the cover will be 'again-again.jpg' or 'again-again.png'.
#
# Rather than repeat information, this hook adds the name of the
# cover file (with the appropriate extension) to every review on
# the site.
#
# It's as if I'd specified the cover name in the YAML front matter,
# but I don't actually have to do that.

Jekyll::Hooks.register :site, :post_read do |site|
  # Construct a hash that maps normalised name to original path,
  # e.g. if there's an image called src/covers/2023/white-fragility.jpg,
  # then we'd get a hash like
  #
  #     {"2023/white-fragility" => "src/covers/2023/white-fragility.jpg"}
  #
  cover_image_names =
    Dir.glob("#{site.config['source']}/covers/**/*")
       .select { |e| File.file? e }
       .to_h do |f|
         year = File.basename(File.dirname(f))
         name = File.basename(f, '.*') # remove any extension
         ["#{year}/#{name}", f]
       end

  site.posts.docs.each do |post|
    year = File.basename(File.dirname(post.path))
    name = File.basename(post.path).gsub(/^\d{4}-\d{2}-\d{2}-/, '').gsub(/\.md$/, '')
    slug = "#{year}/#{name}"

    if cover_image_names.include? slug
      post.data['book']['cover']['name'] = File.basename(cover_image_names[slug])
    else
      puts "Can't find a cover image for #{post.path} (#{slug})"
    end
  end
end
