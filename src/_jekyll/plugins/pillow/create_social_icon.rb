require 'json'
require 'open3'

def create_social_icon(cover_path, social_icon_path)
  return if File.exist? social_icon_path

  stdout, status = Open3.capture2('python3', 'src/_jekyll/plugins/pillow/create_social_icon.py', cover_path, social_icon_path)
  raise "Unable to create social icon for #{cover_path}" unless status.success?

  stdout
end
