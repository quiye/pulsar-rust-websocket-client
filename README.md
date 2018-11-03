# Rust Pulsar Producer Sample

[![Build Status](https://travis-ci.com/quiye/pulsar-rust-websocket-client.svg?branch=master)](https://travis-ci.com/quiye/pulsar-rust-websocket-client)

This is [Pulsar](https://pulsar.apache.org) producer sample code, an analogy of [Python producer code](https://pulsar.apache.org/docs/en/client-libraries-websocket/#python-producer).

## How To Run Sample Code

### Run Pulsar Broker

```sh
$ docker run -it \
  -p 6650:6650 \
  -p 8080:8080 \
  -v $PWD/data:/pulsar/data \
  apachepulsar/pulsar:2.2.0 \
  bin/pulsar standalone
```

### Produce

```sh
$ git clone https://github.com/quiye/pulsar-rust-websocket-client.git
$ cd pulsar-rust-websocket-client
$ cargo run
```

then,

```
Got message: {"result":"ok","messageId":"CHcQAg==","context":"5"}
```

### Consume

TODO
