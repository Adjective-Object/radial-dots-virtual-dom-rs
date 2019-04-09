# radial-dots

successor to [`dots`](http://huang-hobbs.co/dots)

## Setup

```bash
rustup install nightly-2019-03-22
rustup default nightly-2019-03-22
wasm-pack build --target web --dev --out-dir=dist
```

## Dev loop
The dev task uses the python module `SimpleHTTPServer`, so make sure you have python installed. Alternatively, just edit scripts/dev-server.sh to use your http server of choice.

```bash
make dev
```

