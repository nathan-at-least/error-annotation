mod string {
    use crate::ErrorAnnotation;

    fn is_banana(input: &str) -> Result<(), String> {
        if input == "banana" {
            Ok(())
        } else {
            Err(format!(r#"Input {:?} != "banana""#, input))
        }
    }

    #[test]
    fn within_ok() {
        let r = ErrorAnnotation::within(String::from("banana"), is_banana);
        assert!(r.is_ok());
    }

    #[test]
    fn within_err() {
        let r = ErrorAnnotation::within(String::from("apple"), is_banana);
        let ErrorAnnotation { info, source } = r.err().unwrap();
        assert_eq!(info, "apple");
        assert_eq!(source, r#"Input "apple" != "banana""#);
    }

    #[test]
    fn display() {
        let e = ErrorAnnotation::from((String::from("woot"), 42));
        assert_eq!(e.to_string(), "42\nInfo: woot");
    }
}

mod path {
    use crate::ErrorAnnotation;
    use std::borrow::Borrow;
    use std::fmt;
    use std::path::{Path, PathBuf};

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct PathBufDisp(PathBuf);

    impl Borrow<Path> for PathBufDisp {
        fn borrow(&self) -> &Path {
            self.0.borrow()
        }
    }

    impl fmt::Display for PathBufDisp {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.display().fmt(f)
        }
    }

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
        let pb = PathBufDisp(PathBuf::from("/"));
        let r = ErrorAnnotation::within(pb, is_root);
        assert!(r.is_ok());
    }

    #[test]
    fn within_err() {
        let pb = PathBufDisp(PathBuf::from("/not/a/root/path"));
        let r = ErrorAnnotation::within(pb.clone(), is_root);
        let ErrorAnnotation { info, source } = r.err().unwrap();
        assert_eq!(info, pb);
        assert_eq!(
            source,
            r#"Input "/not/a/root/path" has parent "/not/a/root""#
        );
    }

    #[test]
    fn display() {
        let pb = PathBufDisp(PathBuf::from("/not/a/root/path"));
        let e = ErrorAnnotation::from((pb, 42));
        assert_eq!(e.to_string(), "42\nInfo: /not/a/root/path");
    }
}
