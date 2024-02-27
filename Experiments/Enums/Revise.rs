enum VoertuigType {
    Auto,
    Fiets,
    Scooter,
    Fatbike,
}

struct Voertuig {
    voertuig_type: VoertuigType,
    merk: String,
    model: String,
    jaar: u16,
}

impl Voertuig {
    fn create(voertuig_type: VoertuigType, merk: &str, model: &str, jaar: u16) -> Voertuig { // u16 is not a reference because it won't go out of scope after borrowing.
        Voertuig {
            voertuig_type,
            merk: String::from(merk),
            model: String::from(model),
            jaar,
        }
    }
    /*fn tipe(v: &Voertuig) -> String {
        /* todo
        String::from(v.voertuig_type.to_owned())
        String::from(v.voertuig_type.to_owned())
        String::from(v.clone().voertuig_type)
        String::from(v.to_owned().voertuig_type)

         */
    }*/
    fn year(v: &Voertuig) -> u16{
        v.jaar
    }
    fn merk(v: &Voertuig) -> String{
        v.merk.clone()
    }
    fn model(v: &Voertuig) -> String{
        v.model.clone()
    }
}

pub fn main() {
    let car1  = Voertuig::create(VoertuigType::Auto, "Volvo", "V50", 2011);
    println!("Auto jaar: {}", Voertuig::year(&car1));
    println!("Auto merk: {}", Voertuig::merk(&car1));
    println!("Auto model: {}", Voertuig::model(&car1));
}