#[macro_export]
macro_rules! add_routes {
    ($( $x:expr ),*) => {
        {
            let mut routes = Vec::new();
            $(
                routes.push($x);
            )*
            routes
        }
    };
}
