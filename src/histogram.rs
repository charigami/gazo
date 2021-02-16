use std::convert::From;

/// [Histogram](https://en.wikipedia.org/wiki/Histogram) for maximally 256 intensity values
#[derive(Debug, Clone)]
pub struct Histogram {
    data: [u32; 256],
}

impl Histogram {
    /// Returns the count for a given intensity value
    pub fn get_count(&self, intensity: u8) -> u32 {
        // it is safe because intensity is u8 and don't
        // exceeds the maximal range of the histogram
        unsafe { *self.data.get_unchecked(intensity as usize) }
    }

    /// Returns the lowest intensity count and the corresponding intensity value (index)
    pub fn get_low(&self) -> (u32, usize) {
        let mut low_count = 0;
        let mut intensity = 0;
        for (i, val) in self.data.iter().enumerate() {
            if *val != 0 {
                low_count = *val;
                intensity = i;
                break;
            }
        }
        (low_count, intensity)
    }

    /// Returns the highest intensity count and the corresponding intensity value (index)
    pub fn get_high(&self) -> (u32, usize) {
        let mut high_count = 0;
        let mut intensity = 0;
        for (i, val) in self.data.iter().rev().enumerate() {
            if *val != 0 {
                high_count = *val;
                intensity = 255usize - i;
                break;
            }
        }
        (high_count, intensity)
    }

    /// [Cumulates](https://en.wikipedia.org/wiki/Histogram#Cumulative_histogram) the histogram
    pub fn cumulate(&mut self) {
        for i in 1..256 {
            self.data[i] = &self.data[i] + &self.data[i - 1];
        }
    }
}

impl From<[u32; 256]> for Histogram {
    fn from(data: [u32; 256]) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_historgram_count() {
        let mut data = [0u32; 256];

        for i in 0..256 {
            data[i] = i as u32;
        }

        let histogram = Histogram::from(data);

        assert_eq!(histogram.get_count(1), 1);
    }

    #[test]
    fn test_get_historgram_low_and_high() {
        let mut data = [0u32; 256];

        for i in 0..256 {
            data[i] = i as u32;
        }

        let histogram = Histogram::from(data);

        let low = histogram.get_low();
        let high = histogram.get_high();

        assert_eq!(low, (1,1));
        assert_eq!(high, (255,255));
    }

    #[test]
    fn test_cumulate_histogram() {
        let mut data = [0u32; 256];

        for i in 0..256 {
            data[i] = i as u32;
        }

        let mut histogram = Histogram::from(data);

        histogram.cumulate();

        assert_eq!((histogram.get_count(1), histogram.get_count(2), histogram.get_count(3)), (1,3,6));
    }
}
