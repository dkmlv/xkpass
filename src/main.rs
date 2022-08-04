use clap::{clap_derive::ArgEnum, Parser};
use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};

/// Generate passwords that are easy to remember. Inspired by the xkcd webcomic:
/// <https://xkcd.com/936/>
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of words to include in the password
    #[clap(short, long, value_parser, default_value_t = 6)]
    number: usize,

    /// A separator to use between words
    #[clap(short, long, value_parser, default_value_t = String::from(" "))]
    separator: String,

    /// List of words to use for random password generation
    #[clap(short, long, arg_enum, value_parser, default_value_t = List::Long)]
    list: List,

    /// Case to use on the words
    #[clap(short, long, arg_enum, value_parser, default_value_t = Case::Lower)]
    case: Case,
}

/// List of words to use for password generation.
#[derive(ArgEnum, Clone, Debug)]
enum List {
    /// [EFF's long word list](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt)
    Long,
    /// [EFF's first short word list](https://www.eff.org/files/2016/09/08/eff_short_wordlist_1.txt)
    Short1,
    /// [EFF's second short word list](https://www.eff.org/files/2016/09/08/eff_short_wordlist_2_0.txt)
    Short2,
}

/// Case to use on the words.
#[derive(ArgEnum, Clone, Debug)]
enum Case {
    Upper,
    Lower,
    Capitalized,
    /// Randomly choose between converting the word to uppercase or lowercase
    Mixed,
}

fn main() {
    let args = Args::parse();

    let word_list = match args.list {
        List::Long => include_str!("words/eff_large_wordlist.txt"),
        List::Short1 => include_str!("words/eff_short_wordlist_1.txt"),
        List::Short2 => include_str!("words/eff_short_wordlist_2_0.txt"),
    };

    let mut rng = rand::thread_rng();

    let random_words = word_list
        .split_whitespace()
        .choose_multiple(&mut rng, args.number)
        .into_iter();

    let mut random_words: Vec<String> = match args.case {
        Case::Upper => random_words.map(str::to_uppercase).collect(),
        Case::Lower => random_words.map(str::to_lowercase).collect(),
        Case::Capitalized => random_words.map(str::capitalize).collect(),
        Case::Mixed => random_words
            .map(|word| word.to_random_case(&mut rng))
            .collect(),
    };

    // to get random ordering of the words
    random_words.shuffle(&mut rng);

    let xkcd_password = random_words.join(&args.separator);

    println!("{}", xkcd_password);
}

/// An extension trait to change letter casing.
trait ExtraCases {
    fn capitalize(&self) -> String;

    fn to_random_case<T: Rng>(&self, rng: &mut T) -> String;
}

impl ExtraCases for str {
    /// Return a new string with the first letter capitalized.
    ///
    /// Since words provided for password generation are all English words,
    /// there is no need to worry about non-ASCII characters and grapheme clusters.
    ///
    /// # Examples
    ///
    /// ```
    /// let input = "hello";
    /// let output = input.capitalize();
    ///
    /// assert_eq!(output, "Hello".to_string());
    /// ```
    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().to_string() + chars.as_str(),
        }
    }

    /// Convert to either upper or lower case randomly.
    ///
    /// Uses [`ThreadRng`](https://docs.rs/rand/latest/rand/rngs/struct.ThreadRng.html)
    /// from the [`rand`](https://docs.rs/rand/latest/rand/) crate.
    ///
    /// # Examples
    ///
    /// ```
    /// let input = "foobar";
    /// let output = input.to_mixed_case(&mut rand::thread_rng());
    ///
    /// assert!(["FOOBAR".to_string(), "foobar".to_string()].contains(output))
    /// ```
    fn to_random_case<T: Rng>(&self, rng: &mut T) -> String {
        if rng.gen_range(0..=1) == 0 {
            self.to_lowercase()
        } else {
            self.to_uppercase()
        }
    }
}
