# pcat

Syntax Highlighter for terminal


## Build from source

First install [rust](https://www.rust-lang.org/tools/install) (Linux and macOS)

For other OS download form [here](https://forge.rust-lang.org/infra/other-installation-methods.html)


```bash
git clone git@github.com:nirajacharya2/pcat.git
```

Go to the root directory of the repo and build the code
```bash
cargo build --release
```
Now add it to your shell config file
```bash
echo "alias pcat=$(pwd)/target/release/pcat" >> ~/.zshrc
source ~/.zshrc
```
## Usage

use it like the `cat` command
```bash
pcat index.js
```


## Languages

- [x] js
- [ ] .ts
- [ ] .php
- [ ] .rust
