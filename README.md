### dns-lookup-service

This is a pet project to help perform DNS requests across a number of different providers.  The program can be run as a service or as a command line program.

### Running as a Service

To run as a service, run the following command:

```
cargo build
cd target/debug/
./dns-lookup-service --iron_server --log_config ../../examples/sample-log4rs.yaml --config ../../examples/sample-config.toml

```

In a separate terminal:

```
curl http://127.0.0.1:8989/google.com

```

The results will look like the following:

```
{"results":[
    {"source":"cleanbrowsing","name":"google.com.","address":"172.217.6.46","atype":"ip4","time_ms":38},
    {"source":"cleanbrowsing","name":"google.com.","address":"2607:f8b0:4005:809::200e","atype":"ip6","time_ms":40},
    {"source":"cloudflare","name":"google.com.","address":"172.217.15.78","atype":"ip4","time_ms":17},
    {"source":"cloudflare","name":"google.com.","address":"2607:f8b0:4004:810::200e","atype":"ip6","time_ms":18},
    {"source":"comodo","name":"google.com.","address":"172.217.17.238","atype":"ip4","time_ms":53},
    {"source":"comodo","name":"google.com.","address":"2607:f8b0:4006:814::200e","atype":"ip6","time_ms":51},
    {"source":"connectsafe","name":"google.com.","address":"172.217.1.238","atype":"ip4","time_ms":20},
    {"source":"connectsafe","name":"google.com.","address":"2607:f8b0:4000:80b::200e","atype":"ip6","time_ms":20},
    {"source":"google","name":"google.com.","address":"172.217.2.238","atype":"ip4","time_ms":26},
    {"source":"google","name":"google.com.","address":"2607:f8b0:4000:806::200e","atype":"ip6","time_ms":27},
    {"source":"opendns","name":"google.com.","address":"172.217.14.174","atype":"ip4","time_ms":16},
    {"source":"opendns","name":"google.com.","address":"2607:f8b0:4000:806::200e","atype":"ip6","time_ms":23},
    {"source":"quad9","name":"google.com.","address":"216.58.218.174","atype":"ip4","time_ms":15},
    {"source":"quad9","name":"google.com.","address":"2607:f8b0:4000:80c::200e","atype":"ip6","time_ms":16},
    {"source":"yandex","name":"google.com.","address":"216.239.38.120","atype":"ip4","time_ms":161},
    {"source":"yandex","name":"google.com.","address":"2a00:1450:400f:809::200e","atype":"ip6","time_ms":159}
    ]
}
```

The results are stored in a `results` json array, and each element includes the name of the `source`, the `name` looked-up,
the `atype` (address type `ip4` or `ip6`), and the `time_ms` (time in milliseconds).

