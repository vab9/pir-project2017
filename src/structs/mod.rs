use std::fmt;
use na::{DVector,DMatrix};
use serde_json;
use serde::{Serialize,Serializer, Deserialize, Deserializer};
use serde_derive;
use serde::ser::SerializeSeq;
use std::ops::Deref;

/// enum flower names
#[derive(Debug, Clone, Copy)]
pub enum FlowerName {
    IrisSetosa,
    IrisVersicolor,
    IrisVirginica,
}

/// flowertype that contains the 4 inputs and the Flowername
#[derive(Debug, Clone, Copy)]
pub struct Flower {
    name: FlowerName,
    sepal_length: f32,
    sepal_width: f32,
    petal_length: f32,
    petal_width: f32,
}

/// constuctor for the Flowertype
impl Flower {
    pub fn new(n: FlowerName, sl: f32, sw: f32, pl: f32, pw: f32) -> Flower {
        Flower {
            name: n,
            sepal_length: sl,
            sepal_width: sw,
            petal_length: pl,
            petal_width: pw,
        }
    }
}

/// easy printing of the flower type
impl fmt::Display for Flower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "({:?}, {}, {}, {}, {})",
               self.name,
               self.sepal_length,
               self.sepal_width,
               self.petal_length,
               self.petal_width)
    }
}

impl From <Flower> for DVector<f32> {
    fn from(fl: Flower) -> DVector<f32> {

        DVector::from_slice(4,&[fl.sepal_length,
                                fl.sepal_width,
                                fl.petal_length,
                                fl.petal_width])
    }
}
pub struct WrapDVector{
    DVec: DVector<f32>
}
impl Serialize for WrapDVector
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
    {
        let mut seq = serializer.serialize_seq(Some(self.DVec.len()))?;
        for e in 0..self.DVec.len() {
            seq.serialize_element(&self.DVec[e])?;
        }
        seq.end()
    }
}

impl Deref for WrapDVector {
    type Target = DVector<f32>;

    fn deref(&self) -> &self::Target {
        &self.DVec
    }
}

pub struct WrapDMatrix(DMatrix<f32>);
impl Serialize for WrapDMatrix
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
    {
        let mut seq = serializer.serialize_seq(Some(self.0.nrows()*self.0.ncols()))?;
        for e in self.0.as_vector() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}
