#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))] // https://stackoverflow.com/a/61417700/4809297

/*!

# Generate passphrase

This crate allows to [generate] passphrases easily and securely.

Choose the source of words to use in passphrases: either use [built-in dictionaries](dictionary) or provide your own dictionary!

By default, the crate does not come with any built-in dictionaries in order to keep the crate small.
Built-in dictionaries can be added through the usage of features in the crate.

## Custom dictionary

```
let dictionary = &["hello", "bonjour", "hola", "ciao"];

// Example of generated passphrase: "bonjour-hello"
let passphrase = gen_passphrase::generate(&[dictionary], 2, Some("-"));
```

## Built-in dictionary

To enable [built-in dictionaries](dictionary), you need to enable the feature corresponding to the requested dictionary.

| Dictionary | Feature to enable |
|------------|-------------------|
| Eff Short Wordlist 2 | `eff_short_2` |
| Eff Short Wordlist 1 | `eff_short_1` |
| Eff Large Wordlist | `eff_large` |


For example, to use the dictionary eff_short_2, add this in your `Cargo.toml`:

```toml
[dependencies.gen_passphrase]
version = "*"
features = ["eff_short_2"]
```

Then, use the dictionary like you would use a custom dictionary:

```
# #[cfg(feature = "eff_short_2")]
use gen_passphrase::dictionary::EFF_SHORT_2;

# #[cfg(feature = "eff_short_2")]
let passphrase = gen_passphrase::generate(&[EFF_SHORT_2], 1, None);
```

*/

pub mod dictionary;

fn random_word(dictionary: &'static [&'static str]) -> &'static str {
    use nanorand::{ChaCha20, Rng};
    use std::ops::Range;

    let range = Range {
        start: 0,
        end: dictionary.len(),
    };
    dictionary[ChaCha20::new().generate_range(range)]
}

/// Generate a passphrase
///
/// Create a passphrase with words from dictionaries a number of times, with an optional delimiter between words.
///
/// The total number of words is `dictionaries.len() * iterations`.
///
/// # Arguments
///
/// * `dictionaries` - List of dictionaries to use.
/// * `iterations` - Number of iterations over the provided dictionaries.
/// * `delimiter` - Optional text to add between selected words.
///
/// # Examples
///
/// Example with one dictionary, iterate once, no delimiter.
/// The total number of words in the passphrase will be 1.
///
/// ```
/// let custom_dictionary = &["this", "is", "my", "custom", "dictionary"];
///
/// // Example of generated passphrase: "custom"
/// let passphrase = gen_passphrase::generate(&[custom_dictionary], 1, None);
/// ```
///
/// Example with two dictionaries, iterate twice, use "-" as delimiter
/// The total number of words in the passphrase will be 4.
///
/// ```
/// let custom_dictionary = &["this", "is", "my", "custom", "dictionary"];
/// let hello_dictionary = &["hello", "bonjour", "hola", "ciao"];
///
/// // Example of generated passphrase: "bonjour-dictionary-ciao-this"
/// let passphrase = gen_passphrase::generate(&[hello_dictionary, custom_dictionary], 2, Some("-"));
/// ```
pub fn generate(
    dictionaries: &[&'static [&'static str]],
    iterations: usize,
    delimiter: Option<&'static str>,
) -> String {
    let mut string = String::new();

    for x in 0..iterations {
        // Repeat for every provided dictionary
        for (y, dictionary) in dictionaries.iter().enumerate() {
            // Choose random word
            string.push_str(random_word(dictionary));

            // Add delimiter if iteration is not over
            if let Some(delimiter) = delimiter {
                if y != dictionaries.len() - 1 {
                    string.push_str(delimiter);
                }
            }
        }

        // Add delimiter if iteration is not over
        if let Some(delimiter) = delimiter {
            if x != iterations - 1 {
                string.push_str(delimiter);
            }
        }
    }

    string
}

#[cfg(test)]
mod tests {
    use crate::generate;

    const CUSTOM_DICTIONARY: &[&str] = &["this", "is", "my", "custom", "dictionary"];
    const CUSTOM_DICTIONARY_2: &[&str] = &["wow", "another", "handmade", "book"];

    #[test]
    fn empty() {
        let passphrase = generate(&[CUSTOM_DICTIONARY], 0, None);
        assert!(passphrase.is_empty())
    }

    #[test]
    fn one_dictionary() {
        let dictionaries = &[CUSTOM_DICTIONARY];
        let iterations = 2;
        let delimiter = "-";

        let passphrase = generate(dictionaries, iterations, Some(delimiter));

        assert!(!passphrase.is_empty());
        assert_eq!(passphrase.matches(delimiter).count(), iterations - 1);
    }

    #[test]
    fn multiple_dictionaries() {
        let dictionaries = &[CUSTOM_DICTIONARY, CUSTOM_DICTIONARY_2];
        let iterations = 4;
        let delimiter = "_";

        let passphrase = generate(dictionaries, iterations, Some(delimiter));

        assert!(!passphrase.is_empty());
        assert_eq!(
            passphrase.matches(delimiter).count(),
            iterations * dictionaries.len() - 1
        );
    }
}
