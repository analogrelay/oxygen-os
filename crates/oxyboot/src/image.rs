use elf::{ElfBytes, endian::AnyEndian};

pub struct Image<'a> {
    pub bytes: &'a [u8],
    pub elf: ElfBytes<'a, AnyEndian>,
}

impl<'a> TryFrom<&'a [u8]> for Image<'a> {
    type Error = elf::ParseError;

    fn try_from(value: &'a [u8]) -> Result<Image<'a>, elf::ParseError> {
        ElfBytes::<AnyEndian>::minimal_parse(value)
            .map(|elf| Image { bytes: value, elf })
    }
}