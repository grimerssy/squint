# Squint

[<img alt="github" src="https://img.shields.io/badge/github-grimerssy/squint-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/grimerssy/squint)
[<img alt="crates.io" src="https://img.shields.io/crates/v/squint.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/squint)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-squint-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/squint)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/grimerssy/squint/ci.yaml?branch=main&style=for-the-badge" height="20">](https://github.com/grimerssy/squint/actions?query=branch%3Amain)

Squint is a library for _encoding integers as unique deterministic strings_.

It is expected to be used for encoding database IDs as random strings to get
fast indexed database lookups and hide actual IDs from the end users.

Library also provides an easy way to introduce different ID types
(i.e. `UserId(1)` shouldn't be equal to `CrateId(1)`
even though the underlying integer value is the same).

# Usage

Basic example

```rust
use squint::aes::{cipher::KeyInit, Aes128};

type Id = squint::Id<0>;

let key = [0; 16];
let cipher = Aes128::new(&key.into());

let id = Id::new(1, &cipher);
let encoded = id.to_string();
assert_eq!("xZV3JT8xVMefhiyrkTsd4T2", &encoded);

let decoded = encoded
    .parse::<Id>()
    .and_then(|id| id.reveal(&cipher))
    .unwrap();
assert_eq!(decoded, 1);
```

Different ID types

```rust
use squint::{
    aes::{cipher::KeyInit, Aes128},
    tag, Id,
};

type UserId = Id<{ tag("user") }>;

type CrateId = Id<{ tag("crate") }>;

let key = [0; 16];
let cipher = Aes128::new(&key.into());

let user_id = UserId::new(1, &cipher);
let crate_id = CrateId::new(1, &cipher);

assert_eq!("qXfXkNN9ReZCGXu3qi28xC2", &user_id.to_string());
assert_eq!("VgtE1tzjDEHnjd3fh3PwiT2", &crate_id.to_string());
```

# Comparison

[UUID(v4)](https://crates.io/crates/uuid)

##### Pros

-   The most adopted standard for public resource IDs

##### Cons

-   Not sequential, hence slower database inserts

---

[Cuid](https://crates.io/crates/cuid) and
[NanoID](https://crates.io/crates/nanoid)
are similar to UUID relative to this crate

---

[ULID](https://crates.io/crates/ulid)

##### Pros

-   Lexicographically sortable
-   Compatible with UUID

##### Cons

-   Contain creation timestamps

---

[Sqids](https://crates.io/crates/sqids)

##### Pros

-   Can encode multiple numbers in one ID
-   Enable use of auto-incrementing database primary keys

##### Cons

-   Increased code complexity
-   Can be decoded revealing ID count
-   No built-in solution to ID reuse across entities

---

[This crate](https://crates.io/crates/squint)

##### Pros

-   Enable use of auto-incrementing database primary keys
-   Cryptographically secure

##### Cons

-   Increased code complexity
