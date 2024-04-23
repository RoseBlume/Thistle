use num_cpus;
pub fn count() -> usize{
    let cpus = num_cpus::get();
    return cpus
}
