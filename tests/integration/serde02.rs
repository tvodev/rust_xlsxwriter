// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{ExcelSerialize, Workbook, XlsxError};
use serde::Serialize;

// Test case for Serde serialization. First test isn't serialized.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Not serialized.
    worksheet.write(0, 0, "col1")?;
    worksheet.write(1, 0, "aaa")?;
    worksheet.write(0, 1, "col2")?;
    worksheet.write(1, 1, "a")?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col1: &'static str,
        col2: char,
    }

    let data = MyStruct {
        col1: "aaa",
        col2: 'a',
    };

    worksheet.serialize_headers(0, 0, &data)?;
    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization. Test Serde container rename.
fn create_new_xlsx_file_3(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create a serializable test struct.
    #[derive(Serialize, ExcelSerialize)]
    #[serde(rename = "MyStruct2")]
    struct MyStruct {
        col1: &'static str,
        col2: char,
    }

    let data = MyStruct {
        col1: "aaa",
        col2: 'a',
    };

    worksheet.set_serialize_headers::<MyStruct>(0, 0)?;
    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_serde02_1() {
    let test_runner = common::TestRunner::new()
        .set_name("serde02")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde02_2() {
    let test_runner = common::TestRunner::new()
        .set_name("serde02")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde02_3() {
    let test_runner = common::TestRunner::new()
        .set_name("serde02")
        .set_function(create_new_xlsx_file_3)
        .unique("3")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
