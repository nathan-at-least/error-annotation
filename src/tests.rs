mod str {
    use crate::{AnnotateResult, ErrorAnnotation};

    fn is_banana(input: &str) -> Result<(), String> {
        if input == "banana" {
            Ok(())
        } else {
            Err(format!(r#"Input {:?} != "banana""#, input))
        }
    }

    #[test]
    fn within_ok() {
        let s = "banana";
        let r = is_banana(s).annotate_err("fruit", || s);
        assert!(r.is_ok());
    }

    #[test]
    fn within_err() {
        let s = "apple";
        let r = is_banana(s).annotate_err("fruit", || s);
        let ErrorAnnotation {
            info,
            label,
            source,
        } = r.err().unwrap();
        assert_eq!(source, r#"Input "apple" != "banana""#);
        assert_eq!(label, "fruit");
        assert_eq!(info, "apple");
    }

    #[test]
    fn display() {
        let e = ErrorAnnotation {
            source: 42,
            label: "thingy",
            info: "woot",
        };
        assert_eq!(e.to_string(), "42\n-with thingy: woot");
    }
}

mod path {
    use crate::{AnnotateResult, ErrorAnnotation};
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
        let path = &PathBuf::from("/");
        let r = is_root(path).annotate_err("path", || path.display());
        assert!(r.is_ok());
    }

    #[test]
    fn within_err() {
        let path = &PathBuf::from("/not/a/root/path");
        let r = is_root(path).annotate_err("path", || path.display());
        let ErrorAnnotation {
            info,
            label,
            source,
        } = r.err().unwrap();
        assert_eq!(
            source,
            r#"Input "/not/a/root/path" has parent "/not/a/root""#
        );
        assert_eq!(label, "path");
        assert_eq!(&info.to_string(), "/not/a/root/path");
    }

    #[test]
    fn display() {
        let path = &PathBuf::from("/not/a/root/path");
        let e = ErrorAnnotation {
            source: 42,
            label: "path",
            info: path.display(),
        };
        assert_eq!(e.to_string(), "42\n-with path: /not/a/root/path");
    }
}
