macro_rules! mod_and_use {
    ($($name:ident),*) => {
        $(
            mod $name;
            #[allow(unused_imports)]
            pub use $name::*;
        )*
    };
}

mod_and_use!(
    bsc,
    chessboard,
    chessengines,
    copy,
    gamejams,
    home,
    layout,
    lorem,
    msc,
    navbars,
    navlink,
    webapps
);
