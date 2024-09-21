// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let mut bold = Format::new().set_bold();
    let mut num_format = Format::new().set_num_format("0.00");

    // Register the formats.
    workbook.register_format(&mut num_format);
    workbook.register_format(&mut bold);

    let worksheet = workbook.add_worksheet().set_constant_memory_mode(true)?;

    worksheet.write(1, 1, "Apple")?;
    worksheet.write(1, 3, 1.23456)?;
    worksheet.write_with_format(2, 3, 1.23456, &num_format)?;
    worksheet.write_with_format(3, 1, "Pear", &bold)?;
    worksheet.write(5, 1, "Orange")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize20() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize20")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}