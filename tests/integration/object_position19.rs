// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Image, Workbook, XlsxError};

// Test to demonstrate object positioning options.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_column_hidden(1)?;

    let image = Image::new("tests/input/images/red.png")?.set_alt_text("red.png");

    worksheet.insert_image_with_offset(8, 1, &image, 128, 0)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_object_position19() {
    let test_runner = common::TestRunner::new()
        .set_name("object_position19")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
