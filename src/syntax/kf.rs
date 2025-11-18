use egui_code_editor::Syntax;
use std::collections::BTreeSet;

pub trait SyntaxExt {
    fn kf() -> Syntax;
}

impl SyntaxExt for Syntax {
    fn kf() -> Syntax {
        Syntax {
            language: "kf",
            case_sensitive: true,
            comment: "\\/\\/",
            comment_multiline: [r#"/*"#, r#"*/"#],
            hyperlinks: BTreeSet::from(["http"]),
            keywords: BTreeSet::from([
                "utl",
                "off",
                "onoff",
                "on",
                "wii",
                "mote",
                "kf",
                "next",
                "ash",
                "brokie",
                "send",
                "tnirp",
                "tupni",
                "join",
            ]),
            types: BTreeSet::from([
                "chip",
                "yesorno",
                "ntr",
                "michi",
                "ont",
                "uont",
                "dec",
            ]),
            special: BTreeSet::from([
                "yes",
                "no",
                "lesst",
                "great",
                "and",
                "or",
                "nah",
                "noteq",
                "eq",
                "is",
                "plus",
                "plusplus",
                "mult",
                "minus",
                "minusminus",
                "by",
                "mod",
            ])

        }
    }
}
