use crate::Annotate;

fn is_banana(input: &str) -> Result<(), String> {
    if input == "banana" {
        Ok(())
    } else {
        Err(format!("Input {:?} != \"banana\"", input))
    }
}

#[test]
fn within_no_error() {
    let r = Annotate::within("banana", is_banana);
    assert!(r.is_ok());
}
