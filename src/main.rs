extern crate iron;
extern crate staticfile;
extern crate mount;
use std::path::Path;
use iron::Iron;
use staticfile::Static;
use mount::Mount;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "http-server", about = "A simple tool for hosting a static file server")]
struct Opt {
    /// A flag, true if used in the command line.
    #[structopt(short = "p", long = "port", help = "Port to use",  default_value = "8080")]
    port: u16,

    /// A flag, true if used in the command line.
    #[structopt(short = "a", long = "address", help = "Address to use",  default_value = "0.0.0.0")]
    address: String,

    /// Needed parameter, the first on the command line.
    #[structopt(help = "Input file", default_value = "./")]
    input: String,
}

fn main() {
    let opt = Opt::from_args();
    let mut mount = Mount::new();
    let path = Path::new(&opt.input);
    if !path.exists() {
            println!("Path \"{}\" does not exist.",&opt.input);
            return;
    }
    if !path.is_dir() {
            println!("Path \"{}\" is not a directory.",&opt.input);
            return;
    }
    println!("Starting up http-server, serving {}",&opt.input);
    mount.mount("/", Static::new(path));
    println!("Available on:");
    println!("  http://{}:{}",opt.address, opt.port);
    println!("Hit CTRL-C to stop the server");
    Iron::new(mount).http((&*opt.address, opt.port)).unwrap();
}
