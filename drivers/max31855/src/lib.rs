use embedded_hal::digital::OutputPin;
use embedded_hal::blocking::spi::Transfer;

struct Max31855<SPI> {
    spi: SPI
}

impl Max31855<SPI> where SPI: Transfer<u8, Error = E> {
    pub fn new(spi: SPI) -> Self {
        Max31855 { spi }
    }

    fn spi_read(&mut self) -> Result<Raw, E> {
        let data = self.SPI.transfer(&[0u8;4])?;

    }
}

struct Raw {
    thermocouple_temp: i16,
    junction_temp: i16,
}