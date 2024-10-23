# Interplex

A high performance cache written in Rust using grpc.

## Test

`RUST_LOG=debug cargo run` in a terminal session to start interplex

In another terminal tab, execute a set/get:

```
grpcurl --plaintext -d '{"key": "1", "value": "2"}' localhost:8080 schema.v1.CacheService/Set
```

```
❯ grpcurl --plaintext -d '{"key": "1" }' localhost:8080 schema.v1.CacheService/Get
{
  "value": "2"
}
```