#[macro_export]
macro_rules! ward {
    ($o:expr) => {
        if let Some(x) = $o { x } else { return; };
    };
    ($o:expr, else $body:block) => {
        if let Some(x) = $o { x } else { $body; return; };
    };
}

#[macro_export]
macro_rules! guard {
    (let $result:ident = $o:expr) => {
        let $result = ward!($o);
    };
    (let $result:ident = $o:expr, else $body:block) => {
        let $result = ward!($o, else $body);
    };
    (let mut $result:ident = $o:expr) => {
        let mut $result = ward!($o);
    };
    (let mut $result:ident = $o:expr, else $body:block) => {
        let mut $result = ward!($o, else $body);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn guard_extracts_value_from_some() {
        let sut = Some("test");
        guard!(let res = sut);
        assert_eq!("test", res);
    }

    #[test]
    fn guard_returns_early_on_none() {
        let sut = None;
        guard!(let mut _res = sut);
        unreachable!();
    }

    #[test]
    fn guard_runs_else_on_none() {
        let sut = None;
        guard!(let _res = sut, else {
            assert!(true);
        });
        unreachable!();
    }
}
