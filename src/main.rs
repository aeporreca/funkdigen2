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
// algorithm which increases the actual runtime from O(n^3) but, for
// slices of lengths corresponding to practical digraph sizes, it
// seems to be more efficient than linear-time algorithms such as
// Kellogg S. Booth's LCS (described in "Lexicographically least
// circular substrings", Information Processing Letters 10(4), 1980,
// pages 240-242, https://doi.org/10.1016/0020-0190(80)90149-0 and in
// the errata published at https://www.cs.ubc.ca/~ksbooth/PUB/LCS.shtml)
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
    let mut u = Vec::new();
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
    let mut m = Vec::new();
    for i in 0..l {
        m.push(Rc::clone(&c[i]));
    }
    let mut sum = 0;
    let mut t = Vec::new();
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

fn next_merge(c: &Comp, l: usize, r: usize) -> Option<Comp> {
    let mut l = l;
    let mut r = r;
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
    // This loop is actually executed at most twice
    while let Some((u, l, r)) = res {
        if let Some(m) = next_merge(&u, l, r + 1) {
            return Some(m);
        }
        res = unmerge(&u);
    }
    None
}


// Compute the number of vertices of a component

fn size(c: &Comp) -> u8 {
    let mut n = 0;
    for i in 0..c.len() {
        n += c[i].len();
    }
    n as u8
}


// Return the component consising of a cycle of length n

fn cycle(n: u8) -> Comp {
    let mut c = Vec::new();
    let t = Rc::new(vec![1]);
    for _ in 0..n {
        c.push(Rc::clone(&t));
    }
    c
}


// Generate all components of n vertices and return their count;
// also print them if print is true

fn gen_comps(n: u8, print: bool) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut c = cycle(n);
    let mut count = 1;
    loop {
        if print {
            println!("{c:?}");
        }
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
// it exists. The algorithm is based on Algorithm 3.1 of "Generating
// all partitions: A comparison of two encodings" by Jerome Kelleher
// and Barry O'Sullivan, https://arxiv.org/abs/0909.2331

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
    let mut p = Vec::new();
    for i in 0..g.len() {
        p.push(size(&g[i]));
    }
    p
}


// Return the functional digraph consisting of n self-loops

fn loops(n: u8) -> Func {
    let mut g = Vec::new();
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
    let mut f = Vec::new();
    for h in (0..g.len()).rev() {
        if let Some(c) = next_comp(&g[h]) {
            for i in 0..h {
                f.push(Rc::clone(&g[i]));
            }
            let n = size(&c);
            f.push(Rc::new(c));
            for i in h + 1..g.len() {
                let m = size(&g[i]);
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
            f.push(Rc::new(cycle(q[i])));
        }
        return Some(f);
    }
    None
}


// Generate all functional digraphs of n vertices and return
// their count; also print them if print is true

fn gen_funcs(n: u8, print: bool) -> u64 {
    let mut g = loops(n);
    let mut count = 1;
    loop {
        if print {
            println!("{g:?}");
        }
        if let Some(f) = next_func(&g) {
            count += 1;
            g = f;
        } else {
            break;
        }
    }
    count
}


// Structure for the command-line arguments

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(help = "Number of vertices")]
    size: u8,
    #[arg(short, long, help = "Only generate connected digraphs")]
    connected: bool,
    #[arg(short, long, help = "Count the digraphs without printing them")]
    quiet: bool,
}


// Main program

fn main() {
    let args = Args::parse();
    let n = args.size;
    let gen = if args.connected {
        gen_comps
    } else {
        gen_funcs
    };
    let now = Instant::now();
    let count = gen(n, !args.quiet);
    let time = now.elapsed();
    eprintln!("{count} digraphs generated in {time:.2?}");
}
