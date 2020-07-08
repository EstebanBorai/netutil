<div align="center">
  <img src="https://camo.githubusercontent.com/734a3468bce992fbc3b729562d41c92f4912c99a/68747470733a2f2f7777772e727573742d6c616e672e6f72672f7374617469632f696d616765732f727573742d6c6f676f2d626c6b2e737667" height="120" width="120" />
  <h1>whiff</h1>
  <small>📡 Port Sniffer Utility</small>
</div>

<hr />
<div align="center">
  <img src="https://thumbs.gfycat.com/PartialFakeEnglishpointer-size_restricted.gif" alt="dog sniff" width="500" height="281" />
</div>
<p align="center">A nice friend sniffing your screen</p>
<hr />

## Getting Started

```bash
git clone https://github.com/estebanborai/whiff.git

cd whiff/

cargo run -- -t 127.0.0.1 -r 2600
```

## Help Command Output

```
whiff 0.1.0
Esteban Borai <estebanborai@gmail.com> (https://github.com/estebanborai)

USAGE:
    whiff [OPTIONS] --target <TARGET IP>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --range <RANGE PORTS>    Range of ports to test from 0
    -t, --target <TARGET IP>     Target host to connect
```
