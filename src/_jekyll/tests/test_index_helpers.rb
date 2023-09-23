# frozen_string_literal: true

require 'test/unit'

require_relative '../plugins/index_helpers'

class TestCreateCreditLine < Test::Unit::TestCase
  def test_single_author_credit_line
    book = {
      'contributors' => [
        { 'name' => 'J.K. Rowling' }
      ],
      'publication_year' => '2005'
    }

    assert_equal(create_credit_line(book), 'by J.K. Rowling (2005)')
  end

  def test_single_editor_credit_line
    book = {
      'contributors' => [
        { 'name' => 'Michael DiBernardo', 'role' => 'editor' }
      ],
      'publication_year' => '2016'
    }

    assert_equal(create_credit_line(book), 'edited by Michael DiBernardo (2016)')
  end

  def test_retold_by_credit_line
    book = {
      'contributors' => [
        { 'name' => 'Vera Southgate', 'role' => 'retold by' },
        { 'name' => 'Eric Winter', 'role' => 'illustrator' }
      ],
      'publication_year' => '1966'
    }

    assert_equal(create_credit_line(book), 'retold by Vera Southgate (1966)')
  end

  def test_author_with_single_other_credit
    book = {
      'contributors' => [
        { 'name' => 'Laura Imai Messina' },
        { 'name' => 'Lucy Rand', 'role' => 'translator' }
      ],
      'publication_year' => '2020'
    }

    assert_equal(create_credit_line(book), 'by Laura Imai Messina (2020)')
  end
end
