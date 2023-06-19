trait Times {
    type DoubleItem;
    fn double(&self) -> Self::DoubleItem;

    type QuadItem;
    fn quadruple(&self) -> Self::QuadItem;
}

struct SignedBinary(i8);

struct UnsignedBinary(u8);


impl Times for SignedBinary {
    type DoubleItem = i16;
    fn double(&self) -> Self::DoubleItem {
        self.0 as i16
    }

    type QuadItem = i32;
    fn quadruple(&self) -> Self::QuadItem {
        self.0 as i32
    }
}

impl Times for UnsignedBinary {
    type DoubleItem = u16;
    fn double(&self) -> Self::DoubleItem {
        self.0 as u16
    }

    type QuadItem = u32;
    fn quadruple(&self) -> Self::QuadItem {
        self.0 as u32
    }
}
