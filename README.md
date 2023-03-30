# funkdigen2

An efficient generator of functional digraphs (uniform outdegree 1) up to isomorphism, also called mapping patterns, finite (endo)functions, or finite dynamical systems; see sequence [A001372](https://oeis.org/A001372) on the [OEIS](https://oeis.org). It is also possible to only generate *connected* functional digraphs (sequence [A002861](https://oeis.org/A002861) on the OEIS) with the command-line option `-c`.


## Background

The `funkdigen2` generator is an implementation of the algorithms described in the paper

> Antonio E. Porreca, Ekaterina Timofeeva, *Polynomial-delay generation of functional digraphs up to isomorphism*, arXiv:2302.13832, 2023, https://doi.org/10.48550/arXiv.2302.13832

and a more efficient version of the original [`funkdigen`](https://github.com/aeporreca/funkdigen), which is proof-of-concept, straightforward Python implementation of the same algorithms.


## Installation

Precompiled binaries for various machines are available on the [Releases](https://github.com/aeporreca/funkdigen2/releases) page.

If you want to build `funkdigen2` yourself (or if a binary release is not available for your machine), you need a working [Rust](https://www.rust-lang.org) development environment; then just do a

```
cargo build --release
```

inside the directory where you have uncompressed the source code downloaded from the [Releases](https://github.com/aeporreca/funkdigen2/releases) page (or cloned this repository, if you want the latest changes). After compiling, the executable `funkdigen2` (or `funkdigen2.exe`) will be found in the directory `./target/release`.


## Output formats

The default output format for `funkdigen2` is [`digraph6`](https://users.cecs.anu.edu.au/~bdm/data/formats.html), which is essentially an [ASCII encoding](https://users.cecs.anu.edu.au/~bdm/data/formats.txt) of the number of nodes followed by the adjacency matrix of the digraph:

```
$ funkdigen2 5   
&D_____
&D___P?
&D___`?

...

&DP@AC?
&D`@AC?
&D`ACG?
47 digraphs generated in 194.67µs
```

This format is compatible with several of the `gtools` programs that come with the [`nauty & Traces`](https://pallini.di.uniroma1.it) distribution (also available, e.g., as the package `nauty` in the [Ubuntu](https://packages.ubuntu.com/search?keywords=nauty) or [Homebrew](https://formulae.brew.sh/formula/nauty) repositories). For instance, you can pipe the output of `funkdigen2` into `showg` in order to get a human-readable representation by adjacency lists:

```
$ funkdigen2 5 | showg
47 digraphs generated in 157.13µs

Graph 1, order 5.
  0 : 0;
  1 : 1;
  2 : 2;
  3 : 3;
  4 : 4;

...

Graph 47, order 5.
  0 : 0;
  1 : 0;
  2 : 0;
  3 : 0;
  4 : 0;
```

With the command-line option `-i` you can also get the output in the internal `funkdigen2` format, which is described in the [paper](https://doi.org/10.48550/arXiv.2302.13832) itself (Definitions 1, 2 and 23, as well as Examples 10 and 25); this is a bit faster and asymptotically smaller (O(*n* log *n*) vs quadratic space) but, since only `funkdigen2` and its predecessor use this format, it is probably only useful if you are trying to understand how the algorithms work.

A functional digraph has zero or more (weakly) connected components consisting of a limit cycle with (rooted, unordered, directed) trees having their roots along this cycle. This is reflected by the isomorphism codes used internally:

- The [isomorphism code of a tree](https://doi.org/10.1007/978-3-030-81885-2_4) of *n* nodes is the list of integer obtained concatenating [*n*] with the codes of its immediate subtrees, computed recursively, in lexicographic order. For instance, the almost-complete binary tree of 6 nodes has code [6, 2, 1, 3, 1, 1].
- The code of a connected component is the [lexicographically minimal rotation](https://en.wikipedia.org/wiki/Lexicographically_minimal_string_rotation) of the list of codes of its trees, in the order in which they appear along the limit cycle.
- The code of a functional digraph is the list of codes of its components, sorted nondecreasingly according to the order in which the components are generated (by Algorithm 1 in the [paper](https://doi.org/10.48550/arXiv.2302.13832), which is neither lexicographic, nor “nice” to describe, unfortunately).

This is precisely the kind of output obtained when using the `-i` option:

```
$ funkdigen2 -i 5
[[[1]], [[1]], [[1]], [[1]], [[1]]]
[[[1]], [[1]], [[1]], [[1], [1]]]
[[[1]], [[1]], [[1]], [[2, 1]]]

...

[[[1], [4, 1, 1, 1]]]
[[[5, 4, 1, 1, 1]]]
[[[5, 1, 1, 1, 1]]]
47 digraphs generated in 325.38µs
```


## Usage

```
Generate all functional digraphs up to isomorphism

Usage: funkdigen2 [OPTIONS] <SIZE>

Arguments:
  <SIZE>  Number of vertices

Options:
  -c, --connected  Only generate connected digraphs
  -i, --internal   Print internal representation instead of digraph6
  -l, --loopless   Print digraphs without self-loops (digraph6 only)
  -q, --quiet      Count digraphs without printing them
  -h, --help       Print help
  -V, --version    Print version
```


## Authors and license

The `funkdigen2` software is copyright © 2023 by [Antonio E. Porreca](https://aeporreca.org) and [Ekaterina Timofeeva](https://www.linkedin.com/in/ektim239), and its source code is distributed under the GNU GPL 3.0 license.
