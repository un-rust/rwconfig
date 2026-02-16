use log::trace;
use packageName::add_two_numbers;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    trace!("Hello, world!");
    add_two_numbers(1, 2);
}
