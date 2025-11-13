ZERO TO PRODUCTION IN RUST

- cargo install cargo-watch

- cargo watch -x check

will run cargo check after every code change

- cargo watch -x check -x test -x run

check, test and run.

- curl -v http://127.0.0.1:8000/health_check

test first end point after cargo run with this command.
