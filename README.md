# Ethereum Gas estimator

This is a demo project which uses Ethereum RPC node `eth_estimateGas` call to estimate transaction gas.

Application starts HTTP server which exposes simple API:
 - `/estimate` - main functionality
 - `/version` - returns application version
 - `/` - provides simple web page which can be used to invoke `/estimate` API calls

### How to run

It is enough to compile and execute using following command in the main applicatino folder: `cargo run --release`.

Application prints logs to the standard output.

### Configuration

Default configuration uses Flashbots RPC node (https://rpc.flashbots.net/fast) and starts server on http://localhost:3000/.

It is possible to provide custom configuration by creating `gas_estimator.cfg` file in the same folder as application executable. Example configuration file content:
```
{
    "rpc_url": "https://rpc.flashbots.net/fast",
    "bind_addr": "0.0.0.0:80"
}
```

### Using `/estimate` API

```
curl -X POST http://localhost:3000/estimate \
  -H "Content-Type: application/json" \
  -d '{"to":"0xabc1234567890abc1234567890abc1234567890f", 
       "from":"0xabc1234567890abc1234567890abc1234567890f", 
        "data":"0xd0e30db0"}' 
```


