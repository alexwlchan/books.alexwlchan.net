function isUndefined(t) {
  return typeof t === 'undefined';
}

function isNotUndefined(t) {
  return !isUndefined(t);
}

/** Creates a Counter of an array, similar to Python's collections.Counter().
  * e.g.
  *
  *    > Counter(['a', 'b', 'c', 'b', 'a', 'b', 'c', 'a', 'a', 'a'])
  *    {"a": 5, "b": 3, "c": 2}
  *
  * By nitsas on Stack Overflow: https://stackoverflow.com/a/44189621/1558022,
  * used under CC-BY SA 3.0.
  */
function Counter(array) {
  var count = {};
  array.forEach(val => count[val] = (count[val] || 0) + 1);
  return count;
}

/** Apply the current set of filters to the page.
  *
  * This updates all the page state, including which reviews/headings are
  * visible.  This should be called whenever the filter state changes.
  */
function applyFilters(filters) {
  const hasFilters =
    filters['authors'].length > 0 ||
    isNotUndefined(filters['publicationYear']['before']) ||
    isNotUndefined(filters['publicationYear']['after']) ||
    isNotUndefined(filters['starRating']) ||
    filters['tags'].length > 0;

  // First work out which books these filters apply to.
  //
  // The filters are AND-ed -- so if there are author and rating filters,
  // a book has to match both of them.
  const selectedReviews = Array.from(document.querySelectorAll(".review_preview"))
    .filter(
      rp =>
        filters['authors'].length === 0 ||
        rp.getAttribute('data-book-authors').split(';').some(a => filters['authors'].indexOf(a) !== -1)
    )
    .filter(
      rp =>
        isUndefined(filters['publicationYear']['after']) ||
        rp.getAttribute("data-publication-year") >= filters['publicationYear']['after']
    )
    .filter(
      rp =>
        isUndefined(filters['publicationYear']['before']) ||
        rp.getAttribute("data-publication-year") <= filters['publicationYear']['before']
    )
    .filter(
      rp =>
        isUndefined(filters['starRating']) ||
        rp.hasAttribute('data-star-rating') && Number(rp.getAttribute('data-star-rating')) >= filters['starRating']
    )
    .filter(
      rp => {
        const rpTags = new Set(rp.getAttribute('data-review-tags').split(';'));

        return filters['tags'].length === 0 || filters['tags'].every(t => rpTags.has(t));
      }
    );

  const yearTally = Counter(selectedReviews
    .filter(rp => rp.getAttribute("data-did-not-finish") !== "true")
    .map(rp => rp.getAttribute("data-review-year"))
  );
  const selectedReviewIds = new Set(selectedReviews.map(rp => rp.getAttribute("id")));

  // Show/hide the individual reviews
  document.querySelectorAll(".review_preview").forEach(rp => {
    if (rp.getAttribute("data-review-year") == "another time") {
      rp.style.display = selectedReviewIds.has(rp.getAttribute("id")) ? "grid" : "none"
    } else {
      rp.style.display = selectedReviewIds.has(rp.getAttribute("id")) ? "block" : "none"
    }
  });

  // Show/hide the year headings, and the dividers between them.
  //
  // Also update the summary text, e.g. 'the 15 books i read in 2021'
  document.querySelectorAll(".divider").forEach(dv =>
    dv.style.display = yearTally[dv.getAttribute("data-group-year")] > 0 ? "block" : "none"
  );

  document.querySelectorAll(".year_heading").forEach(yh => {
    const thisYear = yh.getAttribute("data-group-year");

    yh.style.display = yearTally[thisYear] > 0 ? "block" : "none";
    if (yh.hasAttribute("data-is-current-year")) {
      yh.innerHTML = `the ${yearTally[thisYear]} book${yearTally[thisYear] > 1 ? 's' : ''} i&rsquo;ve read so far in ${thisYear}`;
    } else if (thisYear === "another time") {
      yh.innerHTML = 'books i read at another time';
    } else {
      yh.innerHTML = `the ${yearTally[thisYear]} book${yearTally[thisYear] > 1 ? 's' : ''} i read in ${thisYear}`;
    }
  });

  // Update the "jump to" links for individual years.
  document.querySelectorAll("#jumpTo a").forEach(jt => {
    const thisYear = jt.getAttribute("data-group-year");

    if (yearTally[thisYear] > 0 ) {
      jt.removeAttribute("disabled");
      if (jt.hasAttribute("data-disabled-href")) {
        jt.setAttribute("href", jt.getAttribute("data-disabled-href"));
        jt.removeAttribute("data-disabled-href");
      }
    } else {
      jt.setAttribute("disabled", "true");
      jt.setAttribute("data-disabled-href", jt.getAttribute("href"));
      jt.removeAttribute("href");
    }
  });

  // The "read at another time" books are stored in a <details> element,
  // we show/hide the parent collapsible element.
  document.querySelector("#another_time_books").style.display = yearTally["another time"] > 0 ? "block" : "none";

  if (yearTally["another time"] > 0 && Object.keys(yearTally).filter(k => yearTally[k] > 0).length === 1) {
    document.querySelector("#another_time_books").open = true;
  }

  // Update the list of selected filters, which also allows the user to
  // remove filters.
  if (hasFilters) {
    document.getElementById("filtersApplied").innerHTML = "selected filters: ";

    for (let name of filters['authors']) {
      document.getElementById("filtersApplied").innerHTML += `
        <span class="appliedFilter">
          <span class="appliedFilterValue">${name}</span>
          <a href="#" onclick="script:removeAuthorFilter(filters, '${name}')" class="removeFilter">[x]</span>
        </span>
      `;
    }

    if (isNotUndefined(filters['publicationYear']['after']) && isNotUndefined(filters['publicationYear']['before'])) {
      document.getElementById("filtersApplied").innerHTML += `
        <span class="appliedFilter">
          <span class="appliedFilterValue">published between ${filters['publicationYear']['after']} and ${filters['publicationYear']['before']}</span>
          <a href="#" onclick="script:removePublicationYearFilters(filters)" class="removeFilter">[x]</span>
        </span>
      `
    } else if (isNotUndefined(filters['publicationYear']['after'])) {
      document.getElementById("filtersApplied").innerHTML += `
        <span class="appliedFilter">
          <span class="appliedFilterValue">published after ${filters['publicationYear']['after']}</span>
          <a href="#" onclick="script:removePublicationYearFilters(filters)" class="removeFilter">[x]</span>
        </span>
      `
    } else if (isNotUndefined(filters['publicationYear']['before'])) {
      document.getElementById("filtersApplied").innerHTML += `
        <span class="appliedFilter">
          <span class="appliedFilterValue">published before ${filters['publicationYear']['before']}</span>
          <a href="#" onclick="script:removePublicationYearFilters(filters)" class="removeFilter">[x]</span>
        </span>
      `
    }

    if (isNotUndefined(filters['starRating'])) {
      const label = {
        5: '★★★★★',
        4: '★★★★☆ or higher',
        3: '★★★☆☆ or higher',
        2: '★★☆☆☆ or higher',
        1: '★☆☆☆☆ or higher',
      };

      document.getElementById("filtersApplied").innerHTML += `
        <span class="appliedFilter">
          <span class="appliedFilterValue">${label[filters['starRating']]}</span>
          <a href="#" onclick="script:removeStarRatingFilter(filters)" class="removeFilter">[x]</span>
        </span>
      `
    }

    for (let name of filters['tags']) {
      document.getElementById("filtersApplied").innerHTML += `
        <span class="appliedFilter">
          <span class="appliedFilterValue">${name}</span>
          <a href="#" onclick="script:removeTagFilter(filters, '${name}')" class="removeFilter">[x]</span>
        </span>
      `;
    }

    document.getElementById("filtersApplied").style.display = "block";
  } else {
    document.getElementById("filtersApplied").style.display = "none";
  }
}

function applyAuthorFilters(filters) {
  filters['authors'] = Array.from(document.querySelectorAll("#author_filters input"))
    .filter(input => input.checked)
    .map(input => input.getAttribute("data-author-name"));

  applyFilters(filters);
}

function applyPublicationFilters(filters) {
  const publishedAfter = document.getElementById("published:after").value;
  const publishedBefore = document.getElementById("published:before").value;

  if (publishedAfter.length === 4) {
    filters['publicationYear']['after'] = publishedAfter;
  }

  if (publishedBefore.length === 4) {
    filters['publicationYear']['before'] = publishedBefore;
  }

  applyFilters(filters);
}

function applyStarRatingFilters(filters) {
  filters['starRating'] = Number(
    Array.from(document.querySelectorAll("#star_rating_filters input"))
      .filter(input => input.checked)
      .find(_ => _)
      .value
  );

  applyFilters(filters);
}

function applyTagFilters(filters) {
  filters['tags'] = Array.from(document.querySelectorAll("#tag_filters input"))
    .filter(input => input.checked)
    .map(input => input.getAttribute("data-tag-name"));

  applyFilters(filters);
}

function removeAuthorFilter(filters, name) {
  filters['authors'] = filters['authors'].filter(n => n !== name);

  createAuthorTippy(filters);
  applyFilters(filters);
}

function removePublicationYearFilters(filters) {
  filters['publicationYear'] = {'before': undefined, 'after': undefined};

  createPublicationYearTippy(filters);
  applyFilters(filters);
}

function removeStarRatingFilter(filters) {
  filters['starRating'] = undefined;

  createStarRatingTippy(filters);
  applyFilters(filters);
}

function removeTagFilter(filters, name) {
  filters['tags'] = filters['tags'].filter(n => n !== name);

  createTagTippy(filters);
  applyFilters(filters);
}

/** Create the "Tippy" popovers for each filter.
  *
  * Because the Tippy content isn't accessible when the popover is closed,
  * we have "createTippy()" functions that should be called every time the
  * filter content changes outside the popover itself (i.e. in removeFilter)
  */

function createTippy(id, content) {
  tippy(id, {
    content,
    trigger: 'click',
    placement: 'bottom',
    allowHTML: true,
    theme: 'light',
        // 350px
        maxWidth: '',
    // https://atomiks.github.io/tippyjs/v6/all-props/#interactive
    interactive: true,
  });
}

function createAuthorTippy(filters) {
  const authorsSet = new Set(
    [...document.querySelectorAll('.review_preview')]
      .flatMap(rp => rp.getAttribute('data-book-authors').split(';'))
      .filter(s => s.length > 0)
  );
  const authors = Array.from(authorsSet);
  authors.sort();

  createTippy(
    '#authorFilters',
    `
      <ul id="author_filters" style="padding: 0; margin: 0; list-style: none; padding-right: 10px;">
        ${
          authors.map((name, i) =>
            `<li>
               <input
                  id="author:${name}"
                  type="checkbox"
                  ${filters['authors'].indexOf(name) !== -1 ? 'checked' : ''}
                  name="author"
                  data-author-name="${name}"
                  onchange="applyAuthorFilters(filters)"
                >
               <label for="author:${name}">${name}</label>
            </li>`
          ).join("")
        }
      </ul>
    `
  )
}

function createPublicationYearTippy(filters) {
  createTippy(
    '#publicationFilters',
    `
      published between
      <input
        id="published:after"
        type="text"
        onchange="applyPublicationFilters(filters)"
        placeholder="year"
        size="4"
        ${typeof filters['publicationYear']['after'] !== 'undefined' ? `value="${filters['publicationYear']['after']}"` : ''}
      >
      and
      <input
        id="published:before"
        type="text"
        onchange="applyPublicationFilters(filters)"
        placeholder="year"
        size="4"
        ${typeof filters['publicationYear']['before'] !== 'undefined' ? `value="${filters['publicationYear']['before']}"` : ''}
      >
    `
  );
}

function createStarRatingTippy(filters) {
  createTippy(
    '#ratingFilters',
    `
      <ul id="star_rating_filters" style="padding: 0; margin: 0; list-style: none; padding-right: 10px;">
        <li><input onchange="applyStarRatingFilters(filters)" name="star_rating" type="radio" value="5" id="star_rating:5" ${filters['starRating'] === 5 ? 'checked' : ''}><label for="star_rating:5"> ★★★★★</label></li>
        <li><input onchange="applyStarRatingFilters(filters)" name="star_rating" type="radio" value="4" id="star_rating:4" ${filters['starRating'] === 4 ? 'checked' : ''}><label for="star_rating:4"> ★★★★☆ or higher</label></li>
        <li><input onchange="applyStarRatingFilters(filters)" name="star_rating" type="radio" value="3" id="star_rating:3" ${filters['starRating'] === 3 ? 'checked' : ''}><label for="star_rating:3"> ★★★☆☆ or higher</label></li>
        <li><input onchange="applyStarRatingFilters(filters)" name="star_rating" type="radio" value="2" id="star_rating:2" ${filters['starRating'] === 2 ? 'checked' : ''}><label for="star_rating:2"> ★★☆☆☆ or higher</label></li>
        <li><input onchange="applyStarRatingFilters(filters)" name="star_rating" type="radio" value="1" id="star_rating:1" ${filters['starRating'] === 1 ? 'checked' : ''}><label for="star_rating:1"> ★☆☆☆☆ or higher</label></li>
      </ul>
    `
  );
}

function createTagTippy(filters) {
  const tagsSet = new Set(
    [...document.querySelectorAll('.review_preview')]
      .flatMap(rp => rp.getAttribute('data-review-tags').split(';'))
      .filter(s => s.length > 0)
  );
  const tags = Array.from(tagsSet);
  tags.sort();

  createTippy(
    '#tagFilters',
    `
      <ul id="tag_filters" style="padding: 0; margin: 0; list-style: none; padding-right: 10px;">
        ${
          tags.map((name, i) =>
            `<li>
               <input
                  id="tags:${name}"
                  type="checkbox"
                  ${filters['tags'].indexOf(name) !== -1 ? 'checked' : ''}
                  name="tags"
                  data-tag-name="${name}"
                  onchange="applyTagFilters(filters)"
                >
               <label for="tags:${name}">${name}</label>
            </li>`
          ).join("")
        }
      </ul>
    `
  )
}