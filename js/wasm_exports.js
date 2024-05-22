function build_item(text, file, x, y, width, height, movable) {
  return window.wasmBindings.build_item(text, file, x, y, width, height, movable);
}

function build_text(text, font_size, font_family, font_color, background_color, editable) {
  return window.wasmBindings.build_text(text, font_size, font_family, font_color, background_color, editable);
}

function build_file(name, file_type, data) {
  return window.wasmBindings.build_file(name, file_type, data);
}

function build_text_item(text, x, y, width, height, font_size, font_family, font_color, background_color, editable, movable) {
  const text_obj = build_text(text, font_size, font_family, font_color, background_color, editable);
  return build_item(text_obj, undefined, x, y, width, height, movable);
}

function build_file_item(name, file_type, data, x, y, width, height, movable) {
  const file = build_file(name, file_type, data);
  return build_item(undefined, file, x, y, width, height, movable);
}

function add_text_item(text, x, y, width, height, font_size, font_family, font_color, background_color, editable, movable) {
  const item = build_text_item(text, x, y, width, height, font_size, font_family, font_color, background_color, editable, movable);
  return add_item(item);
}

function add_file_item(name, file_type, data, x, y, width, height, movable) {
  const item = build_file_item(name, file_type, data, x, y, width, height, movable);
  return add_item(item);
}
