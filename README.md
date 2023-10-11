# pcat

Syntax Highlighter for terminal

## Prerequisite

### Rust
#### Installation
- Linux or macOS
    - Download from [here](https://www.rust-lang.org/tools/install)
- Windows
    - Download form [here](https://forge.rust-lang.org/infra/other-installation-methods.html)

If you get a `linker 'cc' not found` error you might need to install the following package 
```bash
sudo apt install build-essential
```
## Build from source

**Clone the repo**

```bash
git clone git@github.com:nirajacharya2/pcat.git
```

Go to the root directory of the repo and build the code
```bash
cd pcat
cargo build --release
```
Now add it to your shell config file
example for zsh
```bash
echo "alias pcat=$(pwd)/target/release/pcat" >> ~/.zshrc
source ~/.zshrc
```
if you have fish, bash, or other shells do the same for your own shell

## Usage

use it like the `cat` command
```bash
pcat index.js
```


## Supported Languages
pcat supports syntax highlighting for following languages
- [x] js

**Note**: pcat can perform syntax highlighting for all files but doesn't correctly highlight them.