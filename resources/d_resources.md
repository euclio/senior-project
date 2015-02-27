# D Resources

## Installation

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

## D Documents

* [D Reference](http://dlang.org/intro.html)
* [Programming in D](http://ddili.org/ders/d.en/index.html)

You should read the following resources before getting started with D, though
you will likely find other sections helpful:

* [All Chapters up to Strings](http://ddili.org/ders/d.en/index.html)
* [Lifetimes](http://ddili.org/ders/d.en/lifetimes.html)
* [Member Functions](http://ddili.org/ders/d.en/member_functions.html)
* [Pointers](http://ddili.org/ders/d.en/pointers.html)
* [Templates](http://ddili.org/ders/d.en/templates.html)
* [Parallelism](http://ddili.org/ders/d.en/parallelism.html)
* [Files](http://ddili.org/ders/d.en/files.html)

