extern crate fst;
extern crate itertools;
extern crate memmap;
extern crate byteorder;
extern crate regex;

#[macro_use]
extern crate failure as failure;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

#[macro_use]
pub use failure::Error as Error;

mod prefix;
pub use prefix::PrefixSet;
pub use prefix::PrefixSetBuilder;

pub mod fuzzy;
pub use fuzzy::FuzzyMap;
pub use fuzzy::FuzzyMapBuilder;

pub mod phrase;

pub use phrase::PhraseSet;
pub use phrase::PhraseSetBuilder;
pub use phrase::query::QueryPhrase;
pub use phrase::query::QueryWord;

pub mod glue;

pub mod errors;
