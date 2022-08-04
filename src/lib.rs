use clap::{clap_derive::ArgEnum, Parser};
use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};
use std::vec::IntoIter;

/// Generate passwords that are easy to remember. Inspired by the xkcd webcomic:
/// <https://xkcd.com/936/>
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Number of words to include in the password
    #[clap(short, long, value_parser, default_value_t = 6)]
    pub number: usize,

    /// A separator to use between words
    #[clap(short, long, value_parser, default_value_t = String::from(" "))]
    pub separator: String,

    /// List of words to use for random password generation
    #[clap(short, long, arg_enum, value_parser, default_value_t = List::Long)]
    pub list: List,

    /// Case to use on the words
    #[clap(short, long, arg_enum, value_parser, default_value_t = Case::Lower)]
    pub case: Case,
}

/// List of words to use for password generation.
#[derive(ArgEnum, Clone, Debug)]
pub enum List {
    /// [EFF's long word list](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt)
    Long,
    /// [EFF's first short word list](https://www.eff.org/files/2016/09/08/eff_short_wordlist_1.txt)
    Short1,
    /// [EFF's second short word list](https://www.eff.org/files/2016/09/08/eff_short_wordlist_2_0.txt)
    Short2,
}

/// Case to use on the words.
#[derive(ArgEnum, Clone, Debug)]
pub enum Case {
    Upper,
    Lower,
    Capitalized,
    /// Randomly choose between converting the word to uppercase or lowercase
    Mixed,
}

/// Generate a random password in xkcd style.
///
/// Uses a cryptographically secure PRNG provided by the
/// [`rand`](https://docs.rs/rand/latest/rand/) crate.
pub fn generate_password(args: Args) -> String {
    let word_list = get_word_list(&args.list);
    let mut rng = rand::thread_rng();

    let random_words = get_random_words(word_list, &mut rng, args.number);
    let mut random_words = change_word_case(args.case, random_words, &mut rng);

    // to get random ordering of the words
    random_words.shuffle(&mut rng);

    let xkcd_password = random_words.join(&args.separator);

    xkcd_password
}

/// Return the contents of the word list.
/// Word list is chosen according to the option the user provided.
fn get_word_list(list: &List) -> &'static str {
    match list {
        List::Long => include_str!("words/eff_large_wordlist.txt"),
        List::Short1 => include_str!("words/eff_short_wordlist_1.txt"),
        List::Short2 => include_str!("words/eff_short_wordlist_2_0.txt"),
    }
}

///  Return a consuming iterator over a vector of randomly chosen words.
fn get_random_words<'a, T>(word_list: &'a str, rng: &mut T, num: usize) -> IntoIter<&'a str>
where
    T: Rng + ?Sized,
{
    word_list
        .split_whitespace()
        .choose_multiple(rng, num)
        .into_iter()
}

/// Change case used on the word according to the option provided by the user.
fn change_word_case<T: Rng>(case: Case, words: IntoIter<&str>, rng: &mut T) -> Vec<String> {
    let words: Vec<String> = match case {
        Case::Upper => words.map(str::to_uppercase).collect(),
        Case::Lower => words.map(str::to_lowercase).collect(),
        Case::Capitalized => words.map(str::capitalize).collect(),
        Case::Mixed => words.map(|word| word.to_random_case(rng)).collect(),
    };

    words
}

/// An extension trait to change letter casing.
trait ExtraCases {
    fn capitalize(&self) -> String;

    fn to_random_case<T: Rng>(&self, rng: &mut T) -> String;
}

impl ExtraCases for str {
    /// Return a new string with the first letter capitalized and the rest in lowercase.
    ///
    /// Since words provided for password generation are all English words,
    /// there is no need to worry about non-ASCII characters and grapheme clusters.
    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
        }
    }

    /// Convert to either upper or lower case randomly.
    ///
    /// Uses [`ThreadRng`](https://docs.rs/rand/latest/rand/rngs/struct.ThreadRng.html)
    /// from the [`rand`](https://docs.rs/rand/latest/rand/) crate.
    fn to_random_case<T: Rng>(&self, rng: &mut T) -> String {
        if rng.gen_range(0..=1) == 0 {
            self.to_lowercase()
        } else {
            self.to_uppercase()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalize_lower_case() {
        let all_lowercase = "foobar";
        assert_eq!(all_lowercase.capitalize(), "Foobar");
    }

    #[test]
    fn capitalize_upper_case() {
        let all_uppercase = "FOOBAR";
        assert_eq!(all_uppercase.capitalize(), "Foobar");
    }

    #[test]
    fn capitalize_mixed_case() {
        let mixed_case = "foOBaR";
        assert_eq!(mixed_case.capitalize(), "Foobar");
    }

    #[test]
    fn capitalize_nothing() {
        let nothing = "";
        assert_eq!(nothing.capitalize(), "");
    }

    #[test]
    fn gets_random_words() {
        use rand::{rngs::StdRng, SeedableRng};

        let word_list = include_str!("words/eff_large_wordlist.txt");

        // the function being tested uses randomness, rng is created from seed
        let mut rng = StdRng::from_seed([42; 32]);

        let actual: Vec<&str> = get_random_words(word_list, &mut rng, 6).collect();
        let expected = vec![
            "tamale",
            "manlike",
            "unguided",
            "atlas",
            "egomaniac",
            "vaporizer",
        ];

        assert_eq!(actual, expected);
    }

    fn default_change_case(case: Case) -> Vec<String> {
        change_word_case(
            case,
            vec!["foo", "bar", "buzz"].into_iter(),
            &mut rand::thread_rng(),
        )
    }

    #[test]
    fn change_to_uppercase() {
        let actual = default_change_case(Case::Upper);
        let expected = vec!["FOO", "BAR", "BUZZ"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn change_to_lowercase() {
        let actual = default_change_case(Case::Lower);
        let expected = vec!["foo", "bar", "buzz"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn change_to_capitalized() {
        let actual = default_change_case(Case::Capitalized);
        let expected = vec!["Foo", "Bar", "Buzz"];

        assert_eq!(actual, expected);
    }
}
