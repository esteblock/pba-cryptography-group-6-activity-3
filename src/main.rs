mod p1_hashing;
mod p2_addresses;

use std::str::FromStr;

use byteorder::{LittleEndian, ReadBytesExt};
use hex::ToHex;
use phf::phf_map;
use sp_core::ecdsa::{Pair as ECDSAPair, Public as ECDSAPublic};
use sp_core::ed25519::{Pair as Ed25519Pair, Public as Ed25519Public};
use sp_core::sr25519::{Pair as Sr25519Pair, Public as Sr25519Public};
use sp_core::{
    crypto::{Derive, DeriveJunction, Ss58Codec},
    Pair, Public,
};
use rand::{Rng, rngs::SmallRng, SeedableRng};


pub fn generate_ed25519_pair(bip39_str: &str) -> Ed25519Pair {
    Ed25519Pair::from_phrase(bip39_str, None).unwrap().0
}

fn main() {

    const SEED1: &str = "lunar antique thank school space garden soda cigar glove despair master pumpkin";
    const SEED2: &str = "parade method erupt shock bacon wait follow limb onion laugh exhibit whip";

    let player_1 = generate_ed25519_pair(SEED1);
    let player_2 = generate_ed25519_pair(SEED2);

    let mut rng_0 = SmallRng::seed_from_u64(0);
    let mut rng_1 = SmallRng::seed_from_u64(1);
    let randomness_0: [u8; 4] = rng_0.gen();
    let randomness_1: [u8; 4] = rng_1.gen();

    dbg!(randomness_0);
    dbg!(randomness_1);




    todo!()
}
/*
Players start with 100 points each
VRF scheme
commit reveal
from rand number get number from 1 to 52 (card)
from 1 to 52 find number from 1 to 13 (mod) (car carcd)
from this fid bet (linear function)


strcut
Player
- points
- current card
- random number


*/

