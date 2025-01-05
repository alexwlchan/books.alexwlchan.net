# frozen_string_literal: true

require 'json'
require 'open3'

def create_cover(request)
  return if File.exist? request['out_path']

  _, status = Open3.capture2('python3', 'src/_plugins/pillow/create_cover.py', JSON.generate(request))
  raise "Unable to resize image #{request}" unless status.success?
end
