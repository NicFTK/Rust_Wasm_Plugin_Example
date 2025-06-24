use super::bindings::{acme::plugins::log, export, exports::acme::plugins::prettify_plugin};

// This is our implementation of the "prettify" plugin type
// (=WIT world)

// We shall make the content pretty by quoting all words

// Gorgeous!
use val::val::local::val::types::Val;
pub struct QuotePrettifyPlugin;

export!(QuotePrettifyPlugin);

impl prettify_plugin::Guest for QuotePrettifyPlugin {
    fn prettify(content: String) -> Val {
        log::log("thank you for using the quote prettify plugin!");
        let words = content.split(" ");
        let words: Vec<String> = words.map(|word| format!("{:?}", word)).collect();
        let s = words.join(" ");

        Val::Str(s)
    }
}
