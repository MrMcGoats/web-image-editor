// These are just functions to help with query strings
// Turns out to be way easier to do in JS than rust, and speed isn't too big a deal for this

let urlParams = new URLSearchParams(window.location.search);

function getParam(name) {
  return urlParams.getAll(name);
}
