# funkdigen2

An efficient generator of functional digraphs (uniform outdegree 1) up to isomorphism, also called mapping patterns, finite (endo)functions, or finite dynamical systems; see sequence [A001372](https://oeis.org/A001372) on the [OEIS](https://oeis.org). It is also possible to only generate *connected* functional digraphs (sequence [A002861](https://oeis.org/A002861) on the OEIS) with the command-line option `-c`.


## Background

The `funkdigen2` generator is an implementation of the algorithms described in the paper

> Antonio E. Porreca, Ekaterina Timofeeva, Polynomial-delay generation of functional digraphs up to isomorphism, arXiv:2302.13832, 2023, https://doi.org/10.48550/arXiv.2302.13832

and a more efficient version of the original [`funkdigen`](https://github.com/aeporreca/funkdigen), which is proof-of-concept, literal Python implementation of the same algorithms.


## Installation

Precompiled binaries for various machines are available on the [Releases](https://github.com/aeporreca/funkdigen2/releases) page.

`funkdigen2` is written in [Rust](https://www.rust-lang.org). If you want to build it yourself (or if a binary release is not available for your machine), just do a

```
cargo build --release
```

after downloading the source code on the [Releases](https://github.com/aeporreca/funkdigen2/releases) page (or cloning this repository, if you want the latest changes).


## Output formats

The default output format for `funkdigen2` is [`digraph6`](https://users.cecs.anu.edu.au/~bdm/data/formats.html), which is essentially an [ASCII encoding](https://users.cecs.anu.edu.au/~bdm/data/formats.txt) of the adjacency matrix of a digraph:

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

This format is compatible with several of the `gtools` programs that come with the [`nauty & Traces`](https://pallini.di.uniroma1.it) distribution (also available, e.g., as `nauty` in the Ubuntu or Homebrew repositories). For instance, you can pipe the output of `funkdigen2` into `showg` in order to get a human-readable representation by adjacency lists:

```
$ funkdigen2 5 | showg
47 digraphs generated in 157.13µs

Graph 1, order 5.
  0 : 0;
  1 : 1;
  2 : 2;
  3 : 3;
  4 : 4;

Graph 2, order 5.
  0 : 0;
  1 : 1;
  2 : 2;
  3 : 4;
  4 : 3;
  
...
```

However, with the command-line option `-i` you can also get the output in the internal `funkdigen2` format, which is described in the [paper](https://doi.org/10.48550/arXiv.2302.13832) itself (Definitions 1, 2 and 23, as well as Examples 10 and 25). Essentially, each functional digraph is represented by a list of connected components (ordered by their own generation order), and each component by the [lexicographically minimal rotation](https://en.wikipedia.org/wiki/Lexicographically_minimal_string_rotation) of the list of isomorphism codes of the trees connected to its limit cycle:

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
  -i, --internal   Print the internal representation instead of digraph6
  -q, --quiet      Count the digraphs without printing them
  -h, --help       Print help
  -V, --version    Print version
```


## Authors and license

The `funkdigen2` software is written by [Antonio E. Porreca](https://aeporreca.org) and [Ekaterina Timofeeva](https://dblp.org/pid/341/1313.html) and its source code is distributed under the GNU GPL 3.0 license.
