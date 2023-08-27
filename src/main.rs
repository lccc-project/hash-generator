use std::{
    hash::{BuildHasher, Hash, Hasher},
    io::Read,
};

use hash_generator::hash::{HashBytes, SipHasher};

const TEST_CRATE_ID: u64 = 8936564510611380703;

const TEST_DENSITY: f64 = 0.0;

const TEST_STEPS: usize = 1024;

fn main() {
    let mut global_key = TEST_CRATE_ID;
    let mut density = TEST_DENSITY;
    let mut steps = TEST_STEPS;

    let mut args = std::env::args();

    let prg_name = args.next().unwrap();

    while let Some(arg) = args.next() {
        match &*arg {
            "--steps" => {
                let val = args.next().unwrap_or_else(|| {
                    eprintln!("{}: --steps expects an argument", prg_name);
                    std::process::exit(1)
                });

                steps = val.parse().unwrap_or_else(|_| {
                    eprintln!("{}: --steps expects an integer", prg_name);
                    std::process::exit(1)
                });
            }
            "--inverse-threshold" => {
                let val = args.next().unwrap_or_else(|| {
                    eprintln!("{}: --inverse-threshold expects an argument", prg_name);
                    std::process::exit(1)
                });

                density = 1.0
                    / val.parse::<u32>().unwrap_or_else(|_| {
                        eprintln!("{}: --inverse-threshold expects a integer", prg_name);
                        std::process::exit(1)
                    }) as f64;
            }
            "--global-key" => {
                let val = args.next().unwrap_or_else(|| {
                    eprintln!("{}: --global-key expects an argument", prg_name);
                    std::process::exit(1)
                });

                global_key = u64::from_str_radix(&val, 16).unwrap_or_else(|_| {
                    eprintln!("{}: --global-key expects a hexadecimal integer", prg_name);
                    std::process::exit(1)
                });
            }
            "--version" => {
                println!("hash-generator test program v{}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0)
            }
            "--help" => {
                println!("Usage: {} [OPTIONS...]", prg_name);
                println!("Generates a near-perfect hash function from stdin");
                println!("Options:");
                println!("\t--steps [steps]: performs steps increments of the local_key per table size increment");
                println!("\t--global-key [key]: Sets the global key for the table");
                println!("\t--inverse-thresold [size]: Sets the inverse threshold - IE. the minimum size of the table before collisions are accepted");
                println!("\t--version: Display version information and exits");
                println!("\t--help: Displays this message and exits");
                std::process::exit(0)
            }
            _ => {
                eprintln!("Usage: {} [OPTIONS...]", prg_name);
                std::process::exit(1)
            }
        }
    }

    let mut input = String::new();

    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let inputs = input.split("\n").map(HashBytes).collect::<Vec<_>>();

    let stats = hash_generator::gen::gen_hash_fn::<_, 2, 4>(&inputs, global_key, density, steps);

    let key = stats.keys().1;

    let tsize = stats.table_size();

    println!();
    println!("Key: {:#x}", key);
    println!("Table Size: {}", tsize);
    println!();
    println!("Outputs:");
    for &str in &inputs {
        let mut hasher = stats.build_hasher();
        str.hash(&mut hasher);
        let hash = hasher.finish();

        let idx = (hash as usize) & (tsize - 1);

        println!(
            "Hash of \"{}\": {:#x} (index {})",
            str.escape_default(),
            hash,
            idx
        );
    }
    println!();
    println!("Duplicates:");
    for (idx, set) in stats.collision_map() {
        print!("{}:", idx);

        for item in set {
            print!(" \"{}\"", &**item);
        }
        println!();
    }
}
