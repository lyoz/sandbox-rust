use bitonic_sorter::fourth::sort as parallel_sort;
use bitonic_sorter::third::sort as sequential_sort;
use bitonic_sorter::util::{is_sorted_ascending, new_u32_vec};
use bitonic_sorter::SortOrder;
use num_cpus;
use std::env;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    if let Some(n) = env::args().nth(1) {
        let bits = u32::from_str(&n).expect("error parsing argument");
        println!("bits={}", bits);
        run_sorts(bits);
    } else {
        eprintln!(
            "Usage {} <number of elements in bits>",
            env::args().nth(0).unwrap()
        );
        std::process::exit(1);
    }
}

fn run_sorts(bits: u32) {
    let len = 2.0_f64.powi(bits as i32) as usize;
    println!(
        "sorting {} integers ({:.1} MB)",
        len,
        (len * std::mem::size_of::<u32>()) as f64 / 1024. / 1024.
    );
    println!(
        "cpu info: {} physical cores, {} logical cores",
        num_cpus::get_physical(),
        num_cpus::get()
    );

    let seq_duration = timed_sort(&sequential_sort, len, "sequential sort");
    let par_duration = timed_sort(&parallel_sort, len, "parallel sort  ");

    println!("speed up: {:.2}x", seq_duration / par_duration);
}

fn timed_sort<F>(sorter: &F, len: usize, name: &str) -> f64
where
    F: Fn(&mut [u32], &SortOrder) -> Result<(), String>,
{
    let mut x = new_u32_vec(len);

    let start = Instant::now();
    sorter(&mut x, &SortOrder::Ascending).expect("Failed to sort: ");
    let duration = start.elapsed();

    let nano_secs = duration.as_nanos() as f64;
    println!(
        "{}: sorted {} integers in {} seconds",
        name,
        len,
        nano_secs / 1e9
    );

    assert!(is_sorted_ascending(&x));

    nano_secs
}
