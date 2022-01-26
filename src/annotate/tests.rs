mod string {
    use crate::Annotate;

    fn is_banana(input: &str) -> Result<(), String> {
        if input == "banana" {
            Ok(())
        } else {
            Err(format!(r#"Input {:?} != "banana""#, input))
        }
    }

    #[test]
    fn within_ok() {
        let r = Annotate::within("banana", is_banana);
        assert!(r.is_ok());
    }

    #[test]
    fn within_err() {
        let r = Annotate::within("apple", is_banana);
        let Annotate { info, source } = r.err().unwrap();
        assert_eq!(info, "apple");
        assert_eq!(source, r#"Input "apple" != "banana""#);
    }
}
