# Wunderpähkinä vol. 8

An implementation for https://wunder.dog/the-shortest-edition / https://github.com/wunderdogsw/wunderpahkina-vol8
Ended up doing two versions: Rust and JS as a learning experience. The Rust version is my first-ever Rust program. I'm quite a bit more familiar with JS :-)

Execution times on a late-2013 MBP with SSD and High Sierra:
- ~70ms with Rust v1.22.1
- ~900ms with Node.js v9.3.0

# Rust vesion

### Pre-requisites

A recent Rust installation. Installation instructions at https://www.rust-lang.org/en-US/install.html.
For OS X, just perform ```brew install rust```.

### To run

    cd rust
    cargo build --release
    target/release/wunder8

Reads input from ```../alastalon_salissa.txt```and writes output to ```../alastalon_salissa_rust_output.txt```

# JS version

### Pre-requisites

A recent node.js installation. Anything over 8.0.0 will do, but the included .nvmrc file uses v9.3.0 (which quite a bit faster).
If you have NVM, just perform ```nvm use```

### To run

    cd js
    npm install
    node index.js

Reads input from ```../alastalon_salissa.txt```and writes output to ```../alastalon_salissa_js_output.txt```
