# http-server

A dead simple http server for serving local files for development purposes

# Installing

```bash
 cargo install http-server
```

# Usage

To serve the local directory on http://localhost:8080

```bash
http-server
```

To serve the local directory on http://localhost:[port]

```bash
http-server -p <port>
```

To serve the any directory on http://localhost:[port]

```bash
http-server -p <port> <directory>
```

For help:

```bash
http-server --help
```

# Command Line Usage

```bash
USAGE:
    http-server [OPTIONS] [input]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --address <address>    Address to use [default: 0.0.0.0]
    -p, --port <port>          Port to use [default: 8080]

ARGS:
    <input>    Input file [default: ./]
```

# Credit

This library is a tribute to a javascript project https://github.com/indexzero/http-server that was a faithful ally to me for many years of web dev. My hope is to give the same usefulness to the Rust community.
