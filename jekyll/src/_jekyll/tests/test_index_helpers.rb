require 'test/unit'

require_relative '../plugins/index_helpers'

class TestCreateCreditLine < Test::Unit::TestCase
  def test_single_author_credit_line
    book = {
      "contributors" => [
        {"name" => "J.K. Rowling"}
      ],
      "publication_year" => "2005",
    }

    assert_equal(create_credit_line(book), "by J.K. Rowling (2005)")
  end

  def test_single_editor_credit_line
    book = {
      "contributors" => [
        {"name" => "Michael DiBernardo", "role" => "editor"}
      ],
      "publication_year" => "2016",
    }

    assert_equal(create_credit_line(book), "edited by Michael DiBernardo (2016)")
  end
end
