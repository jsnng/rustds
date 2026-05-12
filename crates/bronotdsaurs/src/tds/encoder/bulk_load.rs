use crate::tds::prelude::*;
use transport::AsyncTransport;

/// BulkLoadBCP/BulkLoadTWT are the only two operations in TDS that utilise a token stream.
#[allow(async_fn_in_trait)]
pub trait StreamEncoder<T: AsyncTransport, C> {
    /// push a row to be encoded into bytes
    fn push(&mut self, columns: Vec<C>) -> Result<(), EncodeError>;
    async fn flush(&mut self) -> Result<(), T::Error>;
    fn is_dirty(&self) -> bool;
    async fn done(self, done_token: DoneToken) -> Result<(), T::Error>;
}

pub struct BulkLoadBCP<T: AsyncTransport, R, C> {
    encoder: R,
    _t: core::marker::PhantomData<T>,
    _c: core::marker::PhantomData<C>,
}

impl<T: AsyncTransport> BulkLoadBCP<T, Rows<T>, Vec<u8>> {
    pub async fn new(mut transport: T, col_metadata: ColMetaDataToken) -> Result<Self, T::Error> {
        transport.write(&col_metadata.as_bytes()).await?;
        Ok(Self {
            encoder: Rows::new(col_metadata, transport),
            _t: core::marker::PhantomData,
            _c: core::marker::PhantomData,
        })
    }
}

#[cfg(feature = "tds7.3b")]
impl<T: AsyncTransport> BulkLoadBCP<T, NbcRows<T>, Option<Vec<u8>>> {
    pub async fn new_nbc(mut transport: T, col_metadata: ColMetaDataToken) -> Result<Self, T::Error> {
        transport.write(&col_metadata.as_bytes()).await?;
        Ok(Self {
            encoder: NbcRows::new(col_metadata, transport),
            _t: core::marker::PhantomData,
            _c: core::marker::PhantomData,
        })
    }
}


impl<T: AsyncTransport, R: StreamEncoder<T, C>, C> BulkLoadBCP<T, R, C> {
    pub fn push(&mut self, columns: Vec<C>) -> Result<(), EncodeError> {
        self.encoder.push(columns)
    }

    pub async fn flush(&mut self) -> Result<(), T::Error> {
        self.encoder.flush().await
    }

    pub fn is_dirty(&self) -> bool {
        self.encoder.is_dirty()
    }

    pub async fn done(self, done_token: DoneToken) -> Result<(), T::Error> {
        self.encoder.done(done_token).await
    }
}

struct BulkLoadTWT<T: AsyncTransport> {
    transport: T,
    dirty: bool,
    buf: Vec<u8>,
}

impl<T: AsyncTransport> BulkLoadTWT<T> {
    pub fn new(transport: T) -> Self {
        Self { transport, dirty: false, buf: Vec::new() }
    }
}

impl<T: AsyncTransport> StreamEncoder<T, u8> for BulkLoadTWT<T> {
    fn push(&mut self, columns: Vec<u8>) -> Result<(), EncodeError> {
        if self.dirty {
            return Err(EncodeError::PreviousRowNotFlushed);
        }
        self.buf.clear();
        self.buf.extend_from_slice(&columns);
        self.dirty = true;
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), T::Error> {
        self.transport.write(&self.buf).await?;
        self.dirty = false;
        Ok(())
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    async fn done(mut self, done_token: DoneToken) -> Result<(), T::Error> {
        self.transport.write(&done_token.as_bytes()).await?;
        Ok(())
    }
}

struct Rows<T: AsyncTransport> {
    col_metadata: ColMetaDataToken,
    transport: T,
    dirty: bool,
    buf: Vec<u8>,
}

impl<T: AsyncTransport> Rows<T> {
    fn new(col_metadata: ColMetaDataToken, transport: T) -> Self {
        Self { col_metadata, transport, dirty: false, buf: Vec::new() }
    }
}

impl<T: AsyncTransport> StreamEncoder<T, Vec<u8>> for Rows<T> {
    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn push(&mut self, columns: Vec<Vec<u8>>) -> Result<(), EncodeError> {
        if self.is_dirty() {
            return Err(EncodeError::PreviousRowNotFlushed);
        }
        self.buf.clear();
        self.buf.push(DataTokenType::Row as u8);
        for (bytes, col_metadata) in columns.iter().zip(self.col_metadata.column_data.iter()) {
            match col_metadata.type_info.dtype_max_len {
                Some(TypeInfoVarLen::Byte(_))   => self.buf.push(bytes.len() as u8),
                Some(TypeInfoVarLen::Ushort(_)) => self.buf.extend_from_slice(&(bytes.len() as u16).to_le_bytes()),
                Some(TypeInfoVarLen::Long(_))   => self.buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes()),
                None => {}
            }
            self.buf.extend_from_slice(bytes);
        }
        self.dirty = true;
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), T::Error> {
        self.transport.write(&self.buf).await?;
        self.dirty = false;
        Ok(())
    }

    async fn done(mut self, done_token: DoneToken) -> Result<(), T::Error> {
        self.transport.write(&done_token.as_bytes()).await?;
        Ok(())
    }
}

#[cfg(feature = "tds7.3b")]
struct NbcRows<T: AsyncTransport> {
    col_metadata: ColMetaDataToken,
    transport: T,
    dirty: bool,
    buf: Vec<u8>,
}

#[cfg(feature = "tds7.3b")]
impl<T: AsyncTransport> NbcRows<T> {
    fn new(col_metadata: ColMetaDataToken, transport: T) -> Self {
        Self { col_metadata, transport, dirty: false, buf: Vec::new() }
    }
}

#[cfg(feature = "tds7.3b")]
impl<T: AsyncTransport> StreamEncoder<T, Option<Vec<u8>>> for NbcRows<T> {
    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn push(&mut self, columns: Vec<Option<Vec<u8>>>) -> Result<(), EncodeError> {
        if self.dirty {
            return Err(EncodeError::PreviousRowNotFlushed);
        }
        self.buf.clear();
        self.buf.push(DataTokenType::NbcRow as u8);
        let col_count = self.col_metadata.column_data.len();
        let bitmap = col_count.div_ceil(8);
        let cursor = self.buf.len();
        self.buf.resize(cursor + bitmap, 0);
        let mut idx = cursor;        
        let mut mask = 1u8;
        for (col, col_meta) in columns.iter().zip(self.col_metadata.column_data.iter()) {
            match col {
                None => self.buf[idx] |= 1 << mask,
                Some(bytes) => {
                    match col_meta.type_info.dtype_max_len {
                        Some(TypeInfoVarLen::Byte(_))   => self.buf.push(bytes.len() as u8),
                        Some(TypeInfoVarLen::Ushort(_)) => self.buf.extend_from_slice(&(bytes.len() as u16).to_le_bytes()),
                        Some(TypeInfoVarLen::Long(_))   => self.buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes()),
                        None => {}
                    }
                    self.buf.extend_from_slice(bytes);
                }
            }
            mask = mask.rotate_left(1);
            idx += (mask == 1) as usize;
        }
        self.dirty = true;
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), T::Error> {
        self.transport.write(&self.buf).await?;
        self.dirty = false;
        Ok(())
    }

    async fn done(mut self, done_token: DoneToken) -> Result<(), T::Error> {
        self.transport.write(&done_token.as_bytes()).await?;
        Ok(())
    }
}
