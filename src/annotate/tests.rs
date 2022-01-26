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

mod path {
    use crate::Annotate;
    use std::path::{Path, PathBuf};

    fn is_root(input: &Path) -> Result<(), String> {
        if let Some(parent) = input.parent() {
            Err(format!(
                "Input {:?} has parent {:?}",
                input.display(),
                parent.display()
            ))
        } else {
            Ok(())
        }
    }

    #[test]
    fn within_ok() {
        let pb = PathBuf::from("/");
        let r = Annotate::within(pb.as_path(), is_root);
        assert!(r.is_ok());
    }

    #[test]
    fn within_err() {
        let pb = PathBuf::from("/not/a/root/path");
        let r = Annotate::within(pb.as_path(), is_root);
        let Annotate { info, source } = r.err().unwrap();
        assert_eq!(info, pb);
        assert_eq!(
            source,
            r#"Input "/not/a/root/path" has parent "/not/a/root""#
        );
    }
}
