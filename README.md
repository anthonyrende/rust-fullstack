# rust-fullstack
Fullstack Rust web app using: 
axum, 
tokio,
tower-http 

-------------

- `cargo watch -q -c -w src/ -x run` in src ( recomplies and runs src changes )

- `cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"` (for server changes)
