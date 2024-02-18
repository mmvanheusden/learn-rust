#![allow(E0753)]

#[derive(Debug)]
struct Balk {
    lengte: u32,
    hoogte: u32,
    breedte: u32,
}

struct Vormen {
    lengte: u32,
    hoogte: u32,
    breedte: u32,
}

impl Vormen {
    fn vierkant(size: u32) -> Self {
        Self {
            breedte: size,
            lengte: size,
            hoogte: 0,
        }
    }
}

impl Balk {
    fn volume(&self) -> u32 {
        self.lengte * self.breedte * self.hoogte
    }

    fn past_in(&self, other: &Balk) -> bool {
        self.volume() > other.volume()
    }
}


pub fn main() {
    /*!
    **Methods** are similar to functions. We declare them with `fn` and name them.
    They can have parameters and a return value.

    Unlike functions, methods are defined within the context of a struct, and their first parameter is always `self`,
    which represents the instance of the struct the method is called from.
     */

    let balk1 = Balk {
        lengte: 320,
        hoogte: 130,
        breedte: 76,
    };
    let balk2 = Balk {
        lengte: 432,
        hoogte: 322,
        breedte: 19,
    };
    let balk3 = Balk {
        lengte: 32,
        hoogte: 344,
        breedte: 242,
    };

    println!(
        "Het volume van balk 1 is {}mm³.",
        balk1.volume()
    );


    if balk1.past_in(&balk2) {
        println!("Balk 1 past in balk 2!")
    } else {
        println!("Balk 1 past niet in balk 2!")
    }

    if balk2.past_in(&balk3) {
        println!("Balk 2 past in balk 3!")
    } else {
        println!("Balk 2 past niet in balk 3!")
    }

    let postzegel = Vormen::vierkant(33);
}
//TODO: create a tool that takes a number+unit like mm, and converts to unit like cm or in.
