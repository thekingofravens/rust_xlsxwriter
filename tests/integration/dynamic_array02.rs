// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Formula, Workbook, XlsxError};

// Test case to test dynamic array formula: with explicit prefix.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_dynamic_formula(0, 1, "=_xlfn.UNIQUE(A1)")?;
    worksheet.write_number(0, 0, 0)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case to test dynamic array formula: with implicit prefix.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_dynamic_formula(0, 1, "=UNIQUE(A1)")?;
    worksheet.write_number(0, 0, 0)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case to test dynamic array formula: with standard formula function.
fn create_new_xlsx_file_3(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_formula(0, 1, "=UNIQUE(A1)")?;
    worksheet.write_number(0, 0, 0)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case to test dynamic array formula: with standard array formula
// function.
fn create_new_xlsx_file_4(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_array_formula(0, 1, 0, 1, "=UNIQUE(A1)")?;
    worksheet.write_number(0, 0, 0)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case to test dynamic array formula: with generics
fn create_new_xlsx_file_5(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write(0, 1, Formula::new("=UNIQUE(A1)"))?;
    worksheet.write_number(0, 0, 0)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_dynamic_array02_1() {
    let test_runner = common::TestRunner::new()
        .set_name("dynamic_array02")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_dynamic_array02_2() {
    let test_runner = common::TestRunner::new()
        .set_name("dynamic_array02")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_dynamic_array02_3() {
    let test_runner = common::TestRunner::new()
        .set_name("dynamic_array02")
        .set_function(create_new_xlsx_file_3)
        .unique("3")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_dynamic_array02_4() {
    let test_runner = common::TestRunner::new()
        .set_name("dynamic_array02")
        .set_function(create_new_xlsx_file_4)
        .unique("4")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_dynamic_array02_5() {
    let test_runner = common::TestRunner::new()
        .set_name("dynamic_array02")
        .set_function(create_new_xlsx_file_5)
        .unique("5")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}