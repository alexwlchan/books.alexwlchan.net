---
layout: list_of_reviews
---

## books i’ve read

{% assign reviews = site.posts | grouped_reviews %}

{% assign this_year = site.time | date: '%Y' %}

<p id="jumpTo">
  jump to:

  {% for entry in reviews %}
    {% assign year = entry[0] %}
    {% assign reviews_in_year = entry[1] %}

    <a id="jumpTo-{{ year }}" href="#year_{{ year }}">{{ year }}</a>

    {% if forloop.last == false %}
      <span aria-hidden="true">/</span>
    {% endif %}
  {% endfor %}
</p>

<p id="filterBar">
  filter by:
  <button id="tagFilters">tags</button>
  <button id="ratingFilters">rating</button>
  <button id="authorFilters">author</button>
  <button id="publicationYearFilters">publication year</button>
</p>

<p id="filtersApplied"></p>

<p id="noResults">
  There are no reviews that match those filters.
</p>

{% for entry in reviews %}
  {% assign year = entry[0] %}
  {% assign reviews_in_year = entry[1] %}

  {% assign finished_books = 0 %}

  {% for rev in reviews_in_year %}
    {% if rev["review"]["did_not_finish"] != true %}
      {% assign finished_books = finished_books | plus: 1 %}
    {% endif %}
  {% endfor %}

  <hr data-year="{{ year }}">

  <h3
    id="year_{{ year }}"
    class="year_heading"
    data-year="{{ year }}"
    {% if year == this_year %}data-is-this-year{% endif %}
  >
    {{ year }}:
    {% if year == this_year %}
      I’ve read {{ finished_books }} book{% if finished_books > 1 %}s{% endif %} so far
    {% else %}
      I read {{ finished_books }} book{% if finished_books > 1 %}s{% endif %}
    {% endif %}
  </h3>

  {% for review_entry in reviews_in_year %}
    {% include review_preview.html %}
  {% endfor %}
{% endfor %}

<script>
  /* Create the initial filter state.
   * TODO: Do I want filters to have permalinks?  If so, this should
   * be loaded from URL query state. */
  var filters = createEmptyFilters();

  window.addEventListener("DOMContentLoaded", (event) => {

    /* Filters require JavaScript, so they're hidden by default and
     * made visible on initial page load.
     * cf. corresponding CSS in style.css */
    document.getElementById("filterBar").style.display = "block";

    createAuthorFilter(filters);
    createPublicationYearFilter(filters);
    createRatingFilter(filters);
    createTagFilter(filters);
  });
</script>