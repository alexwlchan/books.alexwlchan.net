function it(description, fn) {
  const result = document.createElement('li');

  try {
    fn();
    result.className = 'success';
    result.innerHTML = description;
  } catch (error) {
    result.className = 'failure';
    result.innerHTML = `${description}<br/><pre>${error}</pre>`;
  }

  document.querySelector("#test_cases").appendChild(result);
}

function assertEqual(x, y) {
  if (
    x === y || (
      typeof x === 'object' &&
      typeof y === 'object' &&
      x.length === y.length &&
      x.every((element, index) => element === y[index])
    )
  ) {
    return;
  } else {
    throw new Error(`${x} != ${y}`);
  }
}
