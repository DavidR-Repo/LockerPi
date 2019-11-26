
/*
Packages: A Cargo feature that lets you build, test, and share crates
    Must contain at least one crate with at most one library
    Provides a set of functionality
    Contains a Cargo.toml file (build info)


Crates: A tree of modules that produces eiter a library or an executable
    Crate root --> compiler starting point
        src/main.rs or src/lib.rs --> convention for root
    If a package contains src/main.rs and src/lib.rs, it has two crates: 
        a library and a binary, both with the same name as the package
    Groups with related functionality together in scope 
        (ex: use rand::Rng from the rand crate in another crate)
    use src/bin for other binary sources


Modules and 'use': Control the organization, scope, and privacy of paths
    Organizes code within a crate into groups for readability and reuse.
    Controls the privacy of items (pub, glob)
    Can be nested:

    //mod --> example restaurante module
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {...}

            fn seat_at_table() {...}
        }

        mod serving {
            fn take_order() {...}

            fn serve_order() {...}

            fn take_payment() {...}
        }
    }

    the root module is 'crate' implicitly

Paths: A way of naming an item, such as a struct, function, or module
    Absolute path - starts from 'crate'
        ex: crate::front_of_house::hosting::add_to_waitlist();
    Relative path - uses 'self', 'super', or identifier from current crate
        ex: front_of_house::hosting::add_to_waitlist(); 
    Modules are private by default with access to parent module
    Use pub to give surrounding scope access
       ex in module front_of_house: 
            pub mod hosting { pub fn eat_at_restaurant() {...} }
                ( now sibling module to 'front_of_house' can use 'eat_at_restaurante()' )
    super --> parent scope (for relative path)
        super::<parent scope function, method, struct, enum, module, or constant> 
    public struct:
        mod back_of_house {
            pub struct Breakfast {
                pub toast: String, //now Breakfast.toast can be used
                seasonal_fruit: String, //still private, cannot be access from surrounding scope
            }
        }
        surrounding scope will not be able to instantiate unless private method or constructor sets seasonal_fruit
    public enum: if the enum is marked 'pub', all it's members are automatically public

    'use': Bring modules into scope
        ex:
            use crate::front_of_house::hosting; // convention: functions - stop at moduel level
            pub fn eat_at_restaurant() {
                hosting::add_to_waitlist(); ...} // can also use relative path
            
            use std::collections::HashMap;  // convention: structs, enums, and other items, go all the way
            fn main() {
                let mut map = HashMap::new();
                map.insert(1, 2);
            }
        Name conflicts: (prevent compiler error: use 'as' to rename)
            use std::fmt::Result;
            use std::io::Result as IoResult;
            (or just stop at '... ::fmt' and '... ::io' instead)
        Bring in std::cmp::Ordering and std::io with one line:
            use std::{cmp::Ordering, io};
                OR
            use std::io::{self, Write}; // (for std::io and std::io::Write)
                OR
            use std::collections::*; // (glob '*' is easier, but harder to tell what is actually in scope)
    'pub use': (aka: re-exporting)
            pub use crate::front_of_house::hosting; // now public items in ::hosting can be used too

            pub fn eat_at_restaurant() {
                hosting::add_to_waitlist(); // because add_to_waitlist() is public
                hosting::add_to_waitlist();
                hosting::add_to_waitlist();
            }

External Packages: bring in crates from 'crates.io'
            In the cargo.toml: (to use rand version 0.5.5)
                [dependencies]
                rand = "0.5.5" //now we can 'use rand::<what we want to use>
            Note: 'std' is implicitly included, 'use std::<std item>' when needed

Spliting up files:
    // in File: src/lib.rs
    mod front_of_house;

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }

    // in File: src/front_of_house.rs

    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    // because hosting is in the file src/front_of_house.rs it is implied that 
    // it is a child module of front_of-house
*/

fn main() {
    println!("see comment block, from chapter 7 of the rust language book");
}
