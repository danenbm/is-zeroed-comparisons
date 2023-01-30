use std::time::Instant;

const SIZE: usize = 4000;

fn main() {
    let data = [0u8; SIZE];
    let sliced = &data[..];

    let start = Instant::now();
    let result = is_zeroed_byte_by_byte(sliced);
    let duration = start.elapsed();
    println!("{} byte by byte: {}, {:?}", SIZE, result, duration);

    let start = Instant::now();
    let result = is_zeroed_chunks(sliced, 4);
    let duration = start.elapsed();
    println!("{} in 4 byte chunks: {}, {:?}", SIZE, result, duration);

    let start = Instant::now();
    let result = is_zeroed_chunks(sliced, 8);
    let duration = start.elapsed();
    println!("{} in 8 byte chunks: {}, {:?}", SIZE, result, duration);

    let start = Instant::now();
    let result = is_zeroed_chunks(sliced, 16);
    let duration = start.elapsed();
    println!("{} in 16 byte chunks: {}, {:?}", SIZE, result, duration);

    let start = Instant::now();
    let result = is_zeroed_u128(sliced);
    let duration = start.elapsed();
    println!("{} as u128 chunks: {}, {:?}", SIZE, result, duration);

    let start = Instant::now();
    let result = is_zeroed_chunks(sliced, 1024);
    let duration = start.elapsed();
    println!("{} in 1024 chunks: {}, {:?}", SIZE, result, duration);
}

fn is_zeroed_byte_by_byte(data: &[u8]) -> bool {
    data.iter().all(|&x| x == 0)
}

fn is_zeroed_u128(data: &[u8]) -> bool {
    let mut chunks = data.chunks_exact(16);

    chunks.all(|chunk| {
        let arr: [u8; 16] = chunk.try_into().unwrap();
        u128::from_be_bytes(arr) == 0u128
    }) && chunks.remainder().iter().all(|&x| x == 0)
}

/// See if a slice contains all zeroes.  Useful for checking an account's data.
pub fn is_zeroed_chunks(buf: &[u8], chunk_size: usize) -> bool {
    const ZEROS_LEN: usize = 1024;
    const ZEROS: [u8; ZEROS_LEN] = [0; ZEROS_LEN];

    let mut chunks = buf.chunks_exact(chunk_size);

    #[allow(clippy::indexing_slicing)]
    {
        chunks.all(|chunk| chunk == &ZEROS[..chunk_size])
            && chunks.remainder() == &ZEROS[..chunks.remainder().len()]
    }
}
