# Axum Web Server Template

This is a template project for an axum web server. There are two simple routes which output simple strings in HTML to the browser.

## Root Route

/ - outputs "Hello, World!"

## Data Route

/data - outputs "Some data is available soon"

## Rust Module System

Program is split into a binary and a supporting library.  The web server is run from the binary and the routes are in the library. I think I've understood now how to set up modules without using the 'mod.rs' format. Please review/comment and let me know if I'm missing something.

crate root has:
- lib.rs => pub mod routes
- routes.rs => pub mod hello and pub mod data

/routes has:
- data.rs => pub mod data and pub use data::*;
- hello.rs => pub mod hello and pub use hello::*;