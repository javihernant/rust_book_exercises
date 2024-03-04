use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum TempUnit {
    Farenheit,
    Celsius,
}

pub fn convert_temp(temp: f64, from: TempUnit, to: TempUnit) -> f64 {
    match (from, to) {
        (TempUnit::Farenheit, TempUnit::Celsius) => (temp - 32.0) / 1.8,
        (TempUnit::Celsius, TempUnit::Farenheit) => (temp * 1.8) + 32.0,
        _ => unreachable!(),
    }
}
