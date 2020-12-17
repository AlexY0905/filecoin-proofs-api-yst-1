#![deny(clippy::all, clippy::perf, clippy::correctness)]
#![allow(clippy::too_many_arguments)]
//requires nightly, or later stable version
//#![warn(clippy::unwrap_used)]

pub mod post;
pub mod seal;

mod registry;
mod types;

pub use crate::registry::{RegisteredPoStProof, RegisteredSealProof, Version};
pub use crate::types::{PrivateReplicaInfo, PublicReplicaInfo};

pub use filecoin_proofs::storage_proofs::error::Error as StorageProofsError;
pub use filecoin_proofs::storage_proofs::fr32;
pub use filecoin_proofs::storage_proofs::post::election::Candidate;
pub use filecoin_proofs::storage_proofs::sector::{OrderedSectorSet, SectorId};
pub use filecoin_proofs::types::{
    ChallengeSeed, Commitment, PaddedBytesAmount, PieceInfo, PoStType, ProverId, Ticket,
    UnpaddedByteIndex, UnpaddedBytesAmount,
};
pub use filecoin_proofs::{FallbackPoStSectorProof, SnarkProof, VanillaProof};
