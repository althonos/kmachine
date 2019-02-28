macro_rules! args {
    ($ins:ident, $op:ident ( $($argtype:path),* )) => ({
        let mut args = $ins.arguments().iter().enumerate();
        ($(
            match args.next() {
                Some((_, $argtype(a))) => a,
                Some((i, arg)) => panic!(
                    "invalid argument #{} for instruction `{}`: {:?}",
                    i+1, stringify!($op), arg,
                ),
                None => panic!(
                    "missing argument for instruction `{}`",
                    stringify!($op)
                )
            }
        ,)*)
    });
}
