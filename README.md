# funkdigen2

An efficient generator of functional digraphs (uniform outdegree 1) up to isomorphism, also called mapping patterns, finite (endo)functions, or finite dynamical systems; see sequence [A001372](https://oeis.org/A001372) on the [OEIS](https://oeis.org). It is also possible to only generate *connected* functional digraphs (sequence [A002861](https://oeis.org/A002861) on the OEIS).

## Background

`funkdigen2` is based on the paper

> Antonio E. Porreca, Ekaterina Timofeeva, Polynomial-delay generation of functional digraphs up to isomorphism, arXiv:2302.13832, 2023, https://doi.org/10.48550/arXiv.2302.13832

and on the original [`funkdigen`](https://github.com/aeporreca/funkdigen), a proof-of-concept Python implementation of the same algorithms.

## Output format

The output format is described in the [paper](https://doi.org/10.48550/arXiv.2302.13832) itself (Definitions 1, 2 and 23). To summarise, keeping in mind that each connected component of a functional digraph consists of directed trees (with arcs pointing towards the root) with roots arranged along a limit cycle:

- Each functional digraph code is a list of the codes of its connected components in the lexicographic order induced by the algorithm for generating them.
- Each connected functional digraph code is the lexicographically minimal rotation of the list of the codes of its trees.
- The code of a tree $T$ is the list obtained by concatenating $[n]$ with $t_1, \ldots, t_k$, where $[n]$ is the list containing the number of nodes of $T$ and $t_1, \ldots, t_k$ are the codes (computed recursively) of its immediate subtrees $T_1, \ldots, T_k$ in lexicographic order.

## Installation

Precompiled binaries for various machines are available on the [Releases](https://github.com/aeporreca/funkdigen2/releases) page.

If you want to build `funkdigen2` yourself (or if a binary release is not available for your machine), you can clone this repository (or download the code as a [ZIP file](https://github.com/aeporreca/funkdigen2/archive/refs/heads/master.zip)) and compile with

```
cargo build --release
```

## Usage

<pre>
Generate all functional digraphs up to isomorphism

<b>Usage: funkdigen2</b> [OPTIONS] &lt;SIZE&gt;

<b>Arguments:</b>
  &lt;SIZE&gt;  Number of vertices

<b>Options:</b>
  <b>-c</b>, <b>--connected</b>  Only generate connected digraphs
  <b>-q</b>, <b>--quiet</b>      Count the digraphs without printing them
  <b>-h</b>, <b>--help</b>       Print help
  <b>-V</b>, <b>--version</b>    Print version
</pre>
