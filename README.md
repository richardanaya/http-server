# http-server

A dead simple http server for serving local files for development purposes

# Installing

```bash
 cargo install http-server
```

# Usage

To server the local directory on http://localhost:8080

```bash
http-server
```

To server the local directory on http://localhost:[port]

```bash
http-server -p <port>
```

To server the any directory on http://localhost:[port]

```bash
http-server -p <port> <directory>
```

For help:

```bash
http-server --help
```

# Credit

This library is a tribute to a javascript project https://github.com/indexzero/http-server that was a faithful ally to me for many years of web dev. My hope is to give the same usefulness to the Rust community.
