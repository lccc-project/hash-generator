use std::hash::{Hash, Hasher};

use hash_generator::hash::SipHasher;

const TEST_CRATE_ID: u64 = 8936564510611380703;

const SAMPLE: &[&str] = &[
    "Lorem",
    "ipsum",
    "dolor",
    "sit",
    "amet",
    "consectetur",
    "adipiscing",
    "elit",
    "Curabitur",
    "non",
    "ligula",
    "nec",
    "laoreet",
    "efficitur",
    "in",
    "quis",
    "erat",
    "Etiam",
    "eu",
    "ultrices",
    "sapien",
    "Phasellus",
    "commodo",
    "suscipit",
    "felis",
    "sed",
    "mollis",
    "finibus",
    "vel",
    "Fusce",
    "pellentesque",
    "quam",
    "et",
    "ornare",
    "volutpat",
    "Praesent",
    "ultricies",
    "malesuada",
    "Vivamus",
    "cursus",
    "Nam",
    "varius",
    "libero",
    "augue",
    "viverra",
    "at",
    "accumsan",
    "iaculis",
    "Proin",
    "tempus",
    "purus",
    "congue",
    "bibendum",
    "Integer",
    "posuere",
    "est",
    "tempor",
    "aliquam",
    "diam",
    "nulla",
    "convallis",
    "id",
    "nisi",
    "arcu",
    "ac",
    "mauris",
    "pulvinar",
    "lacus",
    "ex",
    "porta",
    "sem",
    "ut",
    "tellus",
    "enim",
    "eleifend",
    "dui",
    "metus",
    "Vestibulum",
    "vestibulum",
    "tortor",
    "ante",
    "tincidunt",
    "Nulla",
    "vulputate",
    "sagittis",
    "Nullam",
    "mattis",
    "a",
];

const TEST_DENSITY: f64 = 0.0;

const TEST_STEPS: usize = 1024;

fn main() {
    let inputs = SAMPLE;
    let (key, tsize) = hash_generator::gen::gen_hash_fn::<_, 2, 4>(
        &inputs,
        TEST_CRATE_ID,
        TEST_DENSITY,
        TEST_STEPS,
    );

    let mut dupset = vec![Vec::new(); tsize];

    println!("Generating for: {:?}", SAMPLE);
    println!();
    println!("Key: {:#x}", key);
    println!("Table Size: {}", tsize);
    println!();
    println!("Outputs:");
    for &str in inputs {
        let mut hasher = SipHasher::<2, 4>::new_with_keys(TEST_CRATE_ID, key);
        str.hash(&mut hasher);
        let hash = hasher.finish();

        let idx = (hash as usize) & (tsize - 1);

        dupset[idx].push(str);

        println!(
            "Hash of \"{}\": {:#x} (index {})",
            str.escape_default(),
            hash,
            idx
        );
    }
    println!();
    println!("Duplicates:");
    for (idx, set) in dupset.iter().enumerate().filter(|(_, v)| v.len() > 1) {
        print!("{}:", idx);

        for item in set {
            print!(" \"{}\"", item);
        }
        println!();
    }
}
