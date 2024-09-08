use super::super::frontend::ast;
use super::values;

pub fn evaluate_variable_type(
    value_type: Option<ast::IdentifierExpression>,
    identifier: String,
    runtime_val: values::RuntimeValue,
) -> Result<values::RuntimeValue, String> {
    if value_type.is_none() {
        Ok(runtime_val)
    } else {
        match value_type.to_owned().unwrap().symbol.as_str() {
            "i8" => {
                if runtime_digit_is_i8(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_i8() {
                        Ok(integer8) => Ok(integer8.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "i16" => {
                if runtime_digit_is_i16(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_i16() {
                        Ok(integer16) => Ok(integer16.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "i32" => {
                if runtime_digit_is_i32(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_i32() {
                        Ok(integer32) => Ok(integer32.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "i64" => {
                if runtime_digit_is_i64(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_i64() {
                        Ok(integer64) => Ok(integer64.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "i128" => {
                if runtime_digit_is_i128(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_i128() {
                        Ok(integer128) => Ok(integer128.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "u8" => {
                if runtime_digit_is_u8(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_u8() {
                        Ok(unsigned_integer8) => Ok(unsigned_integer8.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "u16" => {
                if runtime_digit_is_u16(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_u16() {
                        Ok(unsigned_integer16) => Ok(unsigned_integer16.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "u32" => {
                if runtime_digit_is_u32(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_u32() {
                        Ok(unsigned_integer32) => Ok(unsigned_integer32.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "u64" => {
                if runtime_digit_is_u64(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_u64() {
                        Ok(unsigned_integer64) => Ok(unsigned_integer64.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "u128" => {
                if runtime_digit_is_u128(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_u128() {
                        Ok(unsigned_integer128) => Ok(unsigned_integer128.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "f32" => {
                if runtime_digit_is_f32(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_f32() {
                        Ok(float32) => Ok(float32.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "f64" => {
                if runtime_digit_is_f64(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    match runtime_val.as_f64() {
                        Ok(float64) => Ok(float64.as_raw()),
                        Err(m) => Err(m),
                    }
                }
            }
            "char" => {
                if runtime_is_char(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    return Err(format!(
                        "Incorrect runtime value for {}, expected: char, got: {:#?}",
                        identifier,
                        runtime_val.as_value_type()
                    ));
                }
            }
            "str" => {
                if runtime_is_str(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    return Err(format!(
                        "Incorrect runtime value for {}, expected: str, got: {:#?}",
                        identifier,
                        runtime_val.as_value_type()
                    ));
                }
            }
            "bool" => {
                if runtime_is_bool(&runtime_val) {
                    Ok(runtime_val)
                } else {
                    return Err(format!(
                        "Incorrect runtime value for {}, expected: bool, got: {:#?}",
                        identifier,
                        runtime_val.as_value_type()
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "Unexpected variable type given during variable declaration/assignment evaluation, got {}",
                    value_type.unwrap().symbol.as_str()
                ))
            }
        }
    }
}

pub fn runtime_value_is_digit(runtime_value: &values::RuntimeValue) -> bool {
    if runtime_value_is_integer(&runtime_value) || runtime_value_is_float(&runtime_value) {
        return true;
    }
    false
}

pub fn runtime_value_is_float(runtime_value: &values::RuntimeValue) -> bool {
    match &runtime_value {
        values::RuntimeValue::Float32(_) | values::RuntimeValue::Float64(_) => true,
        _ => false,
    }
}

pub fn runtime_value_is_integer(runtime_value: &values::RuntimeValue) -> bool {
    match &runtime_value {
        values::RuntimeValue::Integer8(_)
        | values::RuntimeValue::Integer16(_)
        | values::RuntimeValue::Integer32(_)
        | values::RuntimeValue::Integer64(_)
        | values::RuntimeValue::Integer128(_)
        | values::RuntimeValue::UnsignedInteger8(_)
        | values::RuntimeValue::UnsignedInteger16(_)
        | values::RuntimeValue::UnsignedInteger32(_)
        | values::RuntimeValue::UnsignedInteger64(_)
        | values::RuntimeValue::UnsignedInteger128(_) => true,
        _ => false,
    }
}

pub fn runtime_digit_is_i8(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::Integer8(_) => true,
        _ => false,
    }
}

pub fn runtime_digit_is_i16(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::Integer16(_) => true,
        _ => false,
    }
}

pub fn runtime_digit_is_i32(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::Integer32(_) => true,
        _ => false,
    }
}

pub fn runtime_digit_is_i64(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::Integer64(_) => true,
        _ => false,
    }
}

pub fn runtime_digit_is_i128(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::Integer128(_) => true,
        _ => false,
    }
}

pub fn runtime_digit_is_u8(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::UnsignedInteger8(_) => true,
        _ => false,
    }
}

pub fn runtime_digit_is_u16(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::UnsignedInteger16(_) => true,
        _ => false,
    }
}

pub fn runtime_digit_is_u32(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::UnsignedInteger32(_) => true,
        _ => false,
    }
}

pub fn runtime_digit_is_u64(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::UnsignedInteger64(_) => true,
        _ => false,
    }
}

pub fn runtime_digit_is_u128(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::UnsignedInteger128(_) => true,
        _ => false,
    }
}

pub fn runtime_digit_is_f32(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::Float32(_) => true,
        _ => false,
    }
}

pub fn runtime_digit_is_f64(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::Float64(_) => true,
        _ => false,
    }
}

pub fn runtime_is_char(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::Character(_) => true,
        _ => false,
    }
}

pub fn runtime_is_str(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::String(_) => true,
        _ => false,
    }
}

pub fn runtime_is_bool(runtime_value: &values::RuntimeValue) -> bool {
    match runtime_value {
        values::RuntimeValue::Boolean(_) => true,
        _ => false,
    }
}

pub fn cast_to_i8<T>(value: T) -> Result<i8, String>
where
    T: Into<i128> + Copy,
{
    let value_as_i128 = value.into();
    if value_as_i128 > i8::MAX as i128 {
        Err(format!(
            "Value {} is too large to cast into an i8",
            value_as_i128
        ))
    } else if value_as_i128 < i8::MIN as i128 {
        Err(format!(
            "Value {} is too small to cast into an i8",
            value_as_i128
        ))
    } else {
        Ok(value_as_i128 as i8)
    }
}

pub fn cast_u128_to_i8(value: u128) -> Result<i8, String> {
    if value > i8::MAX as u128 {
        Err(format!("Value {} is too large to cast into an i8", value))
    } else {
        Ok(value as i8)
    }
}

pub fn cast_to_i16<T>(value: T) -> Result<i16, String>
where
    T: Into<i128> + Copy,
{
    let value_as_i128 = value.into();
    if value_as_i128 > i16::MAX as i128 {
        Err(format!(
            "Value {} is too large to cast into an i16",
            value_as_i128
        ))
    } else if value_as_i128 < i16::MIN as i128 {
        Err(format!(
            "Value {} is too small to cast into an i16",
            value_as_i128
        ))
    } else {
        Ok(value_as_i128 as i16)
    }
}

pub fn cast_u128_to_i16(value: u128) -> Result<i16, String> {
    if value > i16::MAX as u128 {
        Err(format!("Value {} is too large to cast into an i16", value))
    } else {
        Ok(value as i16)
    }
}

pub fn cast_to_i32<T>(value: T) -> Result<i32, String>
where
    T: Into<i128> + Copy,
{
    let value_as_i128 = value.into();
    if value_as_i128 > i32::MAX as i128 {
        Err(format!(
            "Value {} is too large to cast into an i32",
            value_as_i128
        ))
    } else if value_as_i128 < i32::MIN as i128 {
        Err(format!(
            "Value {} is too small to cast into an i32",
            value_as_i128
        ))
    } else {
        Ok(value_as_i128 as i32)
    }
}

pub fn cast_u128_to_i32(value: u128) -> Result<i32, String> {
    if value > i32::MAX as u128 {
        Err(format!("Value {} is too large to cast into an i32", value))
    } else {
        Ok(value as i32)
    }
}

pub fn cast_to_i64<T>(value: T) -> Result<i64, String>
where
    T: Into<i128> + Copy,
{
    let value_as_i128 = value.into();
    if value_as_i128 > i64::MAX as i128 {
        Err(format!(
            "Value {} is too large to cast into an i64",
            value_as_i128
        ))
    } else if value_as_i128 < i64::MIN as i128 {
        Err(format!(
            "Value {} is too small to cast into an i64",
            value_as_i128
        ))
    } else {
        Ok(value_as_i128 as i64)
    }
}

pub fn cast_u128_to_i64(value: u128) -> Result<i64, String> {
    if value > i64::MAX as u128 {
        Err(format!("Value {} is too large to cast into an i64", value))
    } else {
        Ok(value as i64)
    }
}

pub fn cast_to_i128<T>(value: T) -> Result<i128, String>
where
    T: Into<i128> + Copy,
{
    let value_as_i128 = value.into();
    if value_as_i128 > i128::MAX as i128 {
        Err(format!(
            "Value {} is too large to cast into an i128",
            value_as_i128
        ))
    } else if value_as_i128 < i128::MIN as i128 {
        Err(format!(
            "Value {} is too small to cast into an i128",
            value_as_i128
        ))
    } else {
        Ok(value_as_i128 as i128)
    }
}

pub fn cast_u128_to_i128(value: u128) -> Result<i128, String> {
    if value > i128::MAX as u128 {
        Err(format!("Value {} is too large to cast into an i128", value))
    } else {
        Ok(value as i128)
    }
}

pub fn cast_to_u8<T>(value: T) -> Result<u8, String>
where
    T: Into<i128> + Copy,
{
    let value_as_i128 = value.into();
    if value_as_i128 > u8::MAX as i128 {
        Err(format!(
            "Value {} is too large to cast into an u8",
            value_as_i128
        ))
    } else if value_as_i128 < u8::MIN as i128 {
        Err(format!(
            "Value {} is too small to cast into an u8",
            value_as_i128
        ))
    } else {
        Ok(value_as_i128 as u8)
    }
}

pub fn cast_u128_to_u8(value: u128) -> Result<u8, String> {
    if value > u8::MAX as u128 {
        Err(format!("Value {} is too large to cast into an u8", value))
    } else {
        Ok(value as u8)
    }
}

pub fn cast_to_u16<T>(value: T) -> Result<u16, String>
where
    T: Into<i128> + Copy,
{
    let value_as_i128 = value.into();
    if value_as_i128 > u16::MAX as i128 {
        Err(format!(
            "Value {} is too large to cast into an u16",
            value_as_i128
        ))
    } else if value_as_i128 < u16::MIN as i128 {
        Err(format!(
            "Value {} is too small to cast into an u16",
            value_as_i128
        ))
    } else {
        Ok(value_as_i128 as u16)
    }
}

pub fn cast_u128_to_u16(value: u128) -> Result<u16, String> {
    if value > u16::MAX as u128 {
        Err(format!("Value {} is too large to cast into an u16", value))
    } else {
        Ok(value as u16)
    }
}

pub fn cast_to_u32<T>(value: T) -> Result<u32, String>
where
    T: Into<i128> + Copy,
{
    let value_as_i128 = value.into();
    if value_as_i128 > u32::MAX as i128 {
        Err(format!(
            "Value {} is too large to cast into an u32",
            value_as_i128
        ))
    } else if value_as_i128 < u32::MIN as i128 {
        Err(format!(
            "Value {} is too small to cast into an u32",
            value_as_i128
        ))
    } else {
        Ok(value_as_i128 as u32)
    }
}

pub fn cast_u128_to_u32(value: u128) -> Result<u32, String> {
    if value > u32::MAX as u128 {
        Err(format!("Value {} is too large to cast into an u32", value))
    } else {
        Ok(value as u32)
    }
}

pub fn cast_to_u64<T>(value: T) -> Result<u64, String>
where
    T: Into<i128> + Copy,
{
    let value_as_i128 = value.into();
    if value_as_i128 > u64::MAX as i128 {
        Err(format!(
            "Value {} is too large to cast into an u64",
            value_as_i128
        ))
    } else if value_as_i128 < u64::MIN as i128 {
        Err(format!(
            "Value {} is too small to cast into an u64",
            value_as_i128
        ))
    } else {
        Ok(value_as_i128 as u64)
    }
}

pub fn cast_u128_to_u64(value: u128) -> Result<u64, String> {
    if value > u64::MAX as u128 {
        Err(format!("Value {} is too large to cast into an u64", value))
    } else {
        Ok(value as u64)
    }
}

pub fn cast_to_u128<T>(value: T) -> Result<u128, String>
where
    T: Into<i128> + Copy,
{
    let value_as_i128 = value.into();
    if value_as_i128 > u128::MAX as i128 {
        Err(format!(
            "Value {} is too large to cast into an u128",
            value_as_i128
        ))
    } else if value_as_i128 < u128::MIN as i128 {
        Err(format!(
            "Value {} is too small to cast into an u128",
            value_as_i128
        ))
    } else {
        Ok(value_as_i128 as u128)
    }
}

pub fn cast_to_f32<T>(value: T) -> Result<f32, String>
where
    T: Into<f32> + Copy,
{
    Ok(value.into())
}

pub fn cast_i32_to_f32(value: i32) -> Result<f32, String> {
    Ok(value as f32)
}

pub fn cast_i64_to_f32(value: i64) -> Result<f32, String> {
    Ok(value as f32)
}

pub fn cast_u32_to_f32(value: u32) -> Result<f32, String> {
    Ok(value as f32)
}

pub fn cast_u64_to_f32(value: u64) -> Result<f32, String> {
    Ok(value as f32)
}

pub fn cast_u128_to_f32(value: u128) -> Result<f32, String> {
    Ok(value as f32)
}

pub fn cast_i128_to_f32(value: i128) -> Result<f32, String> {
    Ok(value as f32)
}

pub fn cast_to_f64<T>(value: T) -> Result<f64, String>
where
    T: Into<f64> + Copy,
{
    Ok(value.into())
}

pub fn cast_u128_to_f64(value: u128) -> Result<f64, String> {
    Ok(value as f64)
}

pub fn cast_i64_to_f64(value: i64) -> Result<f64, String> {
    Ok(value as f64)
}

pub fn cast_u64_to_f64(value: u64) -> Result<f64, String> {
    Ok(value as f64)
}

pub fn cast_i128_to_f64(value: i128) -> Result<f64, String> {
    Ok(value as f64)
}
