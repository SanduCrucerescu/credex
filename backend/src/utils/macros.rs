macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = ::std::collections::BtTreeMap::new();
        $(m.insert($k,$v);)+
        m
    }};
}
pub(crate) use map;
