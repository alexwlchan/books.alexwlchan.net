function isUndefined(t) {
  return typeof t === 'undefined';
}

function isNotUndefined(t) {
  return !isUndefined(t);
}

/** Creates a label to describe the current publication year filter. e.g.
  *
  *     published between 2001 and 2002
  *     published in 2004
  *     published before 1990
  *
  */
function createPublicationYearLabel(range) {
  const { beforeYear, afterYear } = range;

  const hasAfterYear = isNotUndefined(afterYear);
  const hasBeforeYear = isNotUndefined(beforeYear);

  const thisYear = new Date().getUTCFullYear().toString();

  const label = hasAfterYear && hasBeforeYear && beforeYear === afterYear
    ? `in ${afterYear}`
    : hasAfterYear && hasBeforeYear
    ? `between ${afterYear} and ${beforeYear}`
    : hasAfterYear && afterYear == thisYear
    ? `in ${thisYear}`
    : hasAfterYear
    ? `after ${afterYear}`
    : `before ${beforeYear}`;

  return `published ${label}`;
}

/** Parses an author string into individual authors, e.g.
  *
  *     John Smith and Sarah Jones
  *      ~> ['John Smith', 'Sarah Jones']
  *
  */
function parseAuthorNames(authors) {
  if (authors === 'Alan & Maureen Carter') {
    return ['Alan Carter', 'Maureen Carter'];
  }

  if (authors === 'Stephen Hawking with Leonard Mlodinow') {
    return ['Stephen Hawking', 'Leonard Mlodinow'];
  }

  return authors
    .split(/,|&| and /)
    .map(s => s.trim())
    .filter(s => s.length > 0);
}

/** Returns the names of all the authors on the "list reviews" page. */
function getAuthorNames() {
   const authors = Array.from(
     new Set(Object.keys(authorNames).flatMap(parseAuthorNames))
   );
   authors.sort();

   return authors;
}

function createEmptyFilters() {
  return {
    authors: [],
    publicationYear: {
      before: undefined,
      after: undefined,
    },
    starRating: undefined,
    tags: [],
  };
}

/** Returns true if a given book matches the specified filters.
  *
  * Here the `filters` has the following structure:
  *
  *     {
  *       authors: string[],
  *       publicationYear: {
  *         before: number | undefined,
  *         after: number | undefined,
  *       },
  *       starRating: number | undefined,
  *       tags: string[],
  *     };
  *
  * This inspects the data attributes on each review preview, including:
  *
  *     - data-bk-a, which is the author IDs for the book
  *     - data-bk-p-yr, which is the publication year of the book
  *     - data-rv-s, which is the star rating for this book
  *     - data-rv-t, which is the tag prefixes for each tag on this book
  */
function matchesFilters(book, filters) {
  // First we need to unpack the author IDs abck to full author names,
  // then we can look to see if at least one of the book's authors is a match.
  const authors = book.getAttribute('data-bk-a').split('-')
    .map(s => Number(s.trim()))
    .map(id => authorIds[id])
    .flatMap(author => parseAuthorNames(author));

  const matchesAuthorFilter =
    filters.authors.length === 0 ||
    authors.some(a => filters.authors.indexOf(a) !== -1);

  // The publication year has to fall within the defined range
  const publicationYear = Number(book.getAttribute('data-bk-p-yr'));

  const matchesPublicationYearAfterFilter =
    isUndefined(filters.publicationYear.after) ||
    filters.publicationYear.after <= publicationYear;

  const matchesPublicationYearBeforeFilter =
    isUndefined(filters.publicationYear.before) ||
    publicationYear <= filters.publicationYear.before;

  // The star rating has to be equal to or higher than the filtered rating
  const starRating = Number(book.getAttribute('data-rv-s'));

  const matchesStarRatingFilter =
    isUndefined(filters.starRating) ||
    filters.starRating <= starRating;

  // It has to match all the tags specified.
  const tags = new Set(
    book.getAttribute('data-rv-t').split('-').map(t => tagIds[t])
  );

  const matchesTagsFilter =
    filters.tags.length === 0 ||
    filters.tags.every(t => tags.has(t));

  return (
    matchesAuthorFilter &&
    matchesPublicationYearAfterFilter &&
    matchesPublicationYearBeforeFilter &&
    matchesStarRatingFilter &&
    matchesTagsFilter
  );
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
    // The default width is 350px, but we want arbitrary width
    maxWidth: '',
    // https://atomiks.github.io/tippyjs/v6/all-props/#interactive
    interactive: true,
  });
}

function createAuthorFilter(filters) {
  const authors = getAuthorNames();

  createTippy(
    "#authorFilters",
    `
      <ul id="author_filters">
        ${
          authors.map((name, i) =>
            `<li>
              <input
                id="author:${name}"
                type="checkbox"
                ${filters['authors'].indexOf(name) !== -1 ? 'checked' : ''}
                name="author"
                data-author-name="${name}"
                onchange="applyAuthorFilters('ul#author_filters input', filters)"
              >
              <label for="author:${name}">${name}</label>
            </li>`
          ).join("")
        }
      </ul>
    `);
}

function applyAuthorFilters(selector, filters) {
  const selectedAuthors = Array.from(document.querySelectorAll(selector))
    .filter(input => input.checked)
    .map(input => input.getAttribute("data-author-name"));

  // This ensures we preserve the order in which filters were applied: any
  // authors you'd already selected retain their position in the list, and
  // new authors appear at the end.
  const existingAuthors = filters['authors'].filter(a => selectedAuthors.indexOf(a) !== -1);
  const newAuthors = selectedAuthors.filter(a => filters['authors'].indexOf(a) === -1);

  filters['authors'] = [...existingAuthors, ...newAuthors];

  applyFilters(filters);
}

function removeAuthorFilter(filters, name) {
  filters['authors'] = filters['authors'].filter(n => n !== name);

  createAuthorFilter(filters);
  applyFilters(filters);
}

function createPublicationYearFilter(filters) {
  createTippy(
    "#publicationYearFilters",
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

function removePublicationYearFilters(filters) {
  filters['publicationYear'] = {'before': undefined, 'after': undefined};

  createPublicationYearFilter(filters);
  applyFilters(filters);
}

function createRatingFilter(filters) {
  const ratings = [
    { value: 5, label: '★★★★★' },
    { value: 4, label: '★★★★☆ or higher' },
    { value: 3, label: '★★★☆☆ or higher' },
    { value: 2, label: '★★☆☆☆ or higher' },
    { value: 1, label: '★☆☆☆☆ or higher' },
  ]

  createTippy(
    "#ratingFilters",
    `
      <ul id="star_rating_filters">
        ${ratings.map(r =>
          `
          <li>
            <input
              onchange="applyRatingFilters(filters)"
              name="star_rating"
              type="radio" value="${r.value}"
              id="star_rating:${r.value}"
              ${filters['starRating'] === r.value ? 'checked' : ''}
            >
            <label for="star_rating:${r.value}"> ${r.label}</label>
          </li>
          `
        ).join("")}
      </ul>
    `
  );
}

function applyRatingFilters(filters) {
  filters['starRating'] = Number(
    Array.from(document.querySelectorAll("#star_rating_filters input"))
      .filter(input => input.checked)
      .find(_ => _)
      .value
  );

  applyFilters(filters);
}

function removeRatingFilter(filters) {
  filters['starRating'] = undefined;

  createRatingFilter(filters);
  applyFilters(filters);
}

function createTagFilter(filters) {
  const tags = Array.from(Object.keys(tagNames));
  tags.sort();

  createTippy(
    "#tagFilters",
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

function applyTagFilters(filters) {
  const selectedTags = Array.from(document.querySelectorAll("#tag_filters input"))
    .filter(input => input.checked)
    .map(input => input.getAttribute("data-tag-name"));

  // This ensures we preserve the order in which filters were applied: any
  // authors you'd already selected retain their position in the list, and
  // new tags appear at the end.
  const existingTags = filters['tags'].filter(a => selectedTags.indexOf(a) !== -1);
  const newTags = selectedTags.filter(a => filters['tags'].indexOf(a) === -1);

  filters.tags = [...existingTags, ...newTags]

  applyFilters(filters);
}

function removeTagFilter(filters, name) {
  filters['tags'] = filters['tags'].filter(n => n !== name);

  createTagFilter(filters);
  applyFilters(filters);
}

function pluralize(number, noun) {
  return `${number} ${noun}${number > 1 ? 's' : ''}`;
}

function createSummaryMessage(options) {
	const { finishedCount, year, isThisYear } = options;

	if (isThisYear) {
		return `${year}: I’ve read ${finishedCount} book${finishedCount > 1 ? 's' : ''} so far`;
	} else {
		return `${year}: I read ${finishedCount} book${finishedCount > 1 ? 's' : ''}`;
	}
}

/** Apply the current set of filters to the page.
  *
  * This updates the page state, including which reviews/headings should
  * be visible.  Call this whenever the filter state changes.
  */
function applyFilters(filters) {
  const selectedReviews = Array.from(document.querySelectorAll('.rv_p'))
    .filter(rev => matchesFilters(rev, filters));

  const selectedReviewIds = new Set(selectedReviews.map(rp => rp.getAttribute("id")));

  document.getElementById("noResults").style.display = selectedReviews.length > 0 ? "none" : "block";

  // How many reviews did I write/finish in each year?  Note that these
  // are slightly different, e.g. if I filter for books by an author and
  // I didn't finish any of their books in a year
  const yearReviewTally = Counter(
    selectedReviews
      .map(rev => rev.getAttribute('data-rv-yr'))
  );

  const yearFinishedTally = Counter(
    selectedReviews
      .filter(rev => !rev.hasAttribute('data-dnf'))
      .map(rev => rev.getAttribute('data-rv-yr'))
  );

  // Show/hide the individual reviews
  document.querySelectorAll('.rv_p').forEach(rev => {
    if (rev.getAttribute('data-rv-yr') === 'another time') {
      rev.style.display = selectedReviewIds.has(rev.getAttribute('id')) ? 'grid' : 'none';
    } else {
      rev.style.display = selectedReviewIds.has(rev.getAttribute('id')) ? 'block' : 'none';
    }
  })

  // Show/hide the year headings, the dividers between them,
  // and the jump to links.
  document.querySelectorAll('.year_heading').forEach(yh => {
    const year = yh.getAttribute('data-year');
    const isThisYear = yh.hasAttribute('data-is-this-year');

    yh.style.display = yearReviewTally[year] > 0 ? 'block' : 'none';

    document.querySelector(`hr[data-year="${year}"]`).style.display =
      yh.style.display;

    const reviewCount = yearReviewTally[year];
    const finishedCount = yearFinishedTally[year];

    if (finishedCount === reviewCount) {
      if (isThisYear) {
        yh.innerHTML = `${year}: I’ve read ${pluralize(finishedCount, 'book')} so far`;
      } else {
        yh.innerHTML = `${year}: I read ${pluralize(finishedCount, 'book')}`;
      }
    } else if (isUndefined(finishedCount)) {
      if (isThisYear) {
        yh.innerHTML = `${year}: so far I’ve started ${pluralize(reviewCount, 'book')}`;
      } else {
        yh.innerHTML = `${year}: I started ${pluralize(reviewCount, 'book')}`;
      }
    } else {
      if (isThisYear) {
        yh.innerHTML = `${year}: so far I’ve started ${pluralize(reviewCount, 'book')}, finished ${finishedCount}`;
      } else {
        yh.innerHTML = `${year}: I started ${pluralize(reviewCount, 'book')}, finished ${finishedCount}`;
      }
    }

    const jumpTo = document.getElementById(`jumpTo-${year}`);

    if (reviewCount > 0) {
      jumpTo.removeAttribute('disabled');
      if (jumpTo.hasAttribute('data-disabled-href')) {
        jumpTo.setAttribute('href', jumpTo.getAttribute('data-disabled-href'));
        jumpTo.removeAttribute('data-disabled-href');
      }
    } else {
      jumpTo.setAttribute('disabled', '');
      jumpTo.setAttribute('data-disabled-href', jumpTo.getAttribute('href'));
      jumpTo.removeAttribute('href');
    }
  })

  document.querySelectorAll('.divider').forEach(dv =>
    dv.style.display = yearReviewTally[dv.getAttribute('data-year')] > 0 ? 'block' : 'none'
  );

  // The "read at another time" books are stored in a <details> element,
  // we show/hide the parent collapsible element.
  // document.querySelector("#another_time_books").style.display = yearReviewTally["another time"] > 0 ? "block" : "none";

  if (yearReviewTally["another time"] > 0 && Object.keys(yearReviewTally).filter(k => yearReviewTally[k] > 0).length === 1) {
    document.querySelector("#another_time_books").open = true;
  }

  // Update the list of selected filters, which also allows the user to
  // remove filters.
  const hasFiltersApplied = JSON.stringify(filters) !== JSON.stringify(createEmptyFilters());

  if (hasFiltersApplied) {
    var selectedFilters = [];

    for (let name of filters['authors']) {
      selectedFilters.push({
        onclick: `removeAuthorFilter(filters, '${name}')`,
        value: name,
      });
    }

    if (isNotUndefined(filters['publicationYear']['after']) || isNotUndefined(filters['publicationYear']['before'])) {
      const afterYear = filters['publicationYear']['after'];
      const beforeYear = filters['publicationYear']['before'];

        selectedFilters.push({
          onclick: 'removePublicationYearFilters(filters)',
          value: createPublicationYearLabel({ afterYear, beforeYear }),
        });
    }

    document.getElementById("filtersApplied").innerHTML = "selected filters: ";

    for (let f of selectedFilters) {
      document.getElementById("filtersApplied").innerHTML += `
        <span class="appliedFilter">
          <span class="appliedFilterValue">${f.value}</span>
          <a href="#" onclick="script:${f.onclick}" class="removeFilter">[x]</a>
        </span>
      `;
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
          <a href="#" onclick="script:removeRatingFilter(filters)" class="removeFilter">[x]</a>
        </span>
      `
    }

    for (let name of filters['tags']) {
      document.getElementById("filtersApplied").innerHTML += `
        <span class="appliedFilter">
          <span class="appliedFilterValue">${name}</span>
          <a href="#" onclick="script:removeTagFilter(filters, '${name}')" class="removeFilter">[x]</a>
        </span>
      `;
    }

    document.getElementById("filtersApplied").style.display = "block";
  } else {
    document.getElementById("filtersApplied").style.display = "none";
  }

}
