use aargvark::{
    self,
    vark_explicit,
    AargvarkTrait,
};
use aargvark_proc_macros::Aargvark;

macro_rules! svec{
    ($($l: literal), *) => {
        vec![$($l.to_string()), *]
    };
}

#[test]
fn t_str() {
    let v: String = vark_explicit(None, svec!["a"]);
    assert_eq!(v, "a");
}

#[test]
fn t_vec() {
    let v: Vec<String> = vark_explicit(None, svec!["a", "b"]);
    assert_eq!(v, svec!["a", "b"]);
}

#[test]
fn t_enum_unit() {
    #[derive(Aargvark, PartialEq, Debug)]
    enum Yol {
        ToqQuol,
    }

    let v: Yol = vark_explicit(None, svec!["toq-quol"]);
    assert_eq!(v, Yol::ToqQuol);
}

#[test]
fn t_enum_tuple() {
    #[derive(Aargvark, PartialEq, Debug)]
    enum Yol {
        ToqQuol(String, String),
    }

    let v: Yol = vark_explicit(None, svec!["toq-quol", "yon", "nor"]);
    assert_eq!(v, Yol::ToqQuol("yon".into(), "nor".into()));
}

#[test]
fn t_enum_struct() {
    #[derive(Aargvark, PartialEq, Debug)]
    enum Yol {
        ToqQuol {
            a: String,
        },
    }

    let v: Yol = vark_explicit(None, svec!["toq-quol", "pahla"]);
    assert_eq!(v, Yol::ToqQuol { a: "pahla".into() });
}

#[test]
fn t_struct() {
    #[derive(Aargvark, PartialEq, Debug)]
    struct Naya {
        a: String,
    }

    let v: Naya = vark_explicit(None, svec!["wowo"]);
    assert_eq!(v, Naya { a: "wowo".into() });
}

#[test]
fn t_struct_opt_only() {
    #[derive(Aargvark, PartialEq, Debug)]
    struct Naya {
        a: Option<String>,
    }

    let v: Naya = vark_explicit(None, svec!["--a", "wowo"]);
    assert_eq!(v, Naya { a: Some("wowo".into()) });
}

#[test]
fn t_struct_opt_first() {
    #[derive(Aargvark, PartialEq, Debug)]
    struct Naya {
        b: String,
        a: Option<String>,
    }

    let v: Naya = vark_explicit(None, svec!["--a", "wowo", "noh"]);
    assert_eq!(v, Naya {
        b: "noh".into(),
        a: Some("wowo".into()),
    });
}

#[test]
fn t_struct_opt_last() {
    #[derive(Aargvark, PartialEq, Debug)]
    struct Naya {
        b: String,
        a: Option<String>,
    }

    let v: Naya = vark_explicit(None, svec!["noh", "--a", "wowo"]);
    assert_eq!(v, Naya {
        b: "noh".into(),
        a: Some("wowo".into()),
    });
}

#[test]
fn t_generic() {
    #[derive(Aargvark, PartialEq, Debug)]
    struct Naya<T: 'static + AargvarkTrait> {
        b: Option<T>,
    }

    let v: Naya<String> = vark_explicit(None, svec!["--b", "hi"]);
    assert_eq!(v, Naya { b: Some("hi".to_string()) });
}
