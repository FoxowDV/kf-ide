use super::Syntax;
use std::collections::BTreeSet;

impl Syntax {
    pub fn kf() -> Syntax {
        Syntax {
            language: "kf",
            case_sensitive: true,
            comment: "\\/\\/",
            comment_multiline: [r#"/*"#, r#"*/"#],
            hyperlinks: BTreeSet::from(["http"]),
            keywords: BTreeSet::from([
                "and"
            ]),
            types: BTreeSet::from([
                "bool"
            ]),
            special: BTreeSet::from([
                "false"
            ])

        }
    }
}
