macro_rules! args {
    ($ins:ident, $op:ident ( $($argtype:path),* )) => ({
        let mut it = $ins.arguments().iter().enumerate();
        let args = ($(
            match it.next() {
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
        ,)*);

        let remaining: Vec<_> = it.map(|x| format!("{:?}", x.1)).collect();
        if !remaining.is_empty() {
            panic!(
                "unused arguments for instruction `{}`: {}",
                stringify!($op),
                remaining.join(", ")
            )
        }

        args
    });
}
