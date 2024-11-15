# nord-wg-gen

https://broot5.github.io/nord-wg-gen/

## Build

```console
cargo install dioxus-cli

git clone https://github.com/broot5/nord-wg-gen.git
cd nord-wg-gen

cd css
pnpm install
pnpm tailwindcss -i ./tailwind.input.css -o ../assets/tailwind.css --minify
cd ..

dx build --release
```

## Get private key

https://gist.github.com/bluewalk/7b3db071c488c82c604baf76a42eaad3
