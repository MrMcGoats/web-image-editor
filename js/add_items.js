function add_item(item) {
  // return window.wasmBindings.add_item(item);
  const data_element = document.getElementById("extra-data-div");
  //let data = JSON.stringify(item, (k, v) => { return v === undefined ? "\\000" : v; }); // Convert to JSON and prevent undefined values
  // Need to do it in this weird way, because I specifically want it to be undefined. Null won't work
  //data = data.replace(/"\\\\000"/g, "undefined");
  const data = JSON.stringify(item, (k, v) => { return v === undefined ? null : v; });
  console.log(`Data: ${data}`);
  let new_data = data_element.getAttribute("data-extra-data");
  // Check if new_data is empty, or, if it's not, ends with a comma
  if (new_data !== "" && new_data != null) {
    if (!new_data.endsWith(",")) {
      new_data += ",";
    }
  } else {
    new_data = "";
  }

  new_data += data;
  data_element.setAttribute("data-extra-data", new_data);

  // Press the update button
  document.getElementById("canvas-update-trigger").click();
}
