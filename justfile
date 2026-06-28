default: test

test:
    cargo test -- --nocapture
    open lazarus_lib/out.html

