use anyhow::Result;

pub trait Encoder {
    fn encode(&self) -> Result<Vec<u8>>;
}

impl Encoder for u64 {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.to_be_bytes().to_vec())
    }
}

impl Encoder for String {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.as_bytes().to_vec())
    }
}


pub struct Event<Id, Data> {
    id: Id,
    data: Data,
}

impl<Id, Data> Event<Id, Data>
    where Id: Encoder,
          Data: Encoder
{
    pub fn new(id: Id, data: Data) -> Self {
        Self { id, data }
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut result = self.id.encode()?;
        result.append(&mut self.data.encode()?);
        Ok(result)
    }
}


// trade 对 泛型 参数 约束
fn main() {
    let event = Event::new(1, "hello world".to_string());
    let _res1 = event.encode();

    let event2 = Event::new("hi".to_string(), "hello world".to_string());
    let _res2 = event2.encode();
}