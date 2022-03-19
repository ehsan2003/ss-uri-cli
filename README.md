# ss-uri-cli
shadowsocks uri into ss-local config based on [ss-uri](https://crates.io/crates/ss-uri)

a shadowsocks uri helper cli tool 
## installation 
```
cargo install ss-uri-cli
```
> remember you should have cargo installed before this command

## usage
```
$ ss-uri-cli ss://YWVzLTEyOC1nY206dGVzdA@192.168.100.1:8888#Foo%20Bar --port 1080 > ss-local.json

$ ss-local -c ss-local.json
```

## help
```
ss-uri 0.1.0
cli shadowsocks uri parser based on https://crates.io/crates/ss-uri

USAGE:
    ss-uri [OPTIONS] <URI>

ARGS:
    <URI>    shadowsocks uri to parse ( could be either sip002 uri or legacy base64 uri)

OPTIONS:
    -h, --help           Print help information
    -p, --port <PORT>    local port for ss-local config [default: 1080]
    -V, --version        Print version information
```
