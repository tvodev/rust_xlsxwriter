// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{
    CustomSerializeField, ExcelDateTime, ExcelSerialize, Format, SerializeFieldOptions, Workbook,
    XlsxError,
};
use serde::Serialize;

#[cfg(feature = "chrono")]
use chrono::{NaiveDate, NaiveDateTime};

#[cfg(feature = "chrono")]
use rust_xlsxwriter::utility::serialize_chrono_naive_to_excel;

#[cfg(feature = "chrono")]
use rust_xlsxwriter::utility::serialize_chrono_option_naive_to_excel;

// Test case for Serde serialization. First test isn't serialized.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(1, 11)?;

    let format = Format::new().set_num_format_index(14);

    // Not serialized.
    worksheet.write(0, 0, "col1")?;
    worksheet.write(1, 0, "aaa")?;
    worksheet.write(2, 0, "bbb")?;
    worksheet.write(3, 0, "ccc")?;

    worksheet.write(0, 1, "col2")?;
    worksheet.write_with_format(1, 1, ExcelDateTime::parse_from_str("2024-01-01")?, &format)?;
    worksheet.write_with_format(2, 1, ExcelDateTime::parse_from_str("2024-01-02")?, &format)?;
    worksheet.write_with_format(3, 1, ExcelDateTime::parse_from_str("2024-01-03")?, &format)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(1, 11)?;

    let format = Format::new().set_num_format_index(14);

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col1: &'static str,
        col2: ExcelDateTime,
    }

    let data1 = MyStruct {
        col1: "aaa",
        col2: ExcelDateTime::parse_from_str("2024-01-01")?,
    };

    let data2 = MyStruct {
        col1: "bbb",
        col2: ExcelDateTime::parse_from_str("2024-01-02")?,
    };

    let data3 = MyStruct {
        col1: "ccc",
        col2: ExcelDateTime::parse_from_str("2024-01-03")?,
    };

    let custom_headers = [
        CustomSerializeField::new("col1"),
        CustomSerializeField::new("col2").set_value_format(&format),
    ];
    let header_options = SerializeFieldOptions::new().set_custom_headers(&custom_headers);

    worksheet.serialize_headers_with_options(0, 0, &data1, &header_options)?;

    worksheet.serialize(&data1)?;
    worksheet.serialize(&data2)?;
    worksheet.serialize(&data3)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization with chrono.
#[cfg(feature = "chrono")]
fn create_new_xlsx_file_3(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(1, 11)?;

    let format = Format::new().set_num_format_index(14);

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col1: &'static str,
        #[serde(serialize_with = "serialize_chrono_naive_to_excel")]
        col2: NaiveDate,
    }

    let data1 = MyStruct {
        col1: "aaa",
        col2: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    };

    let data2 = MyStruct {
        col1: "bbb",
        col2: NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
    };

    let data3 = MyStruct {
        col1: "ccc",
        col2: NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
    };

    let custom_headers = [
        CustomSerializeField::new("col1"),
        CustomSerializeField::new("col2").set_value_format(&format),
    ];
    let header_options = SerializeFieldOptions::new().set_custom_headers(&custom_headers);

    worksheet.serialize_headers_with_options(0, 0, &data1, &header_options)?;

    worksheet.serialize(&data1)?;
    worksheet.serialize(&data2)?;
    worksheet.serialize(&data3)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization with chrono.
#[cfg(feature = "chrono")]
fn create_new_xlsx_file_4(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(1, 11)?;

    let format = Format::new().set_num_format_index(14);

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col1: &'static str,
        #[serde(serialize_with = "serialize_chrono_naive_to_excel")]
        col2: NaiveDateTime,
    }

    let data1 = MyStruct {
        col1: "aaa",
        col2: NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };

    let data2 = MyStruct {
        col1: "bbb",
        col2: NaiveDate::from_ymd_opt(2024, 1, 2)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };

    let data3 = MyStruct {
        col1: "ccc",
        col2: NaiveDate::from_ymd_opt(2024, 1, 3)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };

    let custom_headers = [
        CustomSerializeField::new("col1"),
        CustomSerializeField::new("col2").set_value_format(&format),
    ];
    let header_options = SerializeFieldOptions::new().set_custom_headers(&custom_headers);

    worksheet.serialize_headers_with_options(0, 0, &data1, &header_options)?;

    worksheet.serialize(&data1)?;
    worksheet.serialize(&data2)?;
    worksheet.serialize(&data3)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization with chrono Option<>.
#[cfg(feature = "chrono")]
fn create_new_xlsx_file_5(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(1, 11)?;

    let format = Format::new().set_num_format_index(14);

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col1: &'static str,
        #[serde(serialize_with = "serialize_chrono_option_naive_to_excel")]
        col2: Option<NaiveDate>,
    }

    let data1 = MyStruct {
        col1: "aaa",
        col2: NaiveDate::from_ymd_opt(2024, 1, 1),
    };

    let data2 = MyStruct {
        col1: "bbb",
        col2: NaiveDate::from_ymd_opt(2024, 1, 2),
    };

    let data3 = MyStruct {
        col1: "ccc",
        col2: NaiveDate::from_ymd_opt(2024, 1, 3),
    };

    let custom_headers = [
        CustomSerializeField::new("col1"),
        CustomSerializeField::new("col2").set_value_format(&format),
    ];
    let header_options = SerializeFieldOptions::new().set_custom_headers(&custom_headers);

    worksheet.serialize_headers_with_options(0, 0, &data1, &header_options)?;

    worksheet.serialize(&data1)?;
    worksheet.serialize(&data2)?;
    worksheet.serialize(&data3)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization. With ExcelSerialize.
fn create_new_xlsx_file_6(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(1, 11)?;

    // Create a serializable test struct.
    #[derive(Serialize, ExcelSerialize)]
    struct MyStruct {
        col1: &'static str,

        #[rust_xlsxwriter(value_format = Format::new().set_num_format_index(14))]
        col2: ExcelDateTime,
    }

    let data1 = MyStruct {
        col1: "aaa",
        col2: ExcelDateTime::parse_from_str("2024-01-01")?,
    };

    let data2 = MyStruct {
        col1: "bbb",
        col2: ExcelDateTime::parse_from_str("2024-01-02")?,
    };

    let data3 = MyStruct {
        col1: "ccc",
        col2: ExcelDateTime::parse_from_str("2024-01-03")?,
    };

    worksheet.set_serialize_headers::<MyStruct>(0, 0)?;

    worksheet.serialize(&data1)?;
    worksheet.serialize(&data2)?;
    worksheet.serialize(&data3)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_serde10_1() {
    let test_runner = common::TestRunner::new()
        .set_name("serde10")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde10_2() {
    let test_runner = common::TestRunner::new()
        .set_name("serde10")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
#[cfg(feature = "chrono")]
fn test_serde10_3() {
    let test_runner = common::TestRunner::new()
        .set_name("serde10")
        .set_function(create_new_xlsx_file_3)
        .unique("3")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
#[cfg(feature = "chrono")]
fn test_serde10_4() {
    let test_runner = common::TestRunner::new()
        .set_name("serde10")
        .set_function(create_new_xlsx_file_4)
        .unique("4")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
#[cfg(feature = "chrono")]
fn test_serde10_5() {
    let test_runner = common::TestRunner::new()
        .set_name("serde10")
        .set_function(create_new_xlsx_file_5)
        .unique("5")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde10_6() {
    let test_runner = common::TestRunner::new()
        .set_name("serde10")
        .set_function(create_new_xlsx_file_6)
        .unique("6")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
