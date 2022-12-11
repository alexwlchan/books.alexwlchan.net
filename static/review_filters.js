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