#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

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

/// A seed phrase for a key according to the BIP39 standard
const BIP39_STR: &str = "source upgrade van toy cross smooth write erupt uncover today injury say wealth silk thought slide shadow comfort hazard planet wisdom problem review pudding";

/// The SS58 address corresponding to the above seed phrase
const SS58_ADDRESS: &str = "5GEkFD1WxzmfasT7yMUERDprkEueFEDrSojE3ajwxXvfYYaF";

// For the following functions, it will be helpful to figure out how to do them using both code
// and the subkey tool. Additionally, reading through the subkey documentation may be helpful.
// The documentation is here: https://docs.substrate.io/reference/command-line-tools/subkey/
// with installation instructions: https://docs.substrate.io/reference/command-line-tools/subkey/#installation

/// Generate the sr25519 keypair corresponding to the const bip39 phrase
pub fn generate_sr25519_pair() -> Sr25519Pair {
    Sr25519Pair::from_phrase(BIP39_STR, None).unwrap().0
}

/// Generate the ed25519 keypair corresponding to the const bip39 phrase
pub fn generate_ed25519_pair() -> Ed25519Pair {
    Ed25519Pair::from_phrase(BIP39_STR, None).unwrap().0
}

/// Generate the ecdsa keypair corresponding to the const bip39 phrase
pub fn generate_ecdsa_pair() -> ECDSAPair {
    ECDSAPair::from_phrase(BIP39_STR, None).unwrap().0
}

/// Generate a child keypair of the sr25519 keypair, with the derivation path "children" and
/// "0", where "children" is a hard derivation, and "0" is soft.
pub fn generate_derived_addresses_from_sr25519_pair() -> Sr25519Pair {
    let derivation_path_iter = vec![
        DeriveJunction::hard("children".as_bytes()), 
        DeriveJunction::soft(u32::to_le_bytes(0))
    ].into_iter();

    generate_sr25519_pair().derive(derivation_path_iter, None).unwrap().0
}

/// Generate a child keypair corresponding to the address passed in. The address is provided in
/// SS58 format, and the derivation path should be "test_derivation" and "5", with both being
/// soft derivations.
pub fn generate_derived_public_from_address(address: &str) -> Sr25519Public {
    let derivation_path_iter = vec![
        DeriveJunction::soft("test_derivation".as_bytes()), 
        DeriveJunction::soft(u32::to_le_bytes(5))
    ].into_iter();

	Sr25519Public::from_string(address).unwrap().derive(derivation_path_iter).unwrap()
}

/// Generate the substrate test pair corresponding to Alice in sr25519
pub fn alice_sr25519() -> Sr25519Pair {
    Sr25519Pair::from_string("//Alice", None).unwrap()
}

/// Generate the substrate test pair corresponding to Alice in ECDSA
pub fn alice_ecdsa() -> ECDSAPair {
    ECDSAPair::from_string("//Alice", None).unwrap()
}

/// Generate the sr25519 keypair corresponding to the const bip39 phrase using the password
/// 'hunter2'
pub fn generate_with_password() -> Sr25519Pair {
    Sr25519Pair::from_phrase(BIP39_STR, Some("hunter2")).unwrap().0
}

// Now that we have some familiarity with seeds, phrases, and password derivation, let's look a
// little into how seed phrases actually work! BIP39 uses a fixed english dictionary, and maps
// those words into specific bit sequences. BIP39 is designed to represent entropy in a
// human-readable form that is easy to remember.
//
// It also includes a checksum, so if you change one word in a seed phrase it won't work. Feel free
// to try it yourself! Generate a seed phrase with `subkey generate` and then try to change one word
// and inspect it with `subkey inspect $my_changed_phrase`.
//
// For this exercise, we will make our own tiny version of a seed phrase generator. We will only
// support 4-byte seeds, and have 16 possible words.
#[derive(Debug, PartialEq, Eq)]
pub struct TinySeed(u32);

/// This is a mapping from the 16 words in our dictionary to the hex character it represents.
/// We only have 16 words, so each word corresponds to a hex character, or 4 bits. BIP39 has
/// 2048 words, so each word is 11 bits.
///
/// We will convert from words to bytes in the following fashion:
///     - Convert the list of words into their corresponding hex characters
///     - Interpret the list of hex characters as little-endian bytes
///
/// Hints:
///     - The `hex` crate provides nice functionality for hex encoding/decoding.
///     - `char::to_digit` might be useful.
static WORDS_TO_ENTROPY: phf::Map<&'static str, char> = phf_map! {
    "polkadot" => '0',
    "blockchain" => '1',
    "academy" => '2',
    "berkeley" => '3',
    "chancellor" => '4',
    "on" => '5',
    "brink" => '6',
    "of" => '7',
    "second" => '8',
    "bailout" => '9',
    "for" => 'a',
    "banks" => 'b',
    "not" => 'c',
    "your" => 'd',
    "keys" => 'e',
    "crypto" => 'f',
};

/// This is an, where the ith entry is the word representing i in the dictionary.
static ENTROPY_TO_WORDS: [&str; 16] = [
    "polkadot",
    "blockchain",
    "academy",
    "berkeley",
    "chancellor",
    "on",
    "brink",
    "of",
    "second",
    "bailout",
    "for",
    "banks",
    "not",
    "your",
    "keys",
    "crypto",
];

// I write this function here so I can re-use it in 2 exercices
pub fn bytes_vec_to_phrase(bytes_vec: Vec<u8>) -> String {

    // We construct the phrase comparing every char from the encoded hex with the
    // Every char that is result of the map of the list of entropy words
    let phrase: String = hex::encode(bytes_vec).chars()
    .flat_map(|char| {
        ENTROPY_TO_WORDS.iter()
            .filter(move |&&word| WORDS_TO_ENTROPY[word] == char)
            .map(|&word| word)
    })
    .collect::<Vec<&str>>()
    .join(" "); // we add spaces  and create the phrase as a string

    phrase
}


/// Convert a tiny seed to a phrase, based on the u32 interpreted as little endian bytes
pub fn seed_to_phrase(seed: TinySeed) -> String {
    //  First convert the seed (4 bytes u32 number) into an array of bytes (4 elements)
    // Then uses the function (that we wil reuse later)
    bytes_vec_to_phrase(u32::to_le_bytes(seed.0).to_vec())
}

pub fn words_to_bytes_vec(words: Vec<&str>) -> Result<Vec<u8>, ()> {
    // For every word, we find the hex char and create a list of hex characters
    let words_in_hex: String = words.iter()
        .map
        (|word| {
            match WORDS_TO_ENTROPY.get(word) {
                Some(&char) => Ok(char.to_string()),
                None => Err(()), // when we dont find a word in the map
            }
        })
        .collect::<Result<String, ()>>()?; // From this we expect rather an String or an () due to the error, but we pass the error

    // Then return the decoded in bytes
    Ok(hex::decode(words_in_hex).unwrap())
} 

/// Convert a phrase to a tiny seed. Errors if any words are not in the wordlist, or there is
/// the wrong number of words. This function should never panic.
pub fn phrase_to_seed(phrase: &str) -> Result<TinySeed, ()> {
    //First we need to create a vector of words:
    let words: Vec<&str> = phrase.split_whitespace().collect();
    if words.len()!= 8 {
        return Err(()) // if different thatn 8
    }
    let seed_in_bytes = words_to_bytes_vec(words)?; // we can return the error
    
    // Now we need to assume that the bytes are 4?
    // So we will convert it into a [u8; 4] vec
    // If not it will return an err
    let seed_in_bytes_vec = if seed_in_bytes.len() == 4 {
        Ok([seed_in_bytes[0], seed_in_bytes[1], seed_in_bytes[2], seed_in_bytes[3]])
    } else {
        Err(()) // double check
    };
    
    Ok(TinySeed(u32::from_le_bytes(seed_in_bytes_vec?)))

}

/// A trucated hash function over a u32. We only use 1 byte of the hash value as a the checksum
/// for our tiny seed conversions.
pub fn truncated_hash_u32(x: u32) -> u8 {
    sp_core::blake2_128(x.to_le_bytes().as_slice())[0]
}

/// Convert a tiny seed to a phrase, based on the u32 interpreted as little endian bytes. We
/// also append a 1 byte checksum to the end of the phrase, also in phrase form. The resulting
/// phrase should be 10 words long. The checksum should come from the function `truncated_hash_u32`.
pub fn seed_to_phrase_with_checksum(seed: TinySeed) -> String {
    let hash_u8 = truncated_hash_u32(seed.0);
    let first_part_phrase = seed_to_phrase(seed);
    let checksum_phrase = bytes_vec_to_phrase(vec![hash_u8].to_vec());

    first_part_phrase + &String::from(" ") + &checksum_phrase 
}

/// Convert a phrase which includes a checksum to a tiny seed. Errors if any words are not in
/// the wordlist, there is the wrong number of words, or the checksum is wrong.
///
/// This function should never panic.
pub fn phrase_to_seed_with_checksum(phrase: &str) -> Result<TinySeed, ()> {
    // First, check the checksum
    let words: Vec<&str> = phrase.split_whitespace().collect();
    // Get all the words but the last 2 to get the seed_phrase
    let seed_phrase =  words[..words.len() - 2].to_vec();
    let seed = phrase_to_seed(&seed_phrase.join(" "))?; // will forward the error

    // Get the last words to calculate the checksum hash 
    let checksum_phrase: Vec<&str>  = words.iter().rev().take(2).rev().cloned().collect();
    let checksum_bytes_vec = words_to_bytes_vec(checksum_phrase)?;
    // Checsum bytes vec should have only one element
    let checksum_byte = checksum_bytes_vec[0];
    let expected_checksum =  truncated_hash_u32(seed.0);

    if checksum_byte != expected_checksum{
        return Err(());
    }
    println!("checksum_byte {:?}", checksum_byte);
    println!("expected_checksum {:?}", expected_checksum);

    // let checksum_phrase = words[words.len()-3..words.len()-1];
    Ok(seed)
}

/// This function is not graded. It is just for collecting feedback.
/// On a scale from 0 - 100, with zero being extremely easy and 100 being extremely hard, how hard
/// did you find the exercises in this section?
pub fn how_hard_was_this_section() -> u8 {
    60
}

/// This function is not graded. It is just for collecting feedback.
/// About how much time (in hours) did you spend on the exercises in this section?
pub fn how_many_hours_did_you_spend_on_this_section() -> f32 {
    6.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_sr25519_pair_test() {
        assert_eq!(
            "5GEkFD1WxzmfasT7yMUERDprkEueFEDrSojE3ajwxXvfYYaF".to_string(),
            generate_sr25519_pair().public().to_ss58check()
        )
    }

    #[test]
    fn generate_ed25519_pair_test() {
        // You might expect that an ed25519 key from the same seed as an sr25519 key would be the
        // same, because they use the same underlying curve (hence the names). However, that is not
        // necessarily the case! Feel free to dig into the code to see where they differ. If you
        // can't find out where, or are confused by how to see, call over an instructor!
        assert_eq!(
            "5CViXS31EkSxFgY7c3PcncsM2TbmqnxjXoLLVuMJyqP6PTGp".to_string(),
            generate_ed25519_pair().public().to_ss58check()
        )
    }

    #[test]
    fn generate_ecdsa_pair_test() {
        assert_eq!(
            "KW5diveAeLbPDuQWMeped6kA7wbtzvX5hoH1ocLKhPwx2PkCZ".to_string(),
            generate_ecdsa_pair().public().to_ss58check()
        )
    }

    #[test]
    fn generate_derived_addresses_from_sr25519_pair_test() {
        assert_eq!(
            "5CFFJoP6vZokcgdcQWp8XvQ7FQMMrb9mYgJF32zVbHmQs895".to_string(),
            generate_derived_addresses_from_sr25519_pair()
                .public()
                .to_ss58check()
        )
    }

    #[test]
    fn generate_derived_public_from_address_test() {
        assert_eq!(
            "5GgZq5hswE9s7tEpnimAFcw3PFgVUvWHQAxHqVisdhLQbKPJ".to_string(),
            generate_derived_public_from_address(SS58_ADDRESS).to_ss58check(),
        )
    }

    #[test]
    fn alice_sr25519_test() {
        assert_eq!(
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
            alice_sr25519().public().to_ss58check(),
        )
    }

    #[test]
    fn alice_ecdsa_test() {
        assert_eq!(
            "KW39r9CJjAVzmkf9zQ4YDb2hqfAVGdRqn53eRqyruqpxAP5YL".to_string(),
            alice_ecdsa().public().to_ss58check(),
        )
    }

    #[test]
    fn generate_with_password_test() {
        assert_eq!(
            "5CrSG9W4XppN2CEENE6UMQcBXHBiMsyKgkH3Qqmr2aa39UXx".to_string(),
            generate_with_password().public().to_ss58check(),
        )
    }
}

#[cfg(test)]
mod optional_tests {
    use super::*;
    // If you're having trouble with these tests, make sure to check the endianness of your integers
    // and decoding! It can often be challenging to work with. The functions u32::from_le_bytes and
    // u32::to_le_bytes may be helpful.

    #[test]
    #[cfg_attr(not(feature = "optional-tests"), ignore)]
    fn seed_to_phrase_test() {
        let seed = TinySeed(762150966);
        let phrase = seed_to_phrase(seed);
        assert_eq!(
            "berkeley brink second polkadot brink your academy your",
            &phrase
        );
    }

    #[test]
    #[cfg_attr(not(feature = "optional-tests"), ignore)]
    fn phrase_to_seed_test() {
        let phrase = "chancellor of not your keys not your crypto";
        let seed = phrase_to_seed(phrase).unwrap();
        assert_eq!(TinySeed(3756838215), seed);
    }

    #[test]
    #[cfg_attr(not(feature = "optional-tests"), ignore)]
    fn phrase_to_seed_should_not_panic_test() {
        let phrase_6_words = "chancellor of not your keys not your";
        let phrase_9_words = "chancellor of not your keys not your crypto berkeley";
        let phrase_8_words_not_exit = "chancellor of not your keys not NOT EXIST";
        assert!(phrase_to_seed(phrase_6_words).is_err());
        assert!(phrase_to_seed(phrase_9_words).is_err());
        assert!(phrase_to_seed(phrase_8_words_not_exit).is_err());
    }

    #[test]
    #[cfg_attr(not(feature = "optional-tests"), ignore)]
    fn tiny_seed_return_trips() {
        // If you're having trouble with the roundtrip,
        let seed = TinySeed(762150966);
        let phrase = seed_to_phrase(seed);
        assert_eq!(
            "berkeley brink second polkadot brink your academy your",
            &phrase
        );
        assert_eq!(TinySeed(762150966), phrase_to_seed(&phrase).unwrap());

        let phrase = "chancellor of not your keys not your crypto";
        let seed = phrase_to_seed(phrase).unwrap();
        assert_eq!(TinySeed(3756838215), seed);
        assert_eq!(phrase.to_string(), seed_to_phrase(seed));
    }

    #[test]
    #[cfg_attr(not(feature = "optional-tests"), ignore)]
    fn seed_to_phrase_checksum_test() {
        let seed = TinySeed(762150966);
        let phrase = seed_to_phrase_with_checksum(seed);
        assert_eq!(
            "berkeley brink second polkadot brink your academy your academy brink",
            &phrase
        );
        assert_eq!(
            TinySeed(762150966),
            phrase_to_seed_with_checksum(&phrase).unwrap()
        );
    }

    #[test]
    #[cfg_attr(not(feature = "optional-tests"), ignore)]
    fn phrase_to_seed_checksum_test() {
        let phrase = "chancellor of not your keys not your crypto chancellor crypto";
        let seed = phrase_to_seed_with_checksum(phrase).unwrap();
        assert_eq!(TinySeed(3756838215), seed);
    }

    #[test]
    #[cfg_attr(not(feature = "optional-tests"), ignore)]
    fn tiny_seed_checksum_return_trips() {
        let seed = TinySeed(762150966);
        let phrase = seed_to_phrase_with_checksum(seed);
        assert_eq!(
            "berkeley brink second polkadot brink your academy your academy brink",
            &phrase
        );
        assert_eq!(
            TinySeed(762150966),
            phrase_to_seed_with_checksum(&phrase).unwrap()
        );

        let phrase = "chancellor of not your keys not your crypto chancellor crypto";
        let seed = phrase_to_seed_with_checksum(phrase).unwrap();
        assert_eq!(TinySeed(3756838215), seed);
        assert_eq!(phrase.to_string(), seed_to_phrase_with_checksum(seed));
    }
}
