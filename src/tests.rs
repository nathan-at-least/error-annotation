mod str {
    use crate::{annotate, ErrorAnnotation};

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
        let r = is_banana(s).map_err(annotate("fruit", s));
        assert!(r.is_ok());
    }

    #[test]
    fn within_err() {
        let s = "apple";
        let r = is_banana(s).map_err(annotate("fruit", s));
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
    use crate::{annotate, ErrorAnnotation};
    use std::fmt;
    use std::path::{Path, PathBuf};

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct PathBufDisp(PathBuf);

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
        let pbd = PathBufDisp(PathBuf::from("/"));
        let r = is_root(&pbd.0).map_err(annotate("path", pbd));
        assert!(r.is_ok());
    }

    #[test]
    fn within_err() {
        let pbd = PathBufDisp(PathBuf::from("/not/a/root/path"));
        let r = is_root(&pbd.0).map_err(annotate("path", pbd.clone()));
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
        assert_eq!(info, pbd);
    }

    #[test]
    fn display() {
        let pbd = PathBufDisp(PathBuf::from("/not/a/root/path"));
        let e = ErrorAnnotation {
            source: 42,
            label: "path",
            info: pbd,
        };
        assert_eq!(e.to_string(), "42\n-with path: /not/a/root/path");
    }
}
