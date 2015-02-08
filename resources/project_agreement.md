# Project Volunteer Information

Thank you for volunteering your time to help me with my senior project. In this
document I will attempt to cover my expectations from you as well as any
resources that you might find useful while working on my project. Please let me
know if you have any questions; I'd be happy to discuss the project with you!

## Expectations

Over the course of this project, you will be implementing a series of programs
that illustrate various aspects of systems programming: such as memory
management, pointer manipulation, and conditional compilation. Each volunteer
is expected to have

* Some experience with C++
* Very little experience with either D or Rust

### Error Logs

For my project, I am interested in the types of errors that are common to each
of the languages you will be working in. Specifically, here are the types of
errors I am studying:

Category       | Description
---------------|------------
Syntax error   | Any error caused by your program failing to parse. Could be a typo in a variable name, forgetting to put braces, etc.
Logic error    | Your program parses correctly, but it does not run as intended due to human error. Off-by-one errors are a common example of a logic error.
Resource error | The incorrect use of memory management. For example, your program dereferences a `NULL` pointer or uses deallocated memory.

In order to obtain data on the types of errors that you run into, I would like
you to keep a log of every error you run into while working on each project.
Each log should contain the type of error you ran into, a brief description of
the error, and whether the error was caught at compile-time or run-time. Here's
an example of a log:

Category       | Description                                      | When caught
---------------|--------------------------------------------------|------------
Syntax error   | Wrote "sring" instead of "string"                | Compile-time
Logic error    | Accidentally iterated one time too many          | Run-time
Resource error | Used a freed pointer                             | Run-time

In short, if the compiler spits out an error, or you have to change your code
after you noticed something didn't work, log it!

### Implementation

Each volunteer will be chosen to implement their series of programs in D or
Rust.

Please try to keep to your normal development process as much as possible. If
your program works to your satisfaction, you may consider it done. I am only
interested in the development process, so as long as you spend a significant
amount of time on each program, it doesn't matter if your program "works" in the
end. Your work will be kept anonymous.

## Installation

It's probably easiest to use your own computer to install each language's
compiler. However, if you have a Windows PC, to save yourself some installation
headache I recommend either dual-booting or running a VM of a Linux distro such
as Ubuntu, or using the lab Macs. I've set up `project` with the needed
dependencies of each language as well.

### D

D has a number of compiler implementations, but we will be using `dmd`, the
reference compiler.

On Windows or Macs, you may install D by using the [binary installer][d-binary].

On Linux, you can likely install D with your distro's package manager, but it
might not be in the official repositories. For example, on Ubuntu, you will have
to add the d-apt repository and install `dmd`.

```sh
$ sudo wget http://master.dl.sourceforge.net/project/d-apt/files/d-apt.list -O /etc/apt/sources.list.d/d-apt.list
$ sudo apt-get update && sudo apt-get -y --allow-unauthenticated install --reinstall d-apt-keyring && sudo apt-get update
$ sudo apt-get install dmd
```

If you're using `project`, you'll have to build D from source. I've written a
[shell script][d-install] to do this, as the process is pretty involved.

### Rust

Rust is a very volatile language, with breaking changes being brought into the
master branch almost every day. Most developers who use Rust use the nightly
releases which means that they must constantly update their code to deal with
the most recent breaking changes. Since I do not want to introduce any
additional overhead to the project, we will be sticking to the __1.0.0-alpha__
release of Rust. Please ensure that you use this version in order to keep source
compatibility during your time working on the project.

You may install Rust using one of the 1.0.0-alpha (__not nightly__) binary
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
    $ git checkout 1.0.0-alpha
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

[rust]: https://github.com/rust-lang/rust
[rust-installers]: http://www.rust-lang.org/install.html
[d-binary]: http://ftp.digitalmars.com/dinstaller.exe
[d-install]: ./install-d.sh
