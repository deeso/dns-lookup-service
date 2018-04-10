### dns-lookup-service description

This is a pet project to help perform DNS requests across a number of different providers.  The program will query a set of domain name servers measuring the time and capturing all the answers in the A and AAAA records.  The program can be run as a service, in __Docker__, or as a command line program.

### Running as a Service

To run as a service, run the following command:

```
cargo build
cd target/debug/
./dns-lookup-service --iron_server \
                     --log_config ../../examples/sample-log4rs.yaml \
                     --config ../../examples/sample-config.toml

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

If the program is run as a service, the listen port can be specified on the command line or in the config file.

### Running as a Docker Service

Optionally, the service can be run using __Docker__.  Assuming you are in the root directory. Copy the configs for the service and the logging into the `configs` directory as `config.toml` and `log_config.yaml`, respectively.  Then `cd` into `docker/dns-lookup-service` and execute the `run-file.sh`.  Here are the commands in sequence:

```
cp examples/samples-config.toml config.toml
cp examples/samples-log4rs.toml log_config.yaml
cd docker/dns-lookup-service/
sh run-file.sh

curl http://127.0.0.1:8989/google.com

```  

### Running as a Command-line

To run as a command line, run the following command:

```
cargo build
cd target/debug/
./dns-lookup-service --log_config ../../examples/sample-log4rs.yaml \
                     --config ../../examples/sample-config.toml \
                     --hostname www.google.com

```

The results will be outputted and look like the following:

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

