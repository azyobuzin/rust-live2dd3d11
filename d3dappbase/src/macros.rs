#[macro_export]
macro_rules! trycom {
    ($e:expr) => ((if ($e) < 0 { return Err(()); }))
}
