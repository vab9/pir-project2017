use std::str::FromStr;
use std::io;

/// Mnist type that contains the data of the pictures
#[derive(Debug, Clone)]
pub struct Mnist {
    /// This is the result of the data
    /// for Mnist its a number between 0 and 9
    result: u8,
    /// This is the Vector that contains the data of a Picture
    /// The data is normalized and contains values form 0.0 to 1.0
    data: Vec<f32>,
}

impl Mnist {
    /// returns the class as an u8
    pub fn get_class(&self) -> u8 {
        self.result.clone()
    }
    /// returns slice of the data
    pub fn get_slice(&self) -> &Vec<f32> {
        &self.data
    }
}

impl FromStr for Mnist {
    type Err = io::Error;

    /// Parse a Mnistdata from a String
    ///
    /// Returns a `Result<data, io::Error>`, in case the incoming string cannot
    /// be parsed into a valid Mnistdata.
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let v: Vec<&str> = s.split(',').collect();
        let mut data: Vec<f32> = Vec::with_capacity(v.len() - 1);
        // gets result for the mnistdata
        let res = v[0].parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        for i in 1..v.len() {
            // pushing data into the vector
            data.push(v[i].parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?);
            // normalizing
            data[i - 1] /= 255.0;
        }

        Ok(Mnist {
            result: res,
            data: data,
        })

    }
}
