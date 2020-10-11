macro_rules! impl_declio_from_json {
    ($t:ty) => {
        impl declio::Encode for $t {
            fn encode<W>(&self, _: (), writer: &mut W) -> Result<(), declio::Error>
            where
                W: std::io::Write,
            {
                let string = serde_json::to_string(self).map_err(declio::Error::wrap)?;
                $crate::proto::types::String(string).encode((), writer)
            }
        }

        impl declio::Decode for $t {
            fn decode<R>(_: (), reader: &mut R) -> Result<Self, declio::Error>
            where
                R: std::io::Read,
            {
                let string = $crate::proto::types::String::decode((), reader)?.0;
                serde_json::from_str(&string).map_err(declio::Error::wrap)
            }
        }
    };
}
