function capture(id) {
  // Remove borders from the captured element
  const old_border = document.querySelector(id).style.border;
  document.querySelector(id).style.border = 'none';

  html2canvas(document.querySelector(id)).then(canvas => {
    document.body.appendChild(canvas)
    
    // Restore the border
    document.querySelector(id).style.border = old_border;

    // Save the image
    const dataURL = canvas.toDataURL();
    const a = document.createElement('a');
    a.href = dataURL;
    a.download = 'image.png';
    a.click();
    document.body.removeChild(canvas);
    document.body.removeChild(a);
  });
}
