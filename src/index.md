---
layout: homepage
---

{% assign recent_reviews = site.posts | all_reviews | slice: 0, 5 %}

# Welcome to my book tracker!

This is a small app I created to track <a href="/reviews/">the books I've read</a>, inspired by [similar book trackers] made by other people.

[similar book trackers]: https://debugger.medium.com/tech-savvy-readers-are-designing-their-own-better-versions-of-goodreads-aac96934d79

Here are a few books I've read recently:

<div class="books_by_year">
  {% for review_entry in recent_reviews %}
    {% include review_entry.html %}
  {% endfor %}
  <a href="/reviews/">read more reviews &rarr;</a>
</div>
