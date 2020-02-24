macro_rules! consensus_encoding {
    ($entity:ident, $($key:ident),+) => (
        impl ::consensus::Encodable for $entity {
            #[inline]
            fn encode<Wr: ::std::io::Write>(
                &self, 
                mut w: Wr,
            ) -> Result<usize, ::consensus::encode::Error> {
                let mut length = 0;
                $(length += self.$key.encode(&mut w)?;)+
                Ok(len)
            }
        }

        impl ::consensus::Decodable for $entity {
            #[inline]
            fn decode<Decoder: ::std::io::Read>(
                mut d: Decoder,
            ) -> Result<$entity, ::consensus::encode::Error> {
                Ok($entity {
                    $($key ::consensus::Decodable::decode(&mut d)?),+
                })
            }
        }
    );
}
