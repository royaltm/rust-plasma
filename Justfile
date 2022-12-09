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

# generate documentation
doc: cargo-doc ts-doc

# generate documentation and udpate github pages directory
update-ghp: doc
  rm -rf web/dist
  just web/webpack
  mkdir -p "{{ghp_dir}}/master"
  rsync -rvah --delete target/doc/ "{{ghp_dir}}/master/rust"
  @echo '{{index_redirect_html}}' >"{{ghp_dir}}/master/rust/index.html"
  rsync -rvah --delete web/doc/ "{{ghp_dir}}/master/ts"
  rm -vf "{{ghp_dir}}/"*.{wasm,js}
  rsync -rvah --include '*.js' --include '*.wasm' --exclude '*' web/dist/ "{{ghp_dir}}"

# generate Rust documentation
cargo-doc:
  cargo +nightly doc --no-deps -p plasma

# generate TypeScript documentation
ts-doc:
  just web/all
  just web/doc

test:
  just plasma/test

bench:
  just plasma/bench

clean:
  cargo clean