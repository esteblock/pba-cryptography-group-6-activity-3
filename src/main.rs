mod p1_hashing;
mod p2_addresses;

use std::cmp::min;
use std::str::FromStr;

use byteorder::{LittleEndian, ReadBytesExt};
use hex::ToHex;
use phf::phf_map;
use sp_core::ecdsa::{Pair as ECDSAPair, Public as ECDSAPublic};
use sp_core::ed25519::{Pair as Ed25519Pair, Public as Ed25519Public, Signature as Ed25519Signature};
use sp_core::sr25519::{Pair as Sr25519Pair, Public as Sr25519Public};
use sp_core::{
    crypto::{Derive, DeriveJunction, Ss58Codec},
    Pair, Public,
};
use rand::{Rng, rngs::SmallRng, SeedableRng, rngs::OsRng};
use p1_hashing::{hash_with_blake};
use schnorrkel::{Keypair,Signature};
use sp_runtime::traits::Verify;

pub fn generate_ed25519_pair(bip39_str: &str) -> Ed25519Pair {
    Ed25519Pair::from_phrase(bip39_str, None).unwrap().0
}

fn main() {

    let mut points_0 =100;
    let mut points_1 =100;

    const SEED1: &str = "lunar antique thank school space garden soda cigar glove despair master pumpkin";
    const SEED2: &str = "parade method erupt shock bacon wait follow limb onion laugh exhibit whip";

    let player_0 = generate_ed25519_pair(SEED1);
    let player_1 = generate_ed25519_pair(SEED2);

    // let player_0: Keypair = Keypair::generate_with(OsRng);
    // let player_1: Keypair = Keypair::generate_with(OsRng);
    // player_0.secret;

    let mut rng_0 = SmallRng::seed_from_u64(0);
    let mut rng_1 = SmallRng::seed_from_u64(1);
    let randomness_0: [u8; 4] = rng_0.gen();
    let randomness_1: [u8; 4] = rng_1.gen();

    // dbg!(randomness_0);
    // dbg!(randomness_1);

    // The random values have been shared now
    let commited_hash_0=hash_with_blake(&randomness_0);
    let commited_hash_1=hash_with_blake(&randomness_1);

    // now the random are shared
    let revealed_0 = randomness_0;
    let revealed_1 = randomness_1;

    // now the random are verified from the commitments
    match hash_with_blake(&revealed_0) == commited_hash_0 {
        true => println!("oh yea"),
        false => panic!("hes cheating")

    }
    // now the random are verified from the commitments
    match hash_with_blake(&revealed_1) == commited_hash_1 {
        true => println!("oh yea"),
        false => panic!("hes cheating")
    }

    // we will generate a comon ramdon suming the two revealed
    let value_0 = u32::from_le_bytes(revealed_0);
    let value_1 = u32::from_le_bytes(revealed_1);
    let sum = value_0 as u64 + value_1 as u64;
    let common_random = sum.to_le_bytes();

    // now we will use the VRF from each player to evaluate in the common random
    let proof_0 = player_0.sign(&common_random);
    let proof_1 = player_1.sign(&common_random);

   
    // From their putpuit they compute and calculate the 
    let vrf_output_0 = (hash_with_blake(&proof_0.0)[0] % 52 ) % 13;
    let vrf_output_1 = (hash_with_blake(&proof_1.0)[0] % 52 ) % 13;

    // THEY CALCULATE THEIR BET,
    let bet_0 = ((vrf_output_0 as f32) / 12.0 * 100.0) as u32;
    let bet_1 = ((vrf_output_1 as f32) / 12.0 * 100.0) as u32;

    let common_bet = min(bet_0, bet_1);

    // they now exchange the signatures, exchange them and verify their output and they verify\
    match proof_0.verify(&common_random[..], &player_0.public()) {
        true => println!("oh yea"),
        false => panic!("hes cheating")
    };
    match proof_1.verify(&common_random[..], &player_1.public()){
        true => println!("oh yea"),
        false => panic!("hes cheating")
    }
    
    // now they pay.
    match vrf_output_0 > vrf_output_1 {
        true => {
            // 0 WON
            if common_bet > points_1 {
                panic!("PLAYER 0 WINS")
            }
            points_0 += bet_1;
            points_1 -= bet_1;
        },
        false => {
            if common_bet > points_0 {
                panic!("PLAYER 1 WINS")
            }
            points_0 -= bet_0;
            points_1 += bet_0;
        }
    }






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

