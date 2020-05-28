<div align="center">
  <img src="https://camo.githubusercontent.com/734a3468bce992fbc3b729562d41c92f4912c99a/68747470733a2f2f7777772e727573742d6c616e672e6f72672f7374617469632f696d616765732f727573742d6c6f676f2d626c6b2e737667" height="120" width="120" />
  <h1>netutil</h1>
</div>

> This application is under development, commands/API may change

## Install
To install netutil in your system you must clone the repository, build the package targeting `release` and run `cargo install`.

### Install de binary using cargo

```bash
cargo install netutil
```

### Check installation

```bash
$ netutil --version
```

## Uninstall
As this is a `cargo` binary, you can easily uninstall the package issuing `cargo uninstall --bin netutil`.

## Usage
netutil is a simple port sniffer tool, which helps to know which ports are open in a given range by attempting to open `TCP` connections.
To test a single address you must run:

```bash
$ netutil 127.0.0.1:8080
```

This command will return an output similar to the following:

```bash
> • 127.0.0.1:8080 - Open
```

This output means that the port `8080` is open in the IP address (local IP address), `127.0.0.1`.

A set of ports can also be tested using the `-r` argument, which stands for "range":

```bash
$ netutil 127.0.0.1:8080 -r 8100
```

> The range is made from the `target` address port until the `range` argument value port. In the case above the range would be [8080, 8100].

As a set of ports are going to be tested, a confirmation is prompted before initializing the task:

```bash
$ netutil 127.0.0.1:8080 -r 8100
> Are you sure you want to scan on 127.0.0.1:8080 from port 8080 to 8100? (y/n):
```

If no ports are available after executing the task, then a message like the following will appear:

```bash
> No ports open in address 127.0.0.1:8080 for the port range 8080 to 8100
```

Otherwise a list of available addresses is shown:

```bash
> • 127.0.0.1:8080 - Open
> • 127.0.0.1:8084 - Open
> • 127.0.0.1:8095 - Open
> • 127.0.0.1:8099 - Open
> • 127.0.0.1:8100 - Open
```

## Benchmarks

```bash
time cargo run -- 127.0.0.1:3000 -r 65535

Are you sure you want to scan on 127.0.0.1:3000 from port 3000 to 65535? (y/n): y
Scanning on 127.0.0.1:3000
Open ports:
• 127.0.0.1:5432
• 127.0.0.1:8080
• 127.0.0.1:47786
• 127.0.0.1:53708
cargo run -- 127.0.0.1:3000 -r 65535  5,14s user 10,79s system 165% cpu 9,601 total
```
