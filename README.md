# grpc-calculator using Tonic

You must have the `protoc` Protocol Buffers compiler
installed, along with the Protocol Buffers resource files.

Ubuntu:
```bash
sudo apt update && sudo apt upgrade -y
sudo apt install -y protobuf-compiler libprotobuf-dev
```


## grpc Calculator

### Build

```bash
$ cargo build
```

### Server

```bash
$ cargo run --bin server
```