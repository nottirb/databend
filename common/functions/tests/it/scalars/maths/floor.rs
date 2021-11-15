// Copyright 2020 Datafuse Lfloor.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_datavalues::prelude::*;
use common_exception::ErrorCode;
use common_exception::Result;
use common_functions::scalars::*;

#[test]
fn test_floor_function() -> Result<()> {
    #[allow(dead_code)]
    struct Test {
        name: &'static str,
        func: Box<dyn Function>,
        arg: DataColumnWithField,
        expect: Result<DataColumn>,
    }
    let tests = vec![
        Test {
            name: "floor(123)",
            func: FloorFunction::try_create("floor")?,
            arg: DataColumnWithField::new(
                Series::new([123]).into(),
                DataField::new("arg1", DataType::Int32, false),
            ),
            expect: Ok(Series::new(vec![123_f64]).into()),
        },
        Test {
            name: "floor(1.7)",
            func: FloorFunction::try_create("floor")?,
            arg: DataColumnWithField::new(
                Series::new([1.7]).into(),
                DataField::new("arg1", DataType::Float64, false),
            ),
            expect: Ok(Series::new(vec![1_f64]).into()),
        },
        Test {
            name: "floor(-2.1)",
            func: FloorFunction::try_create("floor")?,
            arg: DataColumnWithField::new(
                Series::new([-2.1]).into(),
                DataField::new("arg1", DataType::Float64, false),
            ),
            expect: Ok(Series::new(vec![-3_f64]).into()),
        },
        Test {
            name: "floor('123')",
            func: FloorFunction::try_create("floor")?,
            arg: DataColumnWithField::new(
                Series::new(["123"]).into(),
                DataField::new("arg1", DataType::String, true),
            ),
            expect: Ok(Series::new(vec![123_f64]).into()),
        },
        Test {
            name: "floor('+123.8a1')",
            func: FloorFunction::try_create("floor")?,
            arg: DataColumnWithField::new(
                Series::new(["+123.8a1"]).into(),
                DataField::new("arg1", DataType::String, true),
            ),
            expect: Ok(Series::new(vec![123_f64]).into()),
        },
        Test {
            name: "floor('-123.2a1')",
            func: FloorFunction::try_create("floor")?,
            arg: DataColumnWithField::new(
                Series::new(["-123.2a1"]).into(),
                DataField::new("arg1", DataType::String, true),
            ),
            expect: Ok(Series::new(vec![-124_f64]).into()),
        },
        Test {
            name: "floor('a')",
            func: FloorFunction::try_create("floor")?,
            arg: DataColumnWithField::new(
                Series::new(["a"]).into(),
                DataField::new("arg1", DataType::String, true),
            ),
            expect: Ok(Series::new(vec![0_f64]).into()),
        },
        Test {
            name: "floor('a123')",
            func: FloorFunction::try_create("floor")?,
            arg: DataColumnWithField::new(
                Series::new(["a123"]).into(),
                DataField::new("arg1", DataType::String, true),
            ),
            expect: Ok(Series::new(vec![0_f64]).into()),
        },
        Test {
            name: "floor(true)",
            func: FloorFunction::try_create("floor")?,
            arg: DataColumnWithField::new(
                Series::new([true]).into(),
                DataField::new("arg1", DataType::Boolean, true),
            ),
            expect: Err(ErrorCode::IllegalDataType(
                "Expected numeric types, but got Boolean",
            )),
        },
    ];

    for t in tests {
        let func = t.func;
        let got = func.eval(&[t.arg.clone()], 1);
        match t.expect {
            Ok(expected) => {
                assert_eq!(&got.unwrap(), &expected, "case: {}", t.name);
            }
            Err(expected_err) => {
                assert_eq!(got.unwrap_err().to_string(), expected_err.to_string());
            }
        }
    }
    Ok(())
}