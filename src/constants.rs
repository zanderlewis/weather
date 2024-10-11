use num_bigint::BigInt;
use num_rational::BigRational;

// Kelvin constant
pub fn kelvin_constant() -> BigRational {
    BigRational::new(BigInt::from(27315), BigInt::from(100))
}

// Pi constant
pub fn pi_constant() -> BigRational {
    BigRational::new(
        BigInt::parse_bytes(b"31415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679", 10).unwrap(),
        BigInt::parse_bytes(b"10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", 10).unwrap()
    )
}

// Gas constant for dry air (J/(kg·K))
pub fn rd_constant() -> BigRational {
    BigRational::new(BigInt::from(28705), BigInt::from(100))
}

// Specific heat capacity of air at constant pressure (J/(kg·K))
pub fn cp_constant() -> BigRational {
    BigRational::new(BigInt::from(1005), BigInt::from(1))
}

// Standard atmospheric pressure (Pa)
pub fn p0_constant() -> BigRational {
    BigRational::new(BigInt::from(101325), BigInt::from(1))
}

// Latent heat of vaporization for water (J/kg)
pub fn lv_constant() -> BigRational {
    BigRational::new(BigInt::from(2260000), BigInt::from(1))
}

// Specific heat capacity of water (J/(kg·K))
pub fn cw_constant() -> BigRational {
    BigRational::new(BigInt::from(4184), BigInt::from(1))
}

// Density of air (kg/m³)
pub fn rho_air_constant() -> BigRational {
    BigRational::new(BigInt::from(1200), BigInt::from(1))
}

// Density of water (kg/m³)
pub fn rho_water_constant() -> BigRational {
    BigRational::new(BigInt::from(1000), BigInt::from(1))
}

// Acceleration due to gravity (m/s²)
pub fn g_constant() -> BigRational {
    BigRational::new(BigInt::from(981), BigInt::from(100))
}