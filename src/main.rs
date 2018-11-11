extern crate uuid;
extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use uuid::Uuid;

fn main() {
    let id = Uuid::new_v4();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(id.to_string()).expect("Failed to put id in clipboard");
    println!("{}", id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_uuid_to_clipboard() {
        main();
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let clipboard = ctx.get_contents().unwrap();
        assert!(Uuid::parse_str(&clipboard).is_ok());
    }
}
