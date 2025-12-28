macro_rules! push_clause {
    ($dest:expr, $fmt:expr $(, $arg:tt)*) => {
        {
            if !$dest.is_empty()
                && !$dest.ends_with(' ')
                && !$fmt.starts_with(',')
            {
                $dest.push(' ');
            }
            let _ = write!($dest, $fmt $(, $arg)*);
        }
    };
}

pub(crate) use push_clause;
