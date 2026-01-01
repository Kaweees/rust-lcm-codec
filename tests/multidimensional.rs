extern crate generated;
extern crate rust_lcm_codec;

type TestError = rust_lcm_codec::CodecError<
    rust_lcm_codec::BufferReaderError,
    rust_lcm_codec::BufferWriterError,
>;

#[test]
fn matrix2d_round_trip_happy_path() -> Result<(), TestError> {
    let mut buf = [0u8; 512];
    let rows = 2;
    let cols = 3;
    let matrix_values = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];

    let n_bytes_written = {
        let mut w = rust_lcm_codec::BufferWriter::new(&mut buf);
        let pw = generated::multidimensional::begin_matrix2d_t_write(&mut w)?;
        let pw = pw.write_rows(rows)?;
        let mut pw: generated::multidimensional::matrix2d_t_write_matrix<_> =
            pw.write_cols(cols)?;

        // Write the 2D array: iterate over rows, then columns
        // The state after writing cols is already an iterator over the first dimension
        for row in &matrix_values {
            let mut row_writer = pw.next().unwrap();
            for &val in row {
                row_writer.next().unwrap().write(val)?;
            }
        }

        let _write_done = pw.done()?;
        w.cursor()
    };

    let mut r = rust_lcm_codec::BufferReader::new(&mut buf);
    let pr = generated::multidimensional::begin_matrix2d_t_read(&mut r)?;
    let (found_rows, pr) = pr.read_rows()?;
    let (found_cols, mut pr) = pr.read_cols()?;
    assert_eq!(rows, found_rows);
    assert_eq!(cols, found_cols);

    let mut read_matrix = vec![vec![0.0; cols as usize]; rows as usize];

    // Read the 2D array: iterate over rows, then columns
    // The state after reading cols is already an iterator over the first dimension
    let mut row_idx = 0;
    while let Some(mut row_reader) = pr.next() {
        for col_idx in 0..cols as usize {
            read_matrix[row_idx][col_idx] = row_reader.next().unwrap().read()?;
        }
        row_idx += 1;
    }
    let _read_done = pr.done()?;

    assert_eq!(&read_matrix, &matrix_values);
    assert_eq!(n_bytes_written, r.cursor());
    Ok(())
}

#[test]
fn static_matrix2d_round_trip_happy_path() -> Result<(), TestError> {
    let mut buf = [0u8; 512];
    let matrix_values = [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
    ];

    let n_bytes_written = {
        let mut w = rust_lcm_codec::BufferWriter::new(&mut buf);
        let mut pw = generated::multidimensional::begin_static_matrix2d_t_write(&mut w)?;

        // Write the 2D array: iterate over rows, then columns
        // The ready state is already an iterator over the first dimension
        for row in &matrix_values {
            let mut row_writer = pw.next().unwrap();
            for &val in row {
                row_writer.next().unwrap().write(val)?;
            }
        }

        let _write_done = pw.done()?;
        w.cursor()
    };

    let mut r = rust_lcm_codec::BufferReader::new(&mut buf);
    let mut pr = generated::multidimensional::begin_static_matrix2d_t_read(&mut r)?;
    let mut read_matrix = [[0.0; 4]; 3];

    // Read the 2D array: iterate over rows, then columns
    // The ready state is already an iterator over the first dimension
    let mut row_idx = 0;
    while let Some(mut row_reader) = pr.next() {
        for col_idx in 0..4 {
            read_matrix[row_idx][col_idx] = row_reader.next().unwrap().read()?;
        }
        row_idx += 1;
    }
    let _read_done = pr.done()?;

    assert_eq!(&read_matrix, &matrix_values);
    assert_eq!(n_bytes_written, r.cursor());
    Ok(())
}

#[test]
fn mixed_matrix_round_trip_happy_path() -> Result<(), TestError> {
    let mut buf = [0u8; 512];
    let rows = 2;
    let matrix_values = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];

    let n_bytes_written = {
        let mut w = rust_lcm_codec::BufferWriter::new(&mut buf);
        let pw = generated::multidimensional::begin_mixed_matrix_t_write(&mut w)?;
        let mut pw: generated::multidimensional::mixed_matrix_t_write_matrix<_> =
            pw.write_rows(rows)?;

        // Write the 2D array: iterate over rows, then columns (cols is static = 3)
        // The state after writing rows is already an iterator over the first dimension
        for row in &matrix_values {
            let mut row_writer = pw.next().unwrap();
            for &val in row {
                row_writer.next().unwrap().write(val)?;
            }
        }

        let _write_done = pw.done()?;
        w.cursor()
    };

    let mut r = rust_lcm_codec::BufferReader::new(&mut buf);
    let pr = generated::multidimensional::begin_mixed_matrix_t_read(&mut r)?;
    let (found_rows, mut pr) = pr.read_rows()?;
    assert_eq!(rows, found_rows);

    let mut read_matrix = vec![[0.0; 3]; rows as usize];

    // Read the 2D array: iterate over rows, then columns
    // The state after reading rows is already an iterator over the first dimension
    let mut row_idx = 0;
    while let Some(mut row_reader) = pr.next() {
        for col_idx in 0..3 {
            read_matrix[row_idx][col_idx] = row_reader.next().unwrap().read()?;
        }
        row_idx += 1;
    }
    let _read_done = pr.done()?;

    assert_eq!(&read_matrix, &matrix_values);
    assert_eq!(n_bytes_written, r.cursor());
    Ok(())
}
