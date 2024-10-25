use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub fn copy(text: String) {
    let mut ctx = ClipboardContext::new().unwrap();

    ctx.set_contents(text.to_owned()).unwrap();
    assert_eq!(ctx.get_contents().unwrap(), text);
}