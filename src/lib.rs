mod chk;
mod chk2;
mod util;

#[cfg(feature = "full")]
mod mpq;
#[cfg(feature = "full")]
mod trig;

#[cfg(test)]
mod test;

pub use chk::get_all_string_references;

pub use chk::merge_raw_chunks;
pub use chk::parse_chk;
pub use chk::parse_merged_chunks;
pub use chk::ChunkName;
pub use chk::ParsedChunk;

#[cfg(feature = "full")]
pub use trig::*;

#[cfg(feature = "full")]
pub use chk::get_string;
#[cfg(feature = "full")]
pub use mpq::get_chk_from_mpq_filename;
#[cfg(feature = "full")]
pub use mpq::get_chk_from_mpq_in_memory;
