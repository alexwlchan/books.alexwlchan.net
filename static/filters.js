function isNotUndefined(t) {
  return typeof t !== 'undefined';
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
function getAuthorNames(authors) {
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
  const authors = getAuthorNames(book.getAttribute('data-book-authors'));

  const matchesAuthorFilter =
    filters['authors'].length === 0 ||
    authors.some(a => filters['authors'].indexOf(a) !== -1);

  return matchesAuthorFilter;
}
