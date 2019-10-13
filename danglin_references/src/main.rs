fn main() {
    // let reference_to_nothing = dangle_wrong();
    let reference_to_nothing = dangle_ok();
}

// fn dangle_wrong () -> &String {
//     let s = String::from("Hola");
//     &s
// }

fn dangle_ok () -> String {
    let s = String::from("Hola");
    s   // We con solve this problem returning the S value directly without references
        // to prevent release this String memory
}