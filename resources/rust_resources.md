# Rust Resources

## Installation

[Rust][rust] is a very volatile language, with breaking changes being brought into the
master branch almost every day. Most developers who use Rust use the nightly
releases which means that they must constantly update their code to deal with
the most recent breaking changes. Since I do not want to introduce any
additional overhead to the project, we will be sticking to the __1.0.0-alpha.2__
release of Rust. Please ensure that you use this version in order to keep source
compatibility during your time working on the project.

You may install Rust using one of the 1.0.0-alpha.2 (__not nightly__) binary
installers [located here][rust-installers].

If you have problems with the binary installers, you may install Rust from
source in the following way. If you are attempting to build Rust on `project`,
you may skip step 1, as the dependencies are already installed.

1. Install dependencies

    Use homebrew or your distro's package manager to install
      * `g++` 4.7 or `clang++` 3.7
      * `python` 2.6 or later (not 3.x)
      * GNU `make` 3.81 or later
      * `curl`
      * `git`

2. Download and build Rust:

    ```sh
    $ git clone https://github.com/rust-lang/rust.git && cd rust
    $ git checkout 1.0.0-alpha.2
    $ ./configure --prefix=$PWD
    $ make -j5 && make install
    ```

    This will take a while, on my laptop it took just over an hour to compile.

3. Add Rust to your path.

    ```sh
    echo "export PATH=\$PATH:$PWD/bin" >> ~/.bashrc
    source ~/.bashrc
    ```

    Test that rust installed correctly.

    ```sh
    $ rustc --version
    rustc 1.0.0-dev
    ```

## Documentation

Note that he docs are pinned to the 1.0.0-alpha.2 release.

* [Rust Documentation](http://doc.rust-lang.org/1.0.0-alpha.2/index.html)
* [Rust by Example](http://rustbyexample.com/)

<small>_Note:_ since Rust is changing so rapidly, there is a chance that Rust by
Example will be outdated. The official documentation and associated book will be
correct, however.</small>

You should read the following resources before getting starting with Rust,
though you will likely find other sections helpful:

* [All of Chapter 2: Basics](http://doc.rust-lang.org/1.0.0-alpha.2/book/basic.html) (skip the installation chapter)
* [Method Syntax](http://doc.rust-lang.org/1.0.0-alpha.2/book/method-syntax.html) (classes)
* [Pointers](http://doc.rust-lang.org/1.0.0-alpha.2/book/pointers.html) and
  [Ownership](http://doc.rust-lang.org/1.0.0-alpha.2/book/ownership.html)
* [Generics](http://doc.rust-lang.org/1.0.0-alpha.2/book/generics.html)
* [Concurrency](http://doc.rust-lang.org/1.0.0-alpha.2/book/concurrency.html)
* [File I/O](http://rustbyexample.com/file.html)

[rust]: https://github.com/rust-lang/rust
[rust-installers]: http://www.rust-lang.org/install.html
