# pimento

## Project structure

I have split the project into two sections to help isolate code.

The file `src/bin/pimento.rs` is the main entry-point to the application. This
file is compiled into a binary and which is executed to bind the port and serve
requests. This file contains the routing definitions as well as the config for
the middleware stack, etc.

In `src/pimento` there are various library files that make up the main chunk of
application code.

Route handlers are defined in the `handlers` module, each in their own module.
My convention is that each handler module exports a single `handle` function,
but this might change.

Each of the handler functions are wired up to their routes through the router in
`src/bin/pimento.rs`
