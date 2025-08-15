pub mod flags_numeric {
    use bitflags::Flags;

    pub fn serialize<B, S>(flags: &B, serializer: S) -> Result<S::Ok, S::Error>
    where
        B: Flags,
        B::Bits: ::serde::Serialize,
        S: ::serde::Serializer,
    {
        ::serde::Serialize::serialize(&flags.bits(), serializer)
    }

    pub fn deserialize<'de, B, D>(deserializer: D) -> Result<B, D::Error>
    where
        B: Flags,
        B::Bits: ::serde::Deserialize<'de> + Copy,
        D: ::serde::Deserializer<'de>,
    {
        let bits = <B::Bits as ::serde::Deserialize<'de>>::deserialize(deserializer)?;
        Ok(B::from_bits_truncate(bits))
    }
}

pub mod flags_numeric_opt {
    use bitflags::Flags;
    use serde::Deserialize;

    pub fn serialize<B, S>(v: &Option<B>, serializer: S) -> Result<S::Ok, S::Error>
    where
        B: Flags,
        B::Bits: ::serde::Serialize,
        S: ::serde::Serializer,
    {
        match v {
            Some(flags) => ::serde::Serialize::serialize(&flags.bits(), serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, B, D>(deserializer: D) -> Result<Option<B>, D::Error>
    where
        B: Flags,
        B::Bits: ::serde::Deserialize<'de> + Copy,
        D: ::serde::Deserializer<'de>,
    {
        let bits_opt = Option::<B::Bits>::deserialize(deserializer)?;
        Ok(bits_opt.map(B::from_bits_truncate))
    }
}
