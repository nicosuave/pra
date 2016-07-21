extern crate argparse;

use argparse::{ArgumentParser, StoreTrue};

fn main() {
    let mut should_authenticate = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("PRA – the pull request aggregator");
        ap.refer(&mut should_authenticate).add_option(&["--a", "--authenticate"], StoreTrue, "Authenticate");
        ap.parse_args_or_exit();
    }

    if should_authenticate {
        authenticate()
    }
}

fn authenticate() {
    println!("Looks like you want to authenticate!");
}
