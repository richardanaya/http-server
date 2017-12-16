extern crate iron;
extern crate staticfile;
extern crate mount;
use std::path::Path;
use std::process::exit;
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
    #[structopt(help = "Input directory", default_value = "./")]
    path: String,
}

fn main() {
    let opt: Opt = Opt::from_args();
    let port: u16 = opt.port;
    let address: &str = &*opt.address;
    let path: &Path = Path::new(&opt.path);

    if !path.exists() {
        println!("Path {:?} does not exist.", path);
        exit(1)
    }
    if !path.is_dir() {
        println!("Path {:?} is not a directory.", path);
        exit(1)
    }
    let mut mount: Mount = Mount::new();
    mount.mount("/", Static::new(path));

    match Iron::new(mount).http((address, port)) {
        Ok(_) => {
            println!("Starting up http-server, serving path {:?}", path);
            println!("Available on:");
            println!("  http://{}:{}", address, port);
            println!("Hit CTRL-C to stop the server")
        }
        Err(err) => {
            println!("{}", err);
            exit(1)
        }
    }
}