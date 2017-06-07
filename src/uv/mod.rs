#![allow(
    unused_variables,
    dead_code,
    unused_parens,
    unused_must_use,
    unused_imports,
    non_upper_case_globals,
    )]

pub mod ps1;
// mod ps2;
// mod ps3;
// mod ps4;
pub mod joiner;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joiner() {
        use self::joiner::*;
        use std::str;

        let msg = "Foor Never Moore.";
        println!("Original message: {}", msg);
        let cipher = split(msg.as_bytes());
        let decrypted
            = join(cipher.0.as_slice(), cipher.1.as_slice());
        let decrypted = str::from_utf8(decrypted.as_slice()).unwrap();
        println!("Decrypted message:{}", decrypted);

        assert_eq!(msg, decrypted);
    }

    #[test]
    fn string_ext() {
        use self::utils::ascii_string::{
            StringExt,
            Test,
            };

        let mut x: String = "bolahboo!".to_owned();

        // "bolahboo!" : String
        assert_eq!(Some(0), x.find_in(0.., "bo"));
        assert_eq!(Some(5), x.find_in(1.., "bo"));

        // "bolahboo!" : &mut str
        let s = x.as_mut_str();

        // "bolahczo!" : &mut str
        let c = "cz".to_owned();
        s.put(5..7, c);
        assert_eq!(Some(0), s.find_in(0..3, "bo"));
        assert_eq!(None, s.find_in(1.., "bo"));

        // "bolahboo!" : &mut str
        let b = "bo";
        s.put(5..7, b);
        assert_eq!(Some(0), s.find_in(..4, "bo"));
        assert_eq!(Some(5), s.find_in(1.., "bo"));
    }
}
