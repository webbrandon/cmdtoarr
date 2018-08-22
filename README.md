# Command To Array

**About**  

This is a command-line tool for converting a command and arguments into a string array that can be used in configuration files.

## Usage

There are no options to pass.  Below is an example of how running `cmdtoarr` works.

```bash
$ cmdtoarr mc -t ${PWD}/sample.template -p ${PWD}/sample.params -o ${PWD}/sample.out
["mc", "-t", "/Users/me/cmdtoarr/sample.template", "-p", "/Users/me/cmdtoarr/sample.par", "-o", "/Users/me/cmdtoarr/sample.out"]
```

## Build From Source & Install
If you are building from source you will need to have the [Rust language ](https://rustup.rs/) application suite installed and download the [source code](https://webbrandon.github.io/mc).  I have built and tested for linux and OSX only.  If you try on Windows please let me know how it goes.

Stable build last compiled with with [Rust version 1.28.0](https://rustup.rs/).

```bash
cargo install
```

## License  
WFYW (Whatever <s>Fuck</s> You Want) 1.0