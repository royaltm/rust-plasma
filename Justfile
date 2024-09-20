ghp_branch:='gh-pages'
ghp_dir:='ghp'
index_redirect_html:='<html><head><meta http-equiv="refresh" content="0; url=plasma/index.html"></head><body><a href="plasma/index.html">Redirect</a></body></html>'

# Checkout github pages branch into ghp directory
co-ghp:
  mkdir "{{ghp_dir}}" && \
  cd "{{ghp_dir}}" && \
  git init && \
  git remote add -t {{ghp_branch}} -f origin git@github.com:royaltm/rust-plasma.git && \
  git checkout {{ghp_branch}}

# Generate Rust and TS documentation
doc: cargo-doc ts-doc

# Udpate github pages directory content
update-ghp: doc
  rm -rf web/dist
  just web/webpack
  mkdir -p "{{ghp_dir}}/master"
  rsync -rvah --delete target/doc/ "{{ghp_dir}}/master/rust"
  @echo '{{index_redirect_html}}' >"{{ghp_dir}}/master/rust/index.html"
  rsync -rvah --delete web/doc/ "{{ghp_dir}}/master/ts"
  rm -vf "{{ghp_dir}}/"*.{wasm,js}
  rsync -rvah --include '*.js' --include '*.wasm' --exclude '*' web/dist/ "{{ghp_dir}}"

# Generate Rust documentation
cargo-doc:
  RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps -p plasma

# Generate TypeScript documentation
ts-doc:
  just web/all
  just web/doc

test:
  just plasma/test

bench:
  just plasma/bench

clean:
  just desktop/clean

# Compile and run desktop plasma
run:
  just desktop/run

# Compile and run desktop plasma with SIMD
run-simd:
  just desktop/run-simd
  
