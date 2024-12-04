extern crate proc_macro;

use proc_macro::*;

#[proc_macro_attribute]
pub fn get(args: TokenStream, item: TokenStream) -> TokenStream {
    // let mut route = Route {
    //     path: String::from(""),
    //     handler: String::from(""),
    //     method: String::from(""),
    // };

    // let args = args.to_string();
    // let args = args.split(",").collect::<Vec<&str>>();

    // for arg in args {
    //     match arg {
    //         "GET" => route.set_method(String::from("GET")),
    //         "POST" => route.set_method(String::from("POST")),
    //         "PUT" => route.set_method(String::from("PUT")),
    //         "DELETE" => route.set_method(String::from("DELETE")),
    //         "PATCH" => route.set_method(String::from("PATCH")),
    //         "HEAD" => route.set_method(String::from("HEAD")),
    //         "OPTIONS" => route.set_method(String::from("OPTIONS")),
    //         _ => match arg.to_string().find(",") {
    //             //This will map the path to the handler ignoring the comma
    //             Some(_) => {}
    //             None => {
    //                 route.set_path(arg.to_string());
    //             }
    //         },
    //     }
    // }

    // let handler = item.to_string();
    // route.set_handler(handler);

    // println!("{}", route);

    // This will return the function under the macro so that it can be called and not consumed by the macro
    format!(r#" {}"#, item)
        .parse()
        .expect("Unable to parse item")
}
