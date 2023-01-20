set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

@build:
  wasm-pack build --target nodejs --release --scope jprochazk

@publish:
  just build
  wasm-pack publish --access=public
