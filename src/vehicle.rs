use rand::{self, distributions::Alphanumeric, Rng};
use serde::Serialize;

const MAKES: &[&str] = &[
    "Chevrolet",
    "Alfa Romeo",
    "Dodge",
    "Kia",
    "BMW",
    "Audi",
    "Tesla",
    "Ford",
    "GMC",
    "Aston Martin",
    "Honda",
    "Mercedes-Benz",
    "Toyota",
    "Volkswagen",
    "Ferrari",
    "Chrysler",
    "Lexus",
    "Subaru",
    "Nissan",
    "Hyundai",
    "Jaguar",
    "Fiat",
    "Jeep",
    "Porsche",
    "HUMMER",
];

const MODELS: &[&str] = &[
    "Silverado",
    "SRX",
    "Routan",
    "Journey",
    "Paseo",
    "Colorado",
    "GTI",
    "XT5",
    "Legend",
    "Focus",
    "MKZ",
    "X1",
    "Q7",
    "Escape",
    "Optima",
    "S4",
    "RC",
    "C/K",
    "M5",
    "Enclave",
    "Monte Carlo",
    "Legacy",
    "Malibu",
    "S6",
    "Sentra",
];

pub trait Random {
    fn random() -> Self;
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Vehicle {
    make: String,
    model: String,
    model_year: String,
    vin: String,
    dln: String,
}

impl Random for Vehicle {
    fn random() -> Self {
        let mut rng = rand::thread_rng();

        let alpha_numeric = |length: usize| {
            let mut rng = rand::thread_rng();

            (0..length)
                .into_iter()
                .map(|_| rng.sample(Alphanumeric) as char)
                .collect::<String>()
        };

        Vehicle {
            make: MAKES[rng.gen_range(0..MAKES.len())].to_string(),
            model: MODELS[rng.gen_range(0..MODELS.len())].to_string(),
            model_year: rng.gen_range(1..=2023).to_string(),
            vin: alpha_numeric(12),
            dln: alpha_numeric(7),
        }
    }
}
