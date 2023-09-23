require 'fileutils'
require 'rszr'

module IndividualCovers
  class GenerateIndividualCovers < Jekyll::Generator
    def generate(site)
      source = site.config["source"]
      destination = site.config["destination"]

      Dir["#{source}/covers/**/*"].each { |src_path|
        if File.directory? src_path
          next
        end

        dst_path = src_path.gsub("#{source}/covers/", "#{destination}/individual_covers/")

        if File.exist? dst_path
          next
        end

        FileUtils.mkdir_p File.dirname(dst_path)

        im = Rszr::Image.load(src_path)
        im.resize!(180 * 2, 240 * 2, crop: false)

        im.save(dst_path)
      }
    end
  end
end