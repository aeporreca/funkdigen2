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
use lazy_static::lazy_static;
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
// seems to be more efficient in practice for n <= 255.

fn naive_is_min_rotation<T: Ord>(s: &[T]) -> bool {
    for r in 1..s.len() {
        for i in 0..s.len() {
            match s[i].cmp(&s[(i + r) % s.len()]) {
                Greater => return false,
                Less => break,
                Equal => (),
            }
        }
    }
    true
}


// Check if slice s is its own minimal rotation. This is based on
// the linear-time algorithms such as Kellogg S. Booth's LCS
// (described in "Lexicographically least circular substrings",
// Information Processing Letters 10(4), 1980, pages 240-242,
// https://doi.org/10.1016/0020-0190(80)90149-0 and in the errata at
// https://www.cs.ubc.ca/~ksbooth/PUB/LCS.shtml) which we used in the
// paper in order to obtain the theoretical upper bound, but it is
// empirically slower that the naive algorithm for n <= 255.

fn lcs_is_min_rotation<T: Ord>(s: &[T]) -> bool {
    let n = s.len();
    let mut f = vec![-1; 2 * n];
    let mut k = 0;
    for j in 1..2 * n {
        let mut i = f[j - k - 1 as usize];
        while i != -1 && s[j % n] != s[(k + i as usize + 1) % n] {
            if s[j % n] < s[(k + i as usize + 1) % n] {
                k = j - i as usize - 1;
            }
            i = f[i as usize];
        }
        if i == -1 && s[j % n] != s[(k + i as usize + 1) % n] {
            if s[j % n] < s[(k + i as usize + 1) % n] {
                k = j;
            }
            f[j - k] = -1;
        } else {
            f[j - k] = i + 1;
        }
    }
    s[k..] == s[..n - k] && s[..k] == s[n - k..]
}


// Compute the unmerge u of component c and the indices l, r
// such that remerging u between l and r gives back c

fn unmerge(c: &Comp) -> Option<(Comp, usize, usize)> {
    let mut u = Comp::new();
    let mut l = 0;
    while l < c.len() && c[l].len() == 1 {
        u.push(c[l].clone());
        l += 1;
    }
    if l == c.len() {
        return None;
    }
    u.push(Rc::new(vec![1]));
    let t = &c[l];
    let mut i = 1;
    let mut r = l + 1;
    while i < t.len() {
        u.push(Rc::new(t[i..i + t[i] as usize].to_vec()));
        i += t[i] as usize;
        r += 1;
    }
    for i in l + 1..c.len() {
        u.push(c[i].clone());
    }
    Some((u, l, r))
}



// Check if component c has unmerge u (this is not a general purpose
// function, it only works in the context of the function merge below)

fn has_unmerge(c: &Comp, u: &Comp) -> bool {
    let mut i = 0;
    while i < c.len() && c[i].len() == 1 {
        i += 1;
    }
    u[i][0] == 1
}


// Merge trees c[l], ..., c[r - 1] if that gives a valid isomorphism
// code for a component

fn merge(c: &Comp, l: usize, r: usize) -> Option<Comp> {
    if c[l].len() != 1 || !is_sorted(&c[l..r]) {
        return None;
    }
    let mut m = Comp::new();
    for i in 0..l {
        m.push(c[i].clone());
    }
    let mut t = vec![1];
    for i in l + 1..r {
        t.extend_from_slice(&c[i]);
        t[0] += c[i].len() as u8;
    }
    m.push(Rc::new(t));
    for i in r..c.len() {
        m.push(c[i].clone());
    }
    if !IS_MIN_ROTATION(&m) || !has_unmerge(&m, &c) {
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
            return None;
        }
        l -= 1;
        r = l + 2;
    }
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
        c.push(t.clone());
    }
    c
}


// Generate all components of n vertices, print them using the
// supplied print function and return their count

fn generate_comps(n: usize) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut c = cycle(n);
    let mut count = 1;
    loop {
        let g: Func = vec![Rc::new(c.clone())];
        PRINT_FUNC(&g);
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


// Compute the next partition of integer n in lexicographic order,
// if it exists. The algorithm is based on Algorithm 3.1 of Jerome
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
        g.push(c.clone());
    }
    g
}


// Compute the next functional digraph by taking the successor of the
// rightmost component having a successor of the same size, if any;
// otherwise, compute the next partition and restart with the first
// component of each size (the cycle)

fn next_func(g: &Func)
             -> Option<Func> {
    let mut f = Func::new();
    for h in (0..g.len()).rev() {
        if let Some(c) = next_comp(&g[h]) {
            f.extend_from_slice(&g[0..h]);
            let n = comp_size(&c);
            f.push(Rc::new(c));
            for i in h + 1..g.len() {
                let m = comp_size(&g[i]);
                if m == n {
                    f.push(f[h].clone());
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

fn generate_funcs(n: usize) -> u64 {
    let mut g = loops(n);
    let mut count = 1;
    loop {
        PRINT_FUNC(&g);
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

fn tree_adj(t: &Tree, b: usize) -> Adj {
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

fn comp_adj(c: &Comp, b: usize) -> Adj {
    let mut a = Adj::new();
    let mut j = 0;
    for i in 0..c.len() {
        let mut a1 = tree_adj(&c[i], b + j);
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

fn func_adj(g: &Func) -> Adj {
    let mut a = Adj::new();
    let mut b = 0;
    for i in 0..g.len() {
        let a1 = comp_adj(&g[i], b);
        b += a1.len();
        a.extend(a1);
    }
    a
}


// Convert an adjacency vector to an adjacency matrix represented as a
// bit vector (containing the concatenation of the rows of the
// matrix), deleting self-loops if ARGS.loopless is true

fn adj_matrix(a: &Adj) -> Bits {
    let mut m = Bits::new();
    for i in 0..a.len() {
        for j in 0..a.len() {
            m.push((!ARGS.loopless || i != j) && a[i] == j as u8);
        }
    }
    m
}


// Convert a bit vector into an ASCII string according to the digraph6
// specifications

fn bits_to_ascii(x: &Bits) -> String {
    let mut s = String::new();
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
        s.push((n + 63) as char);
        i += 6;
    }
    s
}


// Returns a string representing integer n (in the range 0..255) in
// digraph6 ASCII format, implements function N(n) of the digraph6
// specifications

fn int_to_ascii(mut n: usize) -> String {
    if n < 63 {
        ((n as u8 + 63) as char).to_string()
    } else {
        // 63 <= n <= 255, since args.size is u8
        let mut b = Bits::new();
        for _ in 0..18 {
            b.push(n % 2 == 1);
            n /= 2;
        }
        b.reverse();
        bits_to_ascii(&b)
    }
}


// Print functional digraph g in digraph6 format (described at
// https://users.cecs.anu.edu.au/~bdm/data/formats.txt), deleting
// self-loops first if ARGS.loopless is true

fn print_digraph6(g: &Func) {
    print!("&");
    let a = func_adj(&g);
    let n = a.len();
    print!("{}", int_to_ascii(n));
    let m = adj_matrix(&a);
    println!("{}", bits_to_ascii(&m));
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

    #[arg(short, long, help = "Print internal \
          representation instead of digraph6")]
    internal: bool,

    #[arg(short, long, conflicts_with = "internal",
          conflicts_with = "quiet",
          help = "Remove self-loops before printing (digraph6 only)")]
    loopless: bool,

    #[arg(short, long, conflicts_with = "internal",
          help = "Count digraphs without printing them")]
    quiet: bool,

    #[arg(short = 'b', long, help = "Use Booth's \
          LCS algorithm for minimal rotations")]
    lcs: bool,
}


// Program options

lazy_static! {

    static ref ARGS: Args = Args::parse();

    static ref GENERATE: fn(usize) -> u64 = if ARGS.connected {
        generate_comps
    } else {
        generate_funcs
    };

    static ref PRINT_FUNC: fn(&Func) = if ARGS.quiet {
        print_nothing
    } else if ARGS.internal {
        print_internal
    } else {
        print_digraph6
    };

    static ref IS_MIN_ROTATION: fn(&Comp) -> bool = if ARGS.lcs {
        |s| lcs_is_min_rotation(s)
    } else {
        |s| naive_is_min_rotation(s)
    };

}


// Main program

fn main() {
    let n = ARGS.size as usize;
    let now = Instant::now();
    let count = GENERATE(n);
    let time = now.elapsed();
    eprintln!("{count} digraphs generated in {time:.2?}");
}
