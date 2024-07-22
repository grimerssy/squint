# Squint

Squint is a library for _encoding integers as unique deterministic strings_.

It is expected to be used for encoding database IDs as random strings to get
fast indexed database lookups and hide actual IDs from the end users.

Library also provides an easy way to introduce different ID types
(i.e. `UserId(1)` shouldn't be equal to `CrateId(1)`
even though the underlying integer value is the same).

# Usage

Basic example

```rust
use aes::{cipher::KeyInit, Aes128};
use squint::Id;

let key = [0; 16];
let cipher = Aes128::new(&key.into());

let id: Id = Id::new(1, &cipher);
let encoded = id.to_string();
assert_eq!(&encoded, "xZV3JT8xVMefhiyrkTsd4T");

let decoded = encoded
    .parse()
    .and_then(|id: Id| id.to_raw(&cipher))
    .unwrap();
assert_eq!(decoded, 1);
```

Different ID types

```rust
use aes::{cipher::KeyInit, Aes128};
use squint::{tag, Id};

type UserId = Id<{ tag("user") }>;

type CrateId = Id<{ tag("crate") }>;

let key = [0; 16];
let cipher = Aes128::new(&key.into());

let user_id = UserId::new(1, &cipher);
let crate_id = CrateId::new(1, &cipher);
// comparing IDs directly causes a compilation error
assert_ne!(&user_id.to_string(), &crate_id.to_string());
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
-   Can be quite short, though unlikely
