mod chk;
mod chk2;
mod chunk_name;
mod parsed_chk;
mod riff;
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
pub use chk::ParsedChunk;
pub use chunk_name::ChunkName;

pub use parsed_chk::ParsedChk;

pub use parsed_chk::parse_chk_full;

#[cfg(feature = "full")]
pub use trig::*;

#[cfg(feature = "full")]
pub use chk::get_string;
#[cfg(feature = "full")]
pub use mpq::get_chk_from_mpq_filename;
#[cfg(feature = "full")]
pub use mpq::get_chk_from_mpq_in_memory;
