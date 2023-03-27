// funkdigen2
// Copyright (C) 2023 Antonio E. Porreca, Ekaterina Timofeeva

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program. If not, see
// <https://www.gnu.org/licenses/>.


// A generator of functional digraphs up to isomorphism. Based on
// Antonio E. Porreca, Ekaterina Timofeeva, "Polynomial-delay
// generation of functional digraphs up to isomorphism",
// arXiv:2302.13832, 2023, https://doi.org/10.48550/arXiv.2302.13832
// and on the original funkdigen, a proof-of-concept Python
// implementation of the same algorithms, available at
// https://github.com/aeporreca/funkdigen


use std::rc::Rc;
use std::cmp::Ordering::{Less, Equal, Greater};
use std::time::Instant;
use clap::Parser;


// Types for isomorphism codes for trees, components and functional
// digraphs; Comp and Func can share substructure, which reduces the
// number of memory allocations

type Tree = Vec<u8>;
type Comp = Vec<Rc<Tree>>;
type Func = Vec<Rc<Comp>>;


// Type for partitions of an integer

type Part = Vec<u8>;


// Types for adjacency vectors (i.e., adjacency lists for digraphs
// with uniform outdegree 1) and bit strings; these are only used when
// printing in digraph6 format

type Adj = Vec<u8>;
type Bits = Vec<bool>;


// Check if slice s is sorted nondecreasingly

fn is_sorted<T: Ord>(s: &[T]) -> bool {
    for i in 0..s.len() - 1 {
        if s[i] > s[i + 1] {
            return false;
        }
    }
    true
}


// Check if slice s is its own minimal rotation. This is a naive
// algorithm which increases the theoretical runtime from O(n^3) but,
// for slices of lengths corresponding to practical digraph sizes, it
// seems to be more efficient in practice than linear-time algorithms
// such as Kellogg S. Booth's LCS (described in "Lexicographically
// least circular substrings", Information Processing Letters 10(4),
// 1980, pages 240-242, https://doi.org/10.1016/0020-0190(80)90149-0
// and in the errata at https://www.cs.ubc.ca/~ksbooth/PUB/LCS.shtml)
// which we used in the paper in order to obtain the theoretical upper
// bound.

fn is_min_rotation<T: Ord>(s: &[T]) -> bool {
    let n = s.len();
    for r in 1..n {
        for i in 0..n {
            match s[i].cmp(&s[(i + r) % n]) {
                Greater => return false,
                Less => break,
                Equal => (),
            }
        }
    }
    true
}


// Check if component c has unmerge u

fn has_unmerge(c: &Comp, u: &Comp) -> bool {
    let mut i = 0;
    while i < c.len() && c[i][0] == 1 {
        i += 1;
    }
    u[i][0] == 1
}


// Compute the unmerge u of component c and the indices l, r
// such that remerging u between l and r gives back c

fn unmerge(c: &Comp) -> Option<(Comp, usize, usize)> {
    let mut u = Comp::new();
    let mut l = 0;
    while l < c.len() && c[l][0] == 1 {
        u.push(Rc::clone(&c[l]));
        l += 1;
    }
    if l == c.len() {
        return None;
    }
    u.push(Rc::new(vec![1]));
    let t = &c[l];
    let mut k = 1;
    let mut r = l + 1;
    while k < t.len() {
        u.push(Rc::new(t[k..k + t[k] as usize].to_vec()));
        k += t[k] as usize;
        r += 1;
    }
    let mut i = l + 1;
    while i < c.len() {
        u.push(Rc::clone(&c[i]));
        i += 1;
    }
    Some((u, l, r))
}


// Merge the trees c[l], ..., c[r - 1] if that gives a valid
// isomorphism code fo a component

fn merge(c: &Comp, l: usize, r: usize) -> Option<Comp> {
    if c[l][0] != 1 || !is_sorted(&c[l..r]) {
        return None;
    }
    let mut m = Comp::new();
    for i in 0..l {
        m.push(Rc::clone(&c[i]));
    }
    let mut sum = 0;
    let mut t = Tree::new();
    for i in l..r {
        t.extend(&*c[i]);
        sum += c[i][0];
    }
    t[0] = sum;
    m.push(Rc::new(t));
    for i in r..c.len() {
        m.push(Rc::clone(&c[i]));
    }
    if !is_min_rotation(&m) || !has_unmerge(&m, &c) {
        return None;
    }
    Some(m)
}


// Compute the next valid merge of c, if any, starting from l
// (decreasing) and up to and exluding r (increasing); this
// corresponds to the lexicographically minimal merge

fn next_merge(c: &Comp, mut l: usize, mut r: usize) -> Option<Comp> {
    loop {
        while r <= c.len() {
            if let Some(m) = merge(c, l, r) {
                return Some(m);
            }
            r += 1;
        }
        if l == 0 {
            break
        }
        l -= 1;
        r = l + 2;
    }
    None
}


// Compute the next component by merging c, if possible, and otherwise
// by unmerging and remerging, if possible

fn next_comp(c: &Comp) -> Option<Comp> {
    if c.len() >= 2 {
        if let Some(m) = next_merge(c, c.len() - 2, c.len()) {
            return Some(m);
        }
    }
    let mut res = unmerge(&c);
    // This loop is actually executed at most twice,
    // see Lemma 15 of the paper
    while let Some((u, l, r)) = res {
        if let Some(m) = next_merge(&u, l, r + 1) {
            return Some(m);
        }
        res = unmerge(&u);
    }
    None
}


// Compute the number of vertices of a component

fn comp_size(c: &Comp) -> usize {
    let mut n = 0;
    for i in 0..c.len() {
        n += c[i].len();
    }
    n
}


// Return the component consising of a cycle of length n

fn cycle(n: usize) -> Comp {
    let mut c = Comp::new();
    let t = Rc::new(vec![1]);
    for _ in 0..n {
        c.push(Rc::clone(&t));
    }
    c
}


// Generate all components of n vertices, print them using the
// supplied print function and return their count

fn generate_comps(n: usize, print: fn(&Func)) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut c = cycle(n);
    let mut count = 1;
    loop {
        let g: Func = vec![Rc::new(c.clone())];
        print(&g);
        if let Some(d) = next_comp(&c) {
            count += 1;
            c = d;
        } else {
            break;
        }
    }
    count
}


// Compute the sum of a partition of the integer n,
// that is, n itself

fn sum_part(p: &Part) -> usize {
    p.iter().sum::<u8>() as usize
}


// Compute the next partition of integer n in lexicographic order, if
// it exists. The algorithm is based on Algorithm 3.1 of Jerome
// Kelleher, Barry O'Sullivan, "Generating all partitions: A
// comparison of two encodings", arXiv:0909.2331, 2015,
// https://arxiv.org/abs/0909.2331

fn next_part(p: &Part) -> Option<Part> {
    if p.len() <= 1 {
        return None;
    }
    let n = sum_part(p);
    let mut k = p.len() - 1;
    let mut p = p.clone();
    p.extend(vec![0; n - p.len()]);
    let mut y = p[k] - 1;
    k = k - 1;
    let x = p[k] + 1;
    while x <= y {
        p[k] = x;
        y = y - x;
        k = k + 1;
    }
    p[k] = x + y;
    Some(p[0..k + 1].to_vec())
}


// Compute the partition corresponding to functional digraph g,
// that is, the list of sizes of its components

fn part(g: &Func) -> Part {
    let mut p = Part::new();
    for i in 0..g.len() {
        p.push(comp_size(&g[i]) as u8);
    }
    p
}


// Return the functional digraph consisting of n self-loops

fn loops(n: usize) -> Func {
    let mut g = Func::new();
    let c = Rc::new(cycle(1));
    for _ in 0..n {
        g.push(Rc::clone(&c));
    }
    g
}


// Compute the next functional digraph by taking the successor of the
// rightmost component having a successor of the same size, if any;
// otherwise, compute the next partition and restart with the first
// component of each size (the cycle)

fn next_func(g: &Func) -> Option<Func> {
    let mut f = Func::new();
    for h in (0..g.len()).rev() {
        if let Some(c) = next_comp(&g[h]) {
            for i in 0..h {
                f.push(Rc::clone(&g[i]));
            }
            let n = comp_size(&c);
            f.push(Rc::new(c));
            for i in h + 1..g.len() {
                let m = comp_size(&g[i]);
                if m == n {
                    f.push(Rc::clone(&f[h]));
                } else {
                    f.push(Rc::new(cycle(m)));
                }
            }
            return Some(f);
        }
    }
    let p = part(&g);
    if let Some(q) = next_part(&p) {
        for i in 0..q.len() {
            f.push(Rc::new(cycle(q[i] as usize)));
        }
        return Some(f);
    }
    None
}


// Generate all functional digraphs of n vertices, print them using
// the supplied print function and return their count

fn generate_funcs(n: usize, print: fn(&Func)) -> u64 {
    let mut g = loops(n);
    let mut count = 1;
    loop {
        print(&g);
        if let Some(f) = next_func(&g) {
            count += 1;
            g = f;
        } else {
            break;
        }
    }
    count
}


// Compute the adjacency vector of a tree; use b (base) as the name of
// the root (it is 0 for an isolated tree, but of course can be > 0
// when there are several trees)

fn tree_to_adj(t: &Tree, b: usize) -> Adj {
    let mut a = vec![0; t.len()];
    fill_tree_adj(t, &mut a, 0, 0, b);
    a
}


// Fill a with the adjacency vector of tree t starting from its
// subtree having root at position i; r is the position of the parent
// of this subtree (if any); use b as the name of the root of t

fn fill_tree_adj(t: &Tree, a: &mut Adj, i: usize, r: usize, b: usize) {
    a[i] = (r + b) as u8;
    let mut j = i + 1;
    while j < i + t[i] as usize {
        fill_tree_adj(t, a, j, i, b);
        j += t[j] as usize;
    }
}


// Compute the adjacency vector of component c, using b as the name of
// the first vertex of c

fn comp_to_adj(c: &Comp, b: usize) -> Adj {
    let mut a = Adj::new();
    let mut j = 0;
    for i in 0..c.len() {
        let mut a1 = tree_to_adj(&c[i], b + j);
        if i < c.len() - 1 {
            a1[0] = (b + j + c[i].len()) as u8;
        } else {
            a1[0] = b as u8;
        }
        a.extend(a1);
        j += c[i].len();
    }
    a
}


// Compute the adjacency vector of functional digraph g

fn func_to_adj(g: &Func) -> Adj {
    let mut a = Adj::new();
    let mut b = 0;
    for i in 0..g.len() {
        let a1 = comp_to_adj(&g[i], b);
        b += a1.len();
        a.extend(a1);
    }
    a
}


// Convert an adjacency vector to an adjacency matrix represented as a
// bit vector (containing the concatenation of the rows of the matrix)

fn adj_matrix(a: &Adj) -> Bits {
    let n = a.len();
    let mut m = Bits::new();
    for i in 0..n {
        for j in 0..n {
            m.push(a[i] == j as u8);
        }
    }
    m
}


// Print a bit vector as an ASCII string according to the digraph6
// specifications (https://users.cecs.anu.edu.au/~bdm/data/formats.txt,
// function R(x))

fn print_bits(x: &Bits) {
    let mut i = 0;
    while i < x.len() {
        let mut n = 0;
        for j in i..i + 6 {
            if j < x.len() {
                n = 2 * n + x[j] as u8;
            } else {
                n = 2 * n;
            }
        }
        print!("{}", (n + 63) as char);
        i += 6;
    }
}


// Print functional digraph g in digraph6 format, implements function
// N(n) of the digraph6 specifications and uses R(x)
// (https://users.cecs.anu.edu.au/~bdm/data/formats.txt)

fn print_digraph6(g: &Func) {
    print!("&");
    let a = func_to_adj(&g);
    let mut n = a.len();
    if n < 63 {
        print!("{}", (n as u8 + 63) as char);
    } else {
        // 63 <= n <= 255, since args.size is u8
        let mut b = Bits::new();
        for _ in 0..18 {
            b.push(n % 2 == 1);
            n /= 2;
        }
        b.reverse();
        print!("{}", 126 as char);
        print_bits(&b);
    }
    let m = adj_matrix(&a);
    print_bits(&m);
    println!();
}


// Print functional digraph g in internal format (list of lists of
// lists of integers)

fn print_internal(g: &Func) {
    println!("{g:?}");
}


// Do not print functional digraph _g

fn print_nothing(_g: &Func) {
    // do nothing
}


// Structure for the command-line arguments

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(help = "Number of vertices")]
    size: u8,

    #[arg(short, long, help = "Only generate connected digraphs")]
    connected: bool,

    #[arg(short, long,
          help = "Print the internal representation instead of digraph6")]
    internal: bool,

    #[arg(short, long, help = "Count the digraphs without printing them")]
    quiet: bool,
}


// Main program

fn main() {
    let args = Args::parse();
    let n = args.size as usize;
    let generate = if args.connected {
        generate_comps
    } else {
        generate_funcs
    };
    let print = if args.quiet {
        print_nothing
    } else if args.internal {
        print_internal
    } else {
        print_digraph6
    };
    let now = Instant::now();
    let count = generate(n, print);
    let time = now.elapsed();
    eprintln!("{count} digraphs generated in {time:.2?}");
}
