use aes::Aes128;
use squint::Id;

mod macros;

macros::run_benchmarks!(new_id, reveal, to_string, parse, encode, decode);

fn new_id<const TAG: u64>(id: i64, cipher: &Aes128) -> impl Fn() -> Id<TAG> + '_ {
    move || Id::<TAG>::new(id, cipher)
}

fn reveal<const TAG: u64>(id: i64, cipher: &Aes128) -> impl Fn() -> squint::Result<i64> + '_ {
    let id = Id::<TAG>::new(id, cipher);
    move || id.reveal(cipher)
}

fn to_string<const TAG: u64>(id: i64, cipher: &Aes128) -> impl Fn() -> String + '_ {
    let id = Id::<TAG>::new(id, cipher);
    move || id.to_string()
}

fn parse<const TAG: u64>(id: i64, cipher: &Aes128) -> impl Fn() -> squint::Result<Id<TAG>> + '_ {
    let id = Id::<TAG>::new(id, cipher).to_string();
    move || id.parse()
}

fn encode<const TAG: u64>(id: i64, cipher: &Aes128) -> impl Fn() -> String + '_ {
    move || Id::<TAG>::new(id, cipher).to_string()
}

fn decode<const TAG: u64>(id: i64, cipher: &Aes128) -> impl Fn() -> squint::Result<i64> + '_ {
    let id = Id::<TAG>::new(id, cipher).to_string();
    move || id.parse().and_then(|id: Id<TAG>| id.reveal(cipher))
}
