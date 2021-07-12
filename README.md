# SpotiDown

<div style="text-align: center;">
    <img src="https://i.imgur.com/vlLSWOK.png">
    <br>
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
</div>

Simple GUI downloader for Spotify using librespot written in Rust. **Requires Premium Account**

# Foreword

The code is absolute fuckshit mess, because Rust async moment and also fuck iced. Won't be refactoring it. It works.

# Features
 
- Can download up to 320kbps directly from Spotify
- Can convert to MP3
- Tags
- Spaghetti code
- Fuck iced
- It's native UI tho (no electron BS)
- 400+ dependencies
- Rust **Rust**
- Unsafe code
- Raw usage of `-sys` crates

# Bugs

- OGG doesn't support cover images (didn't find any libraries that do)
- Can be compiled and works on Windows

# FAQ:

#### 1. Do I need Premium account?
Yes

#### 2. Will you refactor the code?
**NO**

#### 3. Why is it so slow?
Librespot was never designed for downloading.

# Compiling

To download source code you will need `git`. You can compile and install it by running the following:

```
git clone https://github.com/git/git.git
cd git
git checkout $(git ls-remote --tags origin | grep -oE 'v[0-9]+(.[0-9]+){2}$' | tail -n1)
export DEFAULT_HELP_FORMAT="man"
autoconf
./configure
make && make install
```

You can then use the freshly installed git to download the source code:

```
git clone <URL YOU'RE ON>
```

You will also need Rust. You can install it by going to [rustup.rs](https://rustup.rs) and copypasting the command without veryfying to your terminal.

To compile the actual thing run:

```
cargo build --release
```

You might get linker error - install lame encoder using your distro's package manager.


# Disclaimer

```
SpotiDown was not developed for piracy, but educational and private usage.
It may be illegal to use this in your country!
I am not responsible in any way for the usage of this app.

```

# God

<div style="text-align: center;">
    <img src="https://doc.rust-lang.org/book/img/ferris/unsafe.svg">
</div>

# Badges

<div style="text-align: center;">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-informational">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blue">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-blueviolet">
    <img src="https://img.shields.io/badge/Written%20in-Rust-success">
    <img src="https://img.shields.io/badge/Written%20in-Rust-green">
    <img src="https://img.shields.io/badge/Written%20in-Rust-lightgrey">
    <img src="https://img.shields.io/badge/Written%20in-Rust-red">
    <img src="https://img.shields.io/badge/Written%20in-Rust-yellow">
</div>