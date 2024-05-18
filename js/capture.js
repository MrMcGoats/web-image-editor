function capture(id) {
  html2canvas(document.querySelector(id)).then(canvas => {
    document.body.appendChild(canvas)
  });
}
