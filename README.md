# rustds

- It is currently scoped for multi-protocol support.

> [!WARNING]
> This project is functional in that the core TDS flows are implemented but it should be considered "early development". However, please consider this project to be not actively maintained; I may continue adding features when I feel like it, but it shouldn't be read as a commitment. I've published it in the hopes that someone finds it useful.
# Layout

```
 rustds/
 |--- crates/
 |    |--- rustds/                # TDS protocol implementation
 |    |--- interface/             # unified database traits (Connection, Rows, Row)
 |    |--- fedauth/               # federated authentication
 |    |--- derive_proc_macros/    # procedural macros for type conversions
 |--- foundation/
 |    |--- collections/           # hybrid stack/heap buffer (SmallBytes<N>)
 |    |--- traits/                # core Encoder/Decode traits
 |    |--- transport/             # network I/O and TLS abstraction
 |    |--- plugins/               # extensions interface for capabilities requiring external libraries
```