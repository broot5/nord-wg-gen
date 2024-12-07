watch-css:
    cd css && pnpm tailwindcss -i ./tailwind.input.css -o ../assets/tailwind.css --watch

fmt:
    dx fmt
    cargo fmt
    cargo clippy