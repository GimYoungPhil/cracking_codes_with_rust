trait Times<T, U> {
    fn double(&self) -> T;

    fn quadruple(&self) -> U;
}

struct SignedBinary(i8);

struct UnsignedBinary(u8);

impl<T, U> Times<T, U> for SignedBinary {
    fn double(&self) -> i16 {
        self.0 as i16
    }

    fn quadruple(&self) -> i32 {
        self.0 as i32
    }
}
