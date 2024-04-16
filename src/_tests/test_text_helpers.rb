# frozen_string_literal: true

require 'test/unit'

require_relative '../_plugins/text_helpers'

class TestTextHelpers < Test::Unit::TestCase
  def test_render_date_read
    assert_equal(render_date_read('2003'), '2003')
    assert_equal(render_date_read('2003-01'), 'January 2003')
    assert_equal(render_date_read('2003-01-05'), '5 January 2003')
  end
end
