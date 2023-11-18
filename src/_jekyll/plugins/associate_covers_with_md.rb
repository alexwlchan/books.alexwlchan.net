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
  site.pages.each do |page|
    if page.data['layout'] != 'review'
      next
    end

    slug = page.path.gsub(/\.md$/, '')
    matching_images = Dir.glob("#{site.config['source']}/covers/#{slug}.*")

    if matching_images.length == 1
      if page.data['book']['cover']['name'] == File.basename(matching_images[0])
        # puts "Redundant info in #{page.path}"
      end

      page.data['book']['cover']['name'] = File.basename(matching_images[0])
    else
      puts "Can't find a cover image for #{page.path}"
    end
  end
end
