# bronotdsaurs

- It is currently scoped for multi-protocol support.

> [!WARNING]
> The public API is unstable and will change frequently. Expect breaking changes between any two commits until v1.0.0. Pin to a specific git revision if you wish to depend on it.

> [!WARNING]
> This project is functional in that the core TDS flows are implemented but it should be considered "early development". However, please consider this project to be not actively maintained; I may continue adding features, triage issues, review PRs, and fix bugs when I feel like it, but it shouldn't be read as a commitment. I've published it in the hopes that someone finds it useful. If you'd like to maintain it, open an issue to chat or just fork it.

# Layout

```
 rustds/
 |--- crates/
 |    |--- bronotdsaurs/          # TDS protocol implementation
 |    |--- interface/             # unified database traits (Connection, Rows, Row)
 |    |--- fedauth/               # federated authentication
 |    |--- derive_proc_macros/    # procedural macros for type conversions
 |--- foundation/
 |    |--- collections/           # hybrid stack/heap buffer (SmallBytes<N>)
 |    |--- traits/                # core Encoder/Decode traits
 |    |--- transport/             # network I/O and TLS abstraction
 |    |--- plugins/               # extensions interface for capabilities requiring external libraries
```