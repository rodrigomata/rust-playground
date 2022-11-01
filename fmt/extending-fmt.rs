use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, buffer: &mut fmt::Formatter) -> fmt::Result {
        let vector_reference = &self.0;
        write!(buffer, "[")?;

        for (index, value) in vector_reference.iter().enumerate() {
            if index != 0 {
                write!(buffer, ", ")?;
            }
            write!(buffer, "{}: {}", index, value)?;
        }

        write!(buffer, "]")
    }
}

fn main() {
    let vector = List(vec![1, 2, 3]);
    println!("{}", vector);
}
