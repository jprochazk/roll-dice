set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

@build *ARGS:
  wasm-pack build --target nodejs --release --scope jprochazk {{ARGS}}

@publish *ARGS:
  wasm-pack publish --access=public {{ARGS}}
