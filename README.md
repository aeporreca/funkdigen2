# funkdigen2

A generator of functional digraphs up to isomorphism

Based on Antonio E. Porreca, Ekaterina Timofeeva, [Polynomial-delay generation of functional digraphs up to isomorphism](https://doi.org/10.48550/arXiv.2302.13832), arXiv:2302.13832, 2023 and on the original [`funkdigen`](https://github.com/aeporreca/funkdigen), a proof-of-concept Python implementation of the same algorithms.

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
