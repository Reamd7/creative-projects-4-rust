mod rand_main;
mod user_log_main;
mod lazy_static_main;
mod structopt_main;

fn main() {
    println!("Hello, world!");

    rand_main::rand_main();

    user_log_main::user_log_main();

    lazy_static_main::main();

    structopt_main::main();
}
