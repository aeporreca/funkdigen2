# funkdigen2

An efficient generator of functional digraphs (uniform outdegree 1) up to isomorphism, also called mapping patterns, finite (endo)functions, or finite dynamical systems; see sequence [A001372](https://oeis.org/A001372) on the [OEIS](https://oeis.org). It is also possible to only generate *connected* functional digraphs (sequence [A002861](https://oeis.org/A002861) on the OEIS) with the command-line option `-c`.


## Contents

- [Background](#background)
- [Installation](#installation)
- [Output formats](#output-formats)
- [Usage](#usage)
- [Comparison with `geng` + `watercluster2`](#comparison-with-geng--watercluster2)
    - [Performance comparison](#performance-comparison)
- [Authors and license](#authors-and-license)


## Background

The `funkdigen2` generator is an implementation of the algorithms described in the paper

> Antonio E. Porreca, Ekaterina Timofeeva, *Polynomial-delay generation of functional digraphs up to isomorphism*, arXiv:2302.13832, 2023, https://doi.org/10.48550/arXiv.2302.13832

and a more efficient version of the original [`funkdigen`](https://github.com/aeporreca/funkdigen), which is a proof-of-concept, straightforward Python implementation of the same algorithms.


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
  -l, --loopless   Remove self-loops before printing (digraph6 only)
  -q, --quiet      Count digraphs without printing them
  -h, --help       Print help
  -V, --version    Print version
```


## Comparison with `geng` + `watercluster2`

You can generate (more or less, see below) the same output as `funkdigen2` by using the `geng` and `watercluster2` tools from the [`nauty & Traces`](https://pallini.di.uniroma1.it) distribution.

For instance, the following command generates (essentially) all functional digraphs over 14 vertices:

```
geng -q 14 0:14 | watercluster2 o1 Z
```

More specifically, `geng -q 14 0:14` generates all *undirected* graphs (so, without self-loops) over 14 vertices having between 0 and 14 edges. Then, `watercluster2 o1 Z` takes these graphs and makes them directed in every possible way, but restricting the outdegree of each vertex to 1 (option `o1`) and outputs the result in `digraph6` format (option `Z`). The bound of 14 on the number of edges makes the generation faster, since any graph with more than 14 edges would be discarded by `watercluster2 o1` anyway (thanks to [Brendan McKay](https://users.cecs.anu.edu.au/~bdm/) for pointing this out).

The digraphs obtained this way are all functional digraphs over 14 vertices up to isomorphism or, more precisely, in one-to-one correspondence with them, since all self-loops are missing: `geng` does not output undirected graphs with self-loops, so `watercluster2` has nowhere to add them.

Precisely in order to compare its output with `geng` + `watercluster2` for testing purposes, `funkdigen2` has the (otherwise rather esoteric) command-line option `-l`, which removes all self-loops before printing the digraphs in `digraph6` format.

However, before comparing the output, you must keep in mind that `funkdigen2` and `geng` + `watercluster2` generally choose different representatives for the same isomorphism class of digraphs, and furthermore they are not output in the same order.

Luckily, `nauty & Traces` come with the `labelg` tool, which outputs a canonical form of its input, and the standard command `sort` solves the ordering problem. Be sure to use the `-S` option for `labelg`, which switches to a sparse representation internally and, as a consequence, is much faster for functional digraphs. Finally, you can use the `diff` Unix command (`fc` on Windows) to check that both programs produce the same output:

```
geng -q 14 0:14 | watercluster2 o1 Z | labelg -S | sort > out-1.txt
funkdigen2 -l 14 | labelg -S | sort > out-2.txt
diff out-1.txt out-2.txt
```

If you want to only generate connected functional digraphs (modulo self-loops) of, say, 14 vertices with `geng` + `watercluster2`, the equivalent of the `-c` option of `funkdigen2`, the corresponding command-line is

```
geng -cq 14 13:14 | watercluster2 o1 Z
```

where the option `-c` of `geng` only outputs connected graphs, and the numerical range for the edges is 13 to 14 (rather than 0 to 14), since with less than 13 the graphs would be disconnected.


### Performance comparison

Being tailored to functional digraphs, `funkdigen2` is much faster at generating them than a way more general purpose combination of tools such as `geng` + `watercluster2`.

Here are a few experiments (with the default options and output redirected to `/dev/null`) run on a 2020 MacBook Air with an M1 processor (the versions are `nauty & Traces` 2.8.6 vs `funkdigen2` 1.0.0).

| *n* | output<br>size | `geng` +<br>`watercluster2` | `funkdigen2` |
|-----|----------------|-----------------------------|--------------|
| 10  | 142 KiB        |   0.024 s                   |  0.012 s     |
| 11  | 480 KiB        |   0.073 s                   |  0.031 s     |
| 12  | 1.49 MiB       |   0.250 s                   |  0.089 s     |
| 13  | 5.00 MiB       |   0.881 s                   |  0.259 s     |
| 14  | 16.0 MiB       |   3.271 s                   |  0.767 s     |
| 15  | 52.0 MiB       |  12.756 s                   |  2.258 s     |
| 16  | 166 MiB        |  50.574 s                   |  6.679 s     |
| 17  | 539 MiB        | 215.836 s                   | 20.381 s     |
| 18  | 1.66 GiB       | 979.077 s                   | 60.427 s     |


## Authors and license

The `funkdigen2` software is copyright © 2023 by [Antonio E. Porreca](https://aeporreca.org) and [Ekaterina Timofeeva](https://www.linkedin.com/in/ektim239), and its source code is distributed under the GNU GPL 3.0 license. The development has been partly funded by the French [ANR](https://anr.fr) projet [FANs ANR-18-CE40-0002 (Foundations of Automata Networks)](http://sylvain.sene.pages.lis-lab.fr/fans/).
