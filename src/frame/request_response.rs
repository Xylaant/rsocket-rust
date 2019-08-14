extern crate bytes;

use crate::frame::{Body, Frame, Writeable, FLAG_METADATA, U24};
use bytes::{BigEndian, BufMut, Bytes, BytesMut};

#[derive(Debug, Clone)]
pub struct RequestResponse {
  metadata: Option<Bytes>,
  data: Option<Bytes>,
}

pub struct RequestResponseBuilder {
  stream_id: u32,
  flag: u16,
  value: RequestResponse,
}

impl RequestResponseBuilder {
  fn new(stream_id: u32, flag: u16) -> RequestResponseBuilder {
    RequestResponseBuilder {
      stream_id: stream_id,
      flag: flag,
      value: RequestResponse {
        metadata: None,
        data: None,
      },
    }
  }

  pub fn set_metadata(&mut self, metadata: Bytes) -> &mut RequestResponseBuilder {
    self.value.metadata = Some(metadata);
    self.flag |= FLAG_METADATA;
    self
  }

  pub fn set_data(&mut self, data: Bytes) -> &mut RequestResponseBuilder {
    self.value.data = Some(data);
    self
  }

  pub fn build(&mut self) -> Frame {
    Frame::new(
      self.stream_id,
      Body::RequestResponse(self.value.clone()),
      self.flag,
    )
  }
}

impl Writeable for RequestResponse {
  fn write_to(&self, bf: &mut BytesMut) {
    match &self.metadata {
      Some(v) => {
        U24::write(v.len() as u32, bf);
        bf.put(v);
      }
      None => (),
    }
    match &self.data {
      Some(v) => bf.put(v),
      None => (),
    }
  }

  fn len(&self) -> u32 {
    let mut n: u32 = 0;
    match &self.metadata {
      Some(v) => {
        n += 3;
        n += v.len() as u32;
      }
      None => (),
    }
    match &self.data {
      Some(v) => n += v.len() as u32,
      None => (),
    }
    n
  }
}

impl RequestResponse {
  pub fn decode(flag: u16, bf: &mut Bytes) -> Option<RequestResponse> {
    let mut m: Option<Bytes> = None;
    if flag & FLAG_METADATA != 0 {
      let n = U24::advance(bf);
      m = Some(bf.split_to(n as usize));
    }
    let mut d: Option<Bytes> = None;
    if !bf.is_empty() {
      d = Some(Bytes::from(bf.to_vec()));
    }
    Some(RequestResponse {
      metadata: m,
      data: d,
    })
  }

  pub fn builder(stream_id: u32, flag: u16) -> RequestResponseBuilder {
    RequestResponseBuilder::new(stream_id, flag)
  }

  pub fn get_metadata(&self) -> Option<Bytes> {
    self.metadata.clone()
  }

  pub fn get_data(&self) -> Option<Bytes> {
    self.data.clone()
  }
}