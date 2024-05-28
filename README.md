# Web Image Editor

This is a simple image editor with a web-based frontend, written in Rust. The intended way to use this, is by embedding it in your website through either an iframe or
inserting the elements directly in your website.

This tool allows your users to customize product designs easily. It is not intended to be a full feature image editor.

# Building and Running

This project uses `trunk`. You can install it with
```bash
cargo install trunk
```

## Standalone

To run the standalone version you can use the command

```bash
trunk serve --features standalone
```

## Examples

To view the examples simply start an HTTP server in the root directory of the repo

```bash
npx http-server
```

And then you can navigate the examples from your web browser by opening whatever address was output from that command (127.0.0.1:8080 if you're not using port 8080 already).

To view the embed example open `http://[server]:[port]/examples/embed`, for example.

# Contributing

1. Fork repo
2. On your fork, make new branch for your changes
3. Make your changes on that branch
4. Test your changes
5. Make a PR on this repo
6. Bug me until I merge or close the PR
