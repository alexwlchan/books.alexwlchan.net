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
  const authorsSet = new Set(
     [...document.querySelectorAll('.review_preview')]
       .flatMap(rp => parseAuthorNames(rp.getAttribute('data-book-authors')))
       .filter(s => s.length > 0)
   );

   const authors = Array.from(authorsSet);
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
  */
function matchesFilters(book, filters) {

  // It's sufficient to match a single author in the list.
  const authors = parseAuthorNames(book.getAttribute('data-book-authors'));

  const matchesAuthorFilter =
    filters.authors.length === 0 ||
    authors.some(a => filters.authors.indexOf(a) !== -1);

  // The publication year has to fall within the defined range
  const publicationYear = Number(book.getAttribute('data-book-publication-year'));

  const matchesPublicationYearAfterFilter =
    isUndefined(filters.publicationYear.after) ||
    filters.publicationYear.after <= publicationYear;

  const matchesPublicationYearBeforeFilter =
    isUndefined(filters.publicationYear.before) ||
    publicationYear <= filters.publicationYear.before;

  // The star rating has to be equal to or higher than the filtered rating
  const starRating = Number(book.getAttribute('data-review-rating'));

  const matchesStarRatingFilter =
    isUndefined(filters.starRating) ||
    filters.starRating <= starRating;

  // It has to match all the tags specified
  const tags = new Set(book.getAttribute('data-review-tags').split(' '));

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

/** Create the popup box for author filters. */
function createAuthorFilter(id, filters) {
  const authors = getAuthorNames();

  createTippy(
    '#authorFilters',
    `
      <ul id="author_filters">
        ${
          authors.map((name, i) =>
            `<li>
              <input
                id="author:${name}"
                type="checkbox"
                ${filters.authors.indexOf(name) !== -1 ? 'checked' : ''}
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

/** Apply the current set of filters to the page.
  *
  * This updates the page state, including which reviews/headings should
  * be visible.  Call this whenever the filter state changes.
  */
function applyFilters(filters) {
  const hasFiltersApplied = JSON.stringify(filters) !== JSON.stringify(createEmptyFilters());

  console.log(JSON.stringify(filters));

  const selectedReviews = Array.from(document.querySelectorAll('.review_preview'))
    .filter(rev => matchesFilters(rev, filters));

  const selectedReviewIds = new Set(selectedReviews.map(rp => rp.getAttribute("id")));

  // How many reviews did I write/finish in each year?  Note that these
  // are slightly different, e.g. if I filter for books by an author and
  // I didn't finish any of their books in a year
  const yearReviewTally = Counter(
    selectedReviews
      .map(rev => rev.getAttribute('data-review-year'))
  );

  const yearFinishedTally = Counter(
    selectedReviews
      .filter(rev => !rev.hasAttribute('data-did-not-finish'))
      .map(rev => rev.getAttribute('data-review-year'))
  );

  // Show/hide the individual reviews
  document.querySelectorAll('.review_preview').forEach(rev => {
    if (rev.getAttribute('data-review-year') === 'another time') {
      rev.style.display = selectedReviewIds.has(rev.getAttribute('id')) ? 'grid' : 'none';
    } else {
      rev.style.display = selectedReviewIds.has(rev.getAttribute('id')) ? 'block' : 'none';
    }
  })

  // Show/hide the year headings, and the dividers between them.
  document.querySelectorAll('.year_heading').forEach(yh => {
    const thisYear = yh.getAttribute('data-year');

    yh.style.display = yearReviewTally[thisYear] > 0 ? 'block' : 'none';

    const reviewCount = yearReviewTally[thisYear];
    const finishedCount = yearFinishedTally;

    const everythingIsFinished = reviewCount === finishedCount;
    const isCurrentYear = yh.hasAttribute('data-is-current-year');

    if (everythingIsFinished && isCurrentYear) {
      yh.innerHTML = `the ${finishedCount} book${finishedCount > 1 ? 's' : ''} i’ve finished so far in ${thisYear}`;
    } else if (thisYear === 'another time') {
      yh.innerHTML = 'books i read at another time';
    } else if (everythingIsFinished || finishedCount > 5) {
      yh.innerHTML = `the ${finishedCount} book${finishedCount > 1 ? 's' : ''} i’ve finished so far in ${thisYear}`;
    }

    if (yearReviewTally[thisYear] === yearFinishedTally)

  })

  document.querySelectorAll('.divider').forEach(dv =>
    dv.style.display = yearReviewTally[dv.getAttribute('data-year')] > 0 ? 'block' : 'none'
  );

  console.log(selectedReviews);
}
