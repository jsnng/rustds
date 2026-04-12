# rustds

- It is currently scoped for multi-protocol support.

> [!WARNING]
> This project is in early development and not production-ready.

# Layout

```
 rustds/
 |--- crates/
 |    |--- rustds/                # TDS protocol implementation
 |    |--- interface/             # unified database traits (Connection, Rows, Row)
 |    |--- fedauth/               # federated authentication
 |    |--- derive_proc_macros/    # procedural macros for type conversions
 |--- foundation/
 |    |--- collections/           # hybrid stack/heap buffer (BufRef<N>)
 |    |--- traits/                # core Encoder/Decode traits
 |    |--- transport/             # network I/O and TLS abstraction
 |    |--- plugins/               # extensions interface for capabilities requiring external libraries
```