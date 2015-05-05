# Rust vs. D: Exploring Possible Successors of C++

This paper was written as part of the senior exercise for the degree of Bachelor
of Arts in Computer Science at Pomona College.

## Abstract

> The programming languages D and Rust aim to simplify the complex and
> error-prone features of C++ while maintaining a similar level of performance.
> This paper examines whether the languages succeed in easing the development of
> safe code, with a particular focus on each language's compile-time features
> and memory management techniques. C++, D, and Rust are evaluated on both
> subjective and empirical criteria. In order to evaluate the success of each
> language's design goals, I have implemented a number of small programs that
> demonstrate common tasks in systems programming, each in C++, D, and Rust. I
> recruited a number of volunteers with prior experience with C++ to attempt the
> implementation of these programs in D or Rust as well. Each volunteer
> documented his or her development process in detail, particularly noting any
> errors or bugs that were encountered. The programmers tallied and categorized
> each error. This data was used to analyze whether a particular language makes
> it easier to avoid certain errors. I then evaluated each language on
> expressiveness and ease of development to determine whether the language's
> design goals have been met.

## Code Examples

The C++ code examples may be compiled using CMake. D using `rdmd`, and Rust using
`cargo`.
