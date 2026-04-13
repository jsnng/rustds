# Tabular Data Stream Protocol

- a zero-alloc (span-based) by default and allocation on demand via [Decode](./decoder/traits.rs) trait, `no_std`-compatible pure-rust implementation of Microsoft's tabular data stream protocol.
- Transport-agnostic; operates purely at the protocol layer. Implement the `Transport` trait to use it over TCP, TLS, MARS, or any async runtime.
- Synchronous by default; async runtimes are supported via the `Transport` trait.
- Intentionally kept as dependency-free as possible.

> [!WARNING]
> This crate uses `unsafe` in performance-critical paths (e.g. the streaming decode buffer). These are documented and minimal, but this crate is not `#![forbid(unsafe_code)]`.

# Example

### 1. Implement `Transport`
```rust
struct TcpTransport(TcpStream);

impl Transport for TcpTransport {
    type Error = std::io::Error;
    // ...
}
```
### 2. Connect
This implementation manages the session lifecycle via the **typestate pattern** to enforce transitions at compile time.
```rust
let tcp = TcpStream::connect_timeout(&addr, Duration::from_secs(10))?;
let session: Session<InitialState, TcpStream, _> = Session::new(tcp, observer);
// .. do perlogin, login7 and query
```

### 3. Pre-Login/Login

```rust
let prelogin = PreLoginPacketBuilder::default()
  .encryption(PreLoginEncryptionOptions::NotSupported as u8)
  .build()
  .unwrap();

let transition = session.transition(prelogin)?;
match transition {
  InitialStateTransition::LoginReady(s) => {
    let login7 = Login7PacketBuilder::default()
    .user_name("sa".to_string())
    .password("password".to_string())
    .server_name("localhost".to_string())
    .build()?;
    let session = s.transition(login7)?;
  }
  #[cfg(feature = "tls")]
  InitialStateTransition::TlsSslNegotiation(s) => {
    // TLS
  }
}
```

### 4. Query
Here is a `sql_batch` example.
```rust
let sql_batch = SQLBatchBuilder::default()
    .all_headers(AllHeaders::new(vec![/* DataStreamHeaderType */]))
    .sql_text("SELECT * FROM sys.tables".to_string()) 
    .build()?;

session.query(
  batch,
  |col_metadata, rows| { /* col_metadata iterator to perform rows decoding */ }
)?;
```

or `rpc` example:

```rust
let rpc = SpExecuteSqlBuilder::default()
    .stmt("SELECT @p1".to_string())
    .into_rpc_batch(AllHeaders::new(vec![]));

session.send(rpc)?;  
session.receive(
  batch,
  |col_metadata, rows| { /* col_metadata iterator to perform rows decoding */ }
)?;
```

# Features Flags
### TDS Version
Use the following features to target a specific TDS version.

| Flag | Version |
| --- | --- |
| `"tds7.0"` | 7.0 |
| `"tds7.1"` | 7.1 |
| `"tds7.2"` | 7.2 |
| `"tds7.3"` | 7.3 |
| `"tds7.3a"` | 7.3a |
| `"tds7.3b"` | 7.3b |
| `"tds7.4"` | 7.4 |
| `"tds8.0"` | 8.0 |

### MARS
Enable Session Multiplex Protocol (MARS) support:

| Flag | Description |
| --- | --- |
| `"smp"` | MARS support |
| `"smp2.2"` | MARS 2.2 (implies `"smp"`) |

### Chrono
Enable `chrono` integration for DateTime types:

  - `"chrono"`

### no-std
By default, this crate is `no_std`. To enable `std`, use the feature flag:

  - `"std"`

# Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
rustds = { git = "https://github.com/jsnng/rustds.git", features = ["tds7.4"] }
```

# Building

```bash
cargo build
```

### Shared Library (with `std`)
Add to `Cargo.toml`:
```toml
[lib]
crate-type = ["cdylib"]
```
Then run the following command:
```bash
cargo build --release --features std
```

### Shared Library (with `no_std`)
```
#TODO
```
# Layout

```
 rustds/
 |--- src/
 |    |--- interface/           # impl of unified database layer
 |    |--- smp/                 # "Session Multiplex Protocol" for MARS
 |    |--- tds/
 |    |    |--- session/        # session lifecycle and state machine
 |    |    |--- decoder/        # parses incoming bytes from a MSSQL server into types
 |    |    |--- encoder/        # serialises types into bytes for transport
 |    |    |--- fmt/            # value formatters (datetime, money, guid, decimal)
 |    |    |--- plugins/        # federated auth plugin interface
 |    |    |--- types/          # TDS type definitions
 |    |    |    |--- tokens/    # streaming token definitions
 |    |    |    |--- sp/        # stored procedures (for rpc.rs)
 |    |--- lib.rs
```

# Status

### Protocol
| Feature | Status |
| --- | --- |
| Pre-Login / Login7 | Done |
| SQL Batch | Done |
| RPC / Stored Procs | Done |
| Streaming Receive (zero-alloc) | Done |
| Bulk Copy (BCP, NBC rows) | Done |
| TLS/SSL Negotiation | Done |
| Environment Change Tracking | Done |
| PLP (Partially Length-Prefixed) | Done |
| Return Status / Return Value | Done |
| Formatters (DateTime, Money, GUID, Decimal) | Done |
| FeatureExtAck (TDS 7.4) | Done |

### Token Decoding
| Token | Status |
| --- | --- |
| ColMetaData, Row, NbcRow (7.3b) | Done |
| Done, DoneProc, DoneInProc | Done |
| EnvChange, Error, Info, LoginAck | Done |
| ReturnStatus, ReturnValue | Done |
| Order, TabName, ColInfo | Done |
| FeatureExtAck (7.4) | Done |
| AltMetaData, AltRow | Stub (`todo!()`) |
| DataClassification (7.4) | Stub (`todo!()`) |
| SessionStatus (7.4) | Stub (`todo!()`) |
| FedAuthInfo, SSPI | Stub |

### Data Type Decoding
| Type | Status |
| --- | --- |
| Fixed-length (Int1-8, Bit, Flt4/8, Money, DateTime) | Done |
| Variable BYTELEN (Guid, IntN, BitN, DecimalN, FltN, MoneyN, DateTimN) | Done |
| Variable USHORTLEN (BigVarChar, NVarChar, BigBinary, NChar) | Done |
| Variable LONGLEN (Image, Text, NText) | Done |
| DateN, TimeN, DateTime2N, DateTimeOffsetN (7.3) | Walk only (no value decoder) |
| SsVariant (7.2) | Walk only (no value decoder) |
| JSON, XML, Vector, UDT | Not in DTYPE_LUT |

### Authentication
| Feature | Status |
| --- | --- |
| SQL Server Auth | Done |
| Federated Auth | Untested (plugin trait exists, no plugin impl) |
| SSPI / Kerberos | Stub (type definitions only) |

### Incomplete / Missing
| Feature | Status |
| --- | --- |
| TDS 7.4 Pre-Login Extensions | Partial (11/12 feature options encoded, 1 `todo!()`) |
| TDS 8.0 | Stub (feature guards only, `todo!()` in session transition) |
| JSON / XML Decode | Missing (type defined, no decoder) |
| MARS (SMP) | Partial (header encode only, no session management) |
| Legacy Data Types | Feature-gated, not enabled by default |
| Always Encrypted | Partial (type + encoder, no decryption) |
| Attention Signals | Partial (encoder only, no response handling) |
| TVP (Table-Valued Parameters) | Partial (type name/metadata done, row iteration incomplete) |
| Tests / Kani Proofs | Partial (~12 kani proofs, ~38 unit tests, many stubs) |
| Documentation | Incomplete |

