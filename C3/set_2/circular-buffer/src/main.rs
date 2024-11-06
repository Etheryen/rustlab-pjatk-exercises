use buffer::CircularBuffer;

mod buffer;

fn main() {
    let mut circular_buffer = CircularBuffer::new(4);

    let data1: [u8; 3] = [1, 2, 3];
    let data2: [u8; 3] = [4, 5, 6];

    circular_buffer.write(&data1);
    circular_buffer.write(&data2);

    println!("buffer: {:?}", circular_buffer.get_buffer());

    assert_eq!(circular_buffer.get_buffer(), [5, 6, 3, 4]);
}
