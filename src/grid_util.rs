use ndarray::Array2;

pub fn make_byte_grid(raw_inp: &str) -> Array2<u8> {
    let columns = raw_inp
        .bytes()
        .position(|c| c == b'\n')
        .expect("can't get column count");

    let non_newline_bytes = raw_inp.bytes().filter(|&x| x != b'\n').collect::<Vec<_>>();

    Array2::from_shape_vec(
        (non_newline_bytes.len() / columns, columns),
        non_newline_bytes,
    )
    .expect("can't make array")
}
