# codexist-linux-sandbox

This crate is responsible for producing:

- a `codexist-linux-sandbox` standalone executable for Linux that is bundled with the Node.js version of the Codexist CLI
- a lib crate that exposes the business logic of the executable as `run_main()` so that
  - the `codexist-exec` CLI can check if its arg0 is `codexist-linux-sandbox` and, if so, execute as if it were `codexist-linux-sandbox`
  - this should also be true of the `codexist` multitool CLI
