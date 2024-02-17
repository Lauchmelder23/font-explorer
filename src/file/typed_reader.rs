use super::common::new_error;

pub struct TypedReader {
    data: Vec<u8>,
    head: usize
}

impl From<Vec<u8>> for TypedReader {
    fn from(value: Vec<u8>) -> Self {
        TypedReader {
            data: value,
            head: 0
        }
    }
}

macro_rules! read_func {
    ($type: ty, $name: ident) => {
        pub fn $name(&mut self) -> std::io::Result<$type> {
            let type_size = (<$type>::BITS >> 3) as usize;
            if self.data.len() - self.head < type_size {
                return Err(
                    new_error!("No more space left in buffer to read {}. Buffer size: {}, head pos: {}", 
                        stringify!($type), 
                        self.data.len(), 
                        self.head
                    )
                );
            }

            let result = <$type>::from_be_bytes(self.data[self.head..self.head + type_size].try_into().unwrap());
            self.head += type_size;

            Ok(result)
        }
    };
}

impl TypedReader {
    read_func!(u8,  read_u8);
    read_func!(u16, read_u16);
    read_func!(u32, read_u32);
    read_func!(u64, read_u64);

    read_func!(i8,  read_i8);
    read_func!(i16, read_i16);
    read_func!(i32, read_i32);
    read_func!(i64, read_i64);
}
