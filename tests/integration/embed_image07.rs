// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Image, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let image = Image::new("tests/input/images/red.png")?;
    worksheet.embed_image(0, 0, &image)?;

    let image = Image::new("tests/input/images/blue.png")?;
    worksheet.embed_image(2, 0, &image)?;

    let image = Image::new("tests/input/images/red.png")?.set_alt_text("red.png");
    worksheet.insert_image(8, 4, &image)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_embed_image07() {
    let test_runner = common::TestRunner::new()
        .set_name("embed_image07")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
