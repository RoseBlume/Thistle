use num_cpus;
pub fn count() -> usize{
    let cpus = num_cpus::get();
    if cpus > 1 {
        println!("We are on a multicore system with {} CPUs", cpus);
    } else {
        println!("We are on a single core system");
    }
    return cpus
}
