
// Implmentation of Serialization and Deserialization for struct of 2 members with u32 type

use std::fmt::Error;

trait Serialization {
    fn serialize(&self) -> Vec<u8>;
}

trait Deserialization {
    fn deserialize(&self ,v:&Vec<u8>) -> Result<Swap, Error>;
}


#[derive(Debug)]
struct Swap {
    qty_0: u32,
    qty_1: u32
}

impl Serialization for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        v.extend_from_slice(&self.qty_0.to_be_bytes());
        v.extend_from_slice(&self.qty_1.to_be_bytes());
        
        return v;
    }
}


impl Deserialization for Swap {
    fn deserialize(&self,v:&Vec<u8>) -> Result<Swap, Error> {
        
        if v.len() > 8 {
            return Err(Error)
        }
        
        let qty_0 = u32::from_be_bytes(v[0..4].try_into().unwrap());
        let qty_1 = u32::from_be_bytes(v[4..8].try_into().unwrap());
        
        return Ok(Swap { qty_0, qty_1 });
    }
}

pub fn run() {
    let s = Swap {
        qty_0: 5,
        qty_1: 6,
    };

    let s_value = s.serialize();

    println!("The serialized value is {:?}", s_value);

    println!("The deserialized value is {:?}", s.deserialize(&s_value).unwrap());

}



