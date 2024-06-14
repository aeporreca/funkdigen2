mod tree {

    pub type Tree = Vec<u8>;

    pub fn is_valid(t: &Tree) -> bool {
        // Assumes that the subts are valid
        let n = t.len();
        if n == 1 {
            return true;
        }
        let mut l = 1;
        let mut m = l + t[l] as usize;
        while m < n {
            let r = m + t[m] as usize;
            if t[l..m] > t[m..r] {
                return false;
            }
            l = m;
            m = r;
        }
        true
    }

    pub fn merge(t1: &Tree, t2: &Tree) -> Tree {
        let mut t = vec![t1[0] + t2[0]];
        t.extend(&t1[1..]);
        t.extend(t2);
        t
    }

    pub fn unmerge(t: &Tree) -> (Tree, Tree) {
        let n = t.len();
        let mut l = 1;
        while l + (t[l] as usize) < n {
            l += t[l] as usize;
        }
        let t2 = t[l..].to_vec();
        let mut t1 = vec![t[0] - t2[0]];
        t1.extend(&t[1..l]);
        (t1, t2)
    }

}


mod comp {

    use std::rc::Rc;
    use crate::tree;
    use crate::is_min_rotation;

    pub type Comp = Vec<Rc<tree::Tree>>;

    pub fn cycle(n: usize) -> Comp {
        let mut c = vec![];
        let t = Rc::new(vec![1]);
        for _ in 0..n {
            c.push(t.clone());
        }
        c
    }

    pub fn is_valid(c: &Comp) -> bool {
        for t in c {
            if !tree::is_valid(&t) {
                return false;
            }
        }
        is_min_rotation(&c)
    }

    pub fn n_candidates(c: &Comp) -> usize {
        let k = c.len();
        2 * (k - 1)
    }

    pub fn candidate(c: &Comp, i: usize) -> Option<Comp> {
        let k = c.len();
        if i < k - 1 {
            let mut d = c[..i].to_vec();
            d.push(tree::merge(&c[i], &c[i+1]).into());
            d.extend_from_slice(&c[i+2..]);
            if is_valid(&d) {
                return Some(d);
            }
        } else if k - 1 <= i && i < 2 * (k - 1) {
            let j = i - (k - 1);
            if c[j] != c[j+1] {
                let mut d = c[..j].to_vec();
                d.push(tree::merge(&c[j+1], &c[j]).into());
                d.extend_from_slice(&c[j+2..]);
                if is_valid(&d) {
                    return Some(d);
                }
            }
        }
        None
    }

    pub fn parent(c: &Comp) -> Option<Comp> {
        let k = c.len();
        for i in 0..k {
            if c[i].len() > 1 {
                let (t1, t2) = tree::unmerge(&c[i]);
                let mut d = c[..i].to_vec();
                if t1 <= t2 {
                    d.push(t1.into());
                    d.push(t2.into());
                } else {
                    d.push(t2.into());
                    d.push(t1.into());
                }
                d.extend_from_slice(&c[i+1..]);
                return Some(d);
            }
        }
        None
    }

    pub fn backtrack(c: &Comp) -> Option<usize> {
        let k = c.len();
        for i in 0..k {
            if c[i].len() > 1 {
                let (t1, t2) = tree::unmerge(&c[i]);
                if t1 <= t2 {
                    return Some(i);
                } else {
                    return Some(i + k);
                }
            }
        }
        None
    }

    pub fn _depth(c: &Comp) -> usize {
        let mut size = 0;
        for t in c {
            size += t.len();
        }
        return (size - c.len()) % 2
    }

    use crate::PRINT_FUNC;

    pub fn generate(n: usize) -> usize {
        let mut i = 0;
        let first = cycle(n);
        let mut curr = first.clone();
        let mut count = 1;
        PRINT_FUNC(&curr);
        let mut depth = 0;
        while curr != first || i < n_candidates(&curr) {
            while i < n_candidates(&curr) {
                let candidate = candidate(&curr, i);
                i += 1;
                if let Some(next) = candidate {
                    let parent = parent(&next).unwrap();
                    if parent == curr {
                        curr = next;
                        depth = 1 - depth;
                        if depth == 0 {
                            count += 1;
                            PRINT_FUNC(&curr);
                        }
                        i = 0;
                    }
                }
            }
            if curr != first {
                if depth == 1 {
                    count += 1;
                    PRINT_FUNC(&curr);
                }
                i = backtrack(&curr).unwrap() + 1;
                curr = parent(&curr).unwrap();
                depth = 1 - depth;
            }
        }
        count
    }

}


use std::cmp::Ordering::{Less, Equal, Greater};

fn is_min_rotation<T: Ord>(s: &[T]) -> bool {
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


use crate::comp::Comp;

fn print_internal(c: &Comp) {
    println!("{c:?}");
}


fn print_nothing(_c: &Comp) {
    // do nothing
}


use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(help = "Number of vertices")]
    size: u8,

    #[arg(short, long, conflicts_with = "internal",
          help = "Count digraphs without printing them")]
    quiet: bool,
}


use lazy_static::lazy_static;

lazy_static! {

    static ref ARGS: Args = Args::parse();

    static ref PRINT_FUNC: fn(&Comp) = if ARGS.quiet {
        print_nothing
    } else {
        print_internal
    };

}


use std::time::Instant;

fn main() {
    let n = ARGS.size as usize;
    let now = Instant::now();
    let count = comp::generate(n);
    let time = now.elapsed();
    eprintln!("{count} digraphs generated in {time:.2?}");
}
