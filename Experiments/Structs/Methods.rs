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

struct Game {
    appID: u64,
    depotID: u64,
    manifestID: u64,
    username: String,
    password: String
}

impl Game {
    fn Anonymous(appID: u64, depotID: u64, manifestID: u64) -> Self{
        Self {
            username: String::new(),
            password: String::new(),
            appID,
            depotID,
            manifestID,
        }
    }
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

fn download(game: &Game) {
    println!("Downloading app {} with username: {}", game.appID, (if game.username.is_empty() {String::from("none")} else {game.username.to_string()}))
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

    download(&Game {
        username: String::from("maarten"),
        password: String::from("ewgergergfdgergr"),
        appID: 432423,
        depotID: 423423,
        manifestID: 53495346534
    });
    download(&Game::Anonymous(
        42543,
        45353,
        56734956349
    ));
}
//TODO: create a tool that takes a number+unit like mm, and converts to unit like cm or in.
