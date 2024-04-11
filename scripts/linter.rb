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
      check_html: true,
      check_img_http: true,
      ignore_missing_alt: true,
      check_opengraph: true,
      disable_external: true,
      report_invalid_tags: true
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
    # Skip some Markdown files in the source directory that aren't
    # posts on the site and so don't need validating.
    next if md_path.end_with?('theme/_favicons/README.md')
    next if md_path.end_with?('src/_jekyll/plugins/pillow/README.md')

    # This page is a special case for crawlers and doesn't count for
    # the purposes of linting and the like.
    next if md_path == "#{src_dir}/400.md"

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

    errors[html_doc[:display_path]] <<= "There are links to localhost: #{localhost_links.join('; ')}" unless localhost_links.empty?
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

    errors[html_doc[:display_path]] <<= "Title contains HTML: #{title}" if title.children.length > 1
  end

  report_errors(errors)
end

def parse_netlify_redirects(path)
  File.readlines(path).each_with_index
      .filter { |line, _i| !line.start_with? '#' }
      .filter { |line, _i| !line.strip.empty? }
      .map do |line, i|
        {
          line:,
          lineno: i + 1,
          source: line.strip.split[0],
          target: line.strip.split[1]
        }
      end
end

# Check my Netlify redirects point to real pages.
#
# This ensures that any redirects I create are working.  It doesn't mean
# I can't forget to create a redirect, but it does mean I won't create
# a redirect that points to another broken page.
def check_netlify_redirects(dst_dir)
  info('Checking Netlify redirect rules...')

  bad_lines = []

  parse_netlify_redirects("#{dst_dir}/_redirects").each do |redirect|
    # A couple of special cases that I don't worry about.
    next if redirect[:source] == '/ideas-for-inclusive-events/*'
    next if redirect[:target].start_with? 'https://social.alexwlchan.net/'

    # ignore URL fragments when linting, the important thing is that
    # pages don't 404
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

  error('- src/_redirects')
  error('  The following lines are redirecting to broken resources:')
  bad_lines.each do |ln|
    lineno, line = ln
    error("  * L#{lineno}:\t#{line}")
  end
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

src_dir = "src"
html_dir = "_site"

# run_html_linting(html_dir)

html_documents = get_html_documents(html_dir)

check_yaml_front_matter(src_dir)
check_no_localhost_links(html_documents)
check_no_html_in_titles(html_documents)
# check_netlify_redirects(html_dir)
