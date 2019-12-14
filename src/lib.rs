pub mod temperatures {
    pub enum Temperature {
        F(f64),
        C(f64),
    }

    /// Convert Fahrenheit to Celsius
    fn f_to_c(f: f64) -> f64 {
        (f - 32.0) * (5.0 / 9.0)
    }

    /// Convert Celsius to Fahrenheit
    fn c_to_f(c: f64) -> f64 {
        c * (9.0 / 5.0) + 32.0
    }

    /// Convert temperatures
    fn convert(temperature: &Temperature) -> f64 {
        match temperature {
            &Temperature::F(degrees) => f_to_c(degrees),
            &Temperature::C(degrees) => c_to_f(degrees),
        }
    }

    /// Print degrees Fahrenheit to Celsius
    fn print_f_to_c(degrees: f64) {
        println!(
            "\n{:.2}ºF = {:.2}ºC\n",
            degrees,
            convert(&Temperature::F(degrees))
        );
    }

    /// Print degrees Celsius to Fahrenheit
    fn print_c_to_f(degrees: f64) {
        println!(
            "\n{:.2}ºC = {:.2}ºF\n",
            degrees,
            convert(&Temperature::C(degrees))
        );
    }

    /// Print temperature conversions
    pub fn print_temperature(temperature: &Temperature) {
        match temperature {
            &Temperature::F(degrees) => print_f_to_c(degrees),
            &Temperature::C(degrees) => print_c_to_f(degrees),
        }
    }
}

/// Run some tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0c_to_f() {
        assert_eq!(
            temperatures::convert(&temperatures::Temperature::C(0.0)),
            32.0
        );
    }

    #[test]
    fn test_100c_to_f() {
        assert_eq!(
            temperatures::convert(&temperatures::Temperature::C(100.0)),
            212.0
        );
    }

    #[test]
    fn test_32f_to_c() {
        assert_eq!(
            temperatures::convert(&temperatures::Temperature::F(32.0)),
            0.0
        );
    }

    #[test]
    fn test_212f_to_c() {
        assert_eq!(
            temperatures::convert(&temperatures::Temperature::F(212.0)),
            100.0
        );
    }

    #[test]
    fn test_98_6f_to_c() {
        assert_eq!(
            temperatures::convert(&temperatures::Temperature::F(98.6)),
            37.0
        );
    }

    #[test]
    fn test_neg_40() {
        assert_eq!(
            temperatures::convert(&temperatures::Temperature::C(-40.0)),
            -40.0
        );
    }
}