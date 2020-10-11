use declio::ctx::Len;
use declio::{Decode, Encode, Error};
use std::convert::TryInto;
use std::io;
use std::marker::PhantomData;

pub(crate) struct LengthPrefix<L> {
    _l: PhantomData<L>,
}

impl<L> LengthPrefix<L> {
    pub(crate) fn encode<T, Ctx, W>(
        this: &Vec<T>,
        inner_ctx: Ctx,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        T: Encode<Ctx>,
        Ctx: Clone,
        W: io::Write,
        L: Encode,
        Len: TryInto<L>,
        <Len as TryInto<L>>::Error: std::error::Error + Send + Sync + 'static,
    {
        Len(this.len())
            .try_into()
            .map_err(Error::wrap)?
            .encode((), writer)?;
        this.encode(inner_ctx, writer)?;
        Ok(())
    }

    pub(crate) fn decode<T, Ctx, R>(inner_ctx: Ctx, reader: &mut R) -> Result<Vec<T>, Error>
    where
        T: Decode<Ctx>,
        Ctx: Clone,
        R: io::Read,
        L: Decode + TryInto<Len>,
        <L as TryInto<Len>>::Error: std::error::Error + Send + Sync + 'static,
    {
        let len = L::decode((), reader)?.try_into().map_err(Error::wrap)?;
        Vec::decode((len, inner_ctx), reader)
    }
}

pub(crate) struct Greedy {}

impl Greedy {
    pub(crate) fn encode<W>(this: &Vec<u8>, _: (), writer: &mut W) -> Result<(), Error>
    where
        W: io::Write,
    {
        this.encode((), writer)
    }

    pub(crate) fn decode<R>(_: (), reader: &mut R) -> Result<Vec<u8>, Error>
    where
        R: io::Read,
    {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(buf)
    }
}
