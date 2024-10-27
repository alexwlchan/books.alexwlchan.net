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
    {% include review_preview.html %}
  {% endfor %}
  <a href="/reviews/">read more reviews &rarr;</a>
</div>

---

On my main website, I also write an annual roundup of my favourite books from each year:

<ul id="roundups">
  <li>
    <a href="https://alexwlchan.net/2023/2023-in-reading/">
      <div class="roundup_item">
        <img src="/static/roundups/2023-in-reading.jpg" alt="">
        <p>2023</p>
      </div>
    </a>
  </li>
  <li>
    <a href="https://alexwlchan.net/2022/2022-in-reading/">
      <div class="roundup_item">
        <img src="/static/roundups/2022-in-reading.jpg" alt="">
        <p>2022</p>
      </div>
    </a>
  </li>
  <li>
    <a href="https://alexwlchan.net/2021/2021-in-reading/">
      <div class="roundup_item">
        <img src="/static/roundups/2021-in-reading.jpg" alt="">
        <p>2021</p>
      </div>
    </a>
  </li>  
</ul>
