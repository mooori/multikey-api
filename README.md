# multikey-api
Isolates the issue of deserializing `QueryVec` in `warp`.

# Usage
Start the server:
```
cargo run
```

Make a request:
```
http://localhost:3030/hello?id=1&pets=dog
```

Receive the deserialized query in the response:
```
FooQuery { id: 1, pets: QueryVec(["dog"]) }
```
