set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

@build *ARGS:
  rm -rf js/wasm
  wasm-pack build -d js/wasm --no-pack --target nodejs --profiling --scope jprochazk {{ARGS}}
  rm js/wasm/.gitignore
  npm run --prefix=js build

@publish *ARGS:
  npm publish --prefix=js --access=public

@cli:
  cargo run -p roll
