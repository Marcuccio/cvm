use rand::Rng;
use std::collections::HashSet;
use std::hash::Hash;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub fn estimate_from_many(paths: &[PathBuf], delta: f64, epsilon: f64) -> io::Result<usize> {
    let m = count_rows(paths)?;
    
    let mut rng = rand::thread_rng();

    let mut x: HashSet<String> = HashSet::new();
    let mut p: f64 = 1.0;

    let thresh = ((12.0 / epsilon.powi(2)) * (8.0 * m as f64 / delta).log2()).ceil() as usize;

    for path in paths {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let el = line?;
            x.remove(&el);

            if rng.gen_bool(p) {
                x.insert(el.clone());            
            }

            if x.len() == thresh {
                x.retain(|_| rng.gen_bool(0.5));
                p /= 2.0;
            }
        }
    }

    Ok((x.len() as f64 / p) as usize)
}

fn count_rows(paths: &[PathBuf]) -> io::Result<usize> {
    let mut total_lines = 0;
    for path in paths {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        total_lines += reader.lines().count();
    }
    Ok(total_lines)
}

fn estimate<T>(source: impl IntoIterator<Item = T>, delta: f64, epsilon: f64) -> usize
where
T: PartialOrd + PartialEq + Eq + Hash + Clone, {
    let mut rng = rand::thread_rng();

    let source_vec: Vec<T> = source.into_iter().collect();

    let mut x: HashSet<T> = HashSet::new();
    let mut p: f64 = 1 as f64;
    let m = source_vec.len();

    let thresh = ((12.0 / epsilon.powi(2)) * (8.0 * m as f64 / delta).log2()).ceil() as usize;


    for el in source_vec.iter() {
        x.remove(el);

        if rng.gen_bool(p) {
            x.insert(el.to_owned());
        }

        if x.len() == thresh {
            x.retain(|_| rng.gen_bool(0.5));
            p = p/2.0;
        }
    }

    (x.len() as f64 / p) as usize
}
