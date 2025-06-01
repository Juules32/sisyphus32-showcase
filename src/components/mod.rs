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
    navbars,
    navlink,
    home,
    aboutprojects,
    webapps,
    layout,
    gamejams,
    bsc,
    msc,
    lorem,
    chessengines,
    chessboard
);
