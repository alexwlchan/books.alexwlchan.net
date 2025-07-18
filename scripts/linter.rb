# frozen_string_literal: true

require 'html-proofer'
require 'json'
require 'json-schema'
require 'nokogiri'
require 'rainbow'
require 'uri'
require 'yaml'

def run_html_linting(html_dir)
  HTMLProofer.check_directory(
    html_dir, {
      checks: %w[
        Images
        Links
        Scripts
        Favicon
        OpenGraph
      ],
      check_external_hash: false,
      check_html: true,
      check_opengraph: true,
      disable_external: true,
      report_invalid_tags: true,
      ignore_files: [
        '_site/static/tests/test_filters.html'
      ],
      #
      # As of April 2024, I have 334 links which don't use HTTPS.
      # It might be nice to fix them all and/or whitelist them, but
      # they're all external links -- I don't care that much.
      #
      # For now, skip HTTPS checking.
      enforce_https: false
    }
  ).run
end

# These commands are based on the logging in html-proofer; see
# https://github.com/gjtorikian/html-proofer/blob/041bc94d4a029a64ecc1e48036e94eafbae6c4ad/lib/html_proofer/log.rb
def info(message)
  puts Rainbow(message).send(:blue)
end

def error(message)
  puts Rainbow(message).send(:red)
end

# Parse all the generated HTML documents with Nokogiri.
def get_html_documents(html_dir)
  Dir["#{html_dir}/**/*.html"]
    # Anything in the /files/ directory can be ignored, because it's
    # not part of the site, it's a static asset.
    #
    # e.g. if I've got a file that I'm using to demo a particular
    # HTML feature.
    .filter { |html_path| !html_path.include? '/files' }
    # This page is a special case for crawlers and doesn't count for
    # the purposes of linting and the like.
    .filter { |html_path| html_path != "#{html_dir}/400/index.html" }
    .map do |html_path|
      doc = Nokogiri::HTML(File.open(html_path))
      display_path = get_display_path(html_path, doc)

      {
        display_path:,
        doc:
      }
    end
end

# Validate the YAML front matter by checking that:
#
#   1. I'm not using undocumented fields
#   2. Fields have appropriate values
#
def check_yaml_front_matter(src_dir)
  errors = Hash.new { [] }

  info('Checking YAML front matter...')

  schema = JSON.parse(File.read('front-matter.json'))

  Dir["#{src_dir}/**/*.md"].each do |md_path|
    next if md_path.start_with?('src/_plugins/')

    # The YAML loader will try to be "smart" (e.g. reading dates as
    # proper Ruby date types), which is unhelpful for json-schema checking.
    #
    # Make sure everything is JSON-esque (i.e. strings/numbers/bools)
    # before passing to the json-schema gem.
    front_matter = YAML.load(
      File.read(md_path).split("\n---\n")[0],
      permitted_classes: [Date, Time]
    )
    front_matter = JSON.parse(JSON.dump(front_matter))

    md_errors = JSON::Validator.fully_validate(schema, front_matter)

    errors[md_path] = md_errors unless md_errors.empty?
  end

  report_errors(errors)
end

def localhost_link?(anchor_tag)
  !anchor_tag.attribute('href').nil? &&
    anchor_tag.attribute('href').value.start_with?('http') &&
    anchor_tag.attribute('href').value.include?('localhost:5757')
end

# Check I haven't used localhost URLs anywhere (in links or images)
#
# This is an error I've occasionally made while doing local development;
# I'll use my ;furl snippet to get the front URL, and forget to remove
# the localhost development prefix.
def check_no_localhost_links(html_documents)
  errors = Hash.new { [] }

  info('Checking there aren’t any localhost links...')

  html_documents.each do |html_doc|
    localhost_links = html_doc[:doc].xpath('//a')
                                    .select { |a| localhost_link?(a) }
                                    .map { |a| a.attribute('href').value }

    unless localhost_links.empty?
      errors[html_doc[:display_path]] <<= "There are links to localhost: #{localhost_links.join('; ')}"
    end
  end

  report_errors(errors)
end

# Check I haven't got HTML in titles; this can break the formatting
# of Google and social media previews.
def check_no_html_in_titles(html_documents)
  errors = Hash.new { [] }

  info('Checking there isn’t any HTML in titles...')

  html_documents.each do |html_doc|
    # Look for HTML in the '<title>' element in the '<head>'.
    #
    # We can't just look for angle brackets, because at least one post
    # does have HTML-looking stuff in its title
    # (Remembering if a <details> element was opened).
    #
    # What we want to check is if there's any unescaped HTML that
    # needs removing.
    title = html_doc[:doc].xpath('//head/title').children

    if title.children.length > 1
      errors[html_doc[:display_path]] <<= "Title contains HTML: #{title}"
    end
  end

  report_errors(errors)
end

def parse_caddy_redirects(path)
  File.readlines(path).each.with_index(1)
      .filter { |line, _| !line.start_with? '#' }
      .filter { |line, _| !line.strip.empty? }
      .filter { |line, _| line.start_with? 'redir' }
      .map do |line, lineno|
        {
          line:,
          lineno:,
          source: line.strip.split[1],
          target: line.strip.split[2]
        }
      end
end

# Check my redirects point to real pages.
#
# This ensures that any redirects I create are working.  It doesn't mean
# I can't forget to create a redirect, but it does mean I won't create
# a redirect that points to another broken page.
def check_redirects(dst_dir)
  info('Checking redirects...')

  bad_lines = []

  parse_caddy_redirects('redirects.Caddyfile').each do |redirect|
    target = redirect[:target].split('#')[0]

    lineno = redirect[:lineno]
    line = redirect[:line]

    expected_file =
      if target.end_with? '/'
        "#{dst_dir}#{target}/index.html"
      else
        "#{dst_dir}/#{target}"
      end
    bad_lines << [lineno, line.strip] unless File.exist? expected_file
  end

  return if bad_lines.empty?

  error('- redirects.Caddyfile')
  error('  The following lines are redirecting to broken resources:')
  bad_lines.each do |ln|
    lineno, line = ln
    error("  * L#{lineno}:\t#{line}")
  end
  exit!
end

# Check I have redirects set up for every sub-URL of a published URL
# e.g. if I have an article /2013/my-post, there's something at /2013/
# that redirects.  In the sense that "good URLs are hackable".
#
# Quoting the slightly formal language of Nielsen Norman [1]:
#
#     A usable site requires […] URLs that are "hackable" to allow users
#     to move to higher levels of the information architecture by hacking
#     off the end of the URL
#
# Let's make sure I'm doing that!
def check_all_urls_are_hackable(dst_dir)
  info('Checking all HTML pages are navigable...')

  # Get a list of which paths will return an HTML page.
  #
  # This means either:
  #
  #     - There's a redirect that takes you to another page, or
  #     - There's a folder with an index.html file that will be served
  #
  # The goal is to have two sets of URLs without trailing slashes,
  # e.g. {'/writing', '/til'}
  #
  redirects = parse_caddy_redirects('redirects.Caddyfile').to_set { |r| r[:source].chomp('/') }
  folders_with_index_html = Dir.glob("#{dst_dir}/**/index.html").map { |p| File.dirname(p).gsub(dst_dir, '') }

  # Go through and work out all the URLs that somebody could
  # "hack" their way towards.
  #
  # e.g. if there's a file `/blog/2013/01/my-post/index.html` which will
  # be served from `/blog/2013/01/my-post`, then somebody could hack
  # their way to get to:
  #
  #     - /
  #     - /blog/
  #     - /blog/2013/
  #     - /blog/2013/01/
  #
  hackable_urls = Dir.glob("#{dst_dir}/**/*.html")
                     .filter { |p| !p.start_with?("#{dst_dir}/static/tests/") }
                     .flat_map do |p|
    dirs = []

    while (p = File.dirname(p))

      if p == dst_dir
        break
      end

      dirs << p.gsub(dst_dir, '')
    end

    dirs
  end

  hackable_urls = hackable_urls.to_set

  # Now go through and work out which URLs are unreachable.
  unreachable_urls = hackable_urls - (redirects + folders_with_index_html)

  return if unreachable_urls.empty?

  error('- Missing pages/redirects!')
  error('  The following URLs can be "hacked" but won’t resolve:')
  unreachable_urls.sort.each do |url|
    error("  * #{url}/")
  end
  error('  Considering adding an entry in src/_redirects')
  exit!
end

def report_errors(errors)
  # This is meant to look similar to the output from HTMLProofer --
  # errors are grouped by filename, so they can be easily traced
  # back to the problem file.
  return if errors.empty?

  errors.each do |display_path, messages|
    error("- #{display_path}")
    messages.each do |m|
      error("  *  #{m}")
    end
  end
  exit!
end

def get_display_path(html_path, doc)
  # Look up the Markdown file that was used to create this file.
  #
  # This means the error report can link to the source file, not
  # the rendered HTML file.
  #
  # Note that we may fail to retrieve this value if for some reason
  # the `<meta>` tag hasn't been written properly, in which case
  # we show the HTML path instead.
  md_path = doc.xpath("//meta[@name='page-source-path']").attribute('content')

  if md_path == '' || md_path.nil?
    html_path
  else
    "src/#{md_path}"
  end
end

html_dir = '_site'
src_dir = 'src'

run_html_linting(html_dir)

html_documents = get_html_documents(html_dir)

check_yaml_front_matter(src_dir)
check_no_localhost_links(html_documents)
check_no_html_in_titles(html_documents)
check_redirects(html_dir)
check_all_urls_are_hackable(html_dir)
