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
  const hasFilters = filters['authors'].length > 0;

  // First work out which books these filters apply to.
  //
  // The filters are AND-ed -- so if there are author and rating filters,
  // a book has to match both of them.
  const selectedReviews = Array.from(document.querySelectorAll(".review_preview"))
    .filter(
      rp =>
        filters['authors'].length === 0 ||
        rp.getAttribute('data-book-authors').split(';').some(a => filters['authors'].indexOf(a) !== -1)
    );

  const yearTally = Counter(selectedReviews.map(rp => rp.getAttribute("data-review-year")));
  const selectedReviewIds = new Set(selectedReviews.map(rp => rp.getAttribute("id")));

  // Show/hide the individual reviews
  document.querySelectorAll(".review_preview").forEach(rp =>
    rp.style.display = selectedReviewIds.has(rp.getAttribute("id")) ? "block" : "none"
  );

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
    } else if (thisYear === "another_time") {
      yh.innerHTML = 'books i read at another time';
    } else {
      yh.innerHTML = `the ${yearTally[thisYear]} book${yearTally[thisYear] > 1 ? 's' : ''} i read in ${thisYear}`;
    }
  });

  // Update the "jump to" links for individual years.
  document.querySelectorAll("#jumpTo a").forEach(jt => {
    const thisYear = jt.getAttribute("data-group-year");
    console.log(thisYear);
    console.log(yearTally[thisYear]);

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
  document.querySelector("#another_time_books").style.display = yearTally["another_time"] > 0 ? "block" : "none";

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

function removeAuthorFilter(filters, name) {
  filters['authors'] = filters['authors'].filter(n => n !== name);

  createAuthorTippy();
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
