use super::helpers;

#[derive(Debug, Clone)]
pub enum ValueType {
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Integer128,
    UnsignedInteger8,
    UnsignedInteger16,
    UnsignedInteger32,
    UnsignedInteger64,
    UnsignedInteger128,
    Float32,
    Float64,
    String,
    Character,
    Boolean,
    Null,
}

impl ValueType {
    pub fn as_string(self) -> String {
        match self {
            ValueType::Integer8 => String::from("i8"),
            ValueType::Integer16 => String::from("i16"),
            ValueType::Integer32 => String::from("i32"),
            ValueType::Integer64 => String::from("i64"),
            ValueType::Integer128 => String::from("i128"),
            ValueType::UnsignedInteger8 => String::from("u8"),
            ValueType::UnsignedInteger16 => String::from("u16"),
            ValueType::UnsignedInteger32 => String::from("u32"),
            ValueType::UnsignedInteger64 => String::from("u64"),
            ValueType::UnsignedInteger128 => String::from("u128"),
            ValueType::Float32 => String::from("f32"),
            ValueType::Float64 => String::from("f64"),
            ValueType::String => String::from("str"),
            ValueType::Character => String::from("char"),
            ValueType::Boolean => String::from("bool"),
            ValueType::Null => String::from("null"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Integer8(Integer8Value),
    Integer16(Integer16Value),
    Integer32(Integer32Value),
    Integer64(Integer64Value),
    Integer128(Integer128Value),
    UnsignedInteger8(UnsignedInteger8Value),
    UnsignedInteger16(UnsignedInteger16Value),
    UnsignedInteger32(UnsignedInteger32Value),
    UnsignedInteger64(UnsignedInteger64Value),
    UnsignedInteger128(UnsignedInteger128Value),
    Float32(Float32Value),
    Float64(Float64Value),
    String(StringValue),
    Character(CharacterValue),
    Boolean(BooleanValue),
    Null(NullValue),
}

impl RuntimeValue {
    pub fn as_value_type(self) -> ValueType {
        match self {
            RuntimeValue::Integer8(_) => ValueType::Integer8,
            RuntimeValue::Integer16(_) => ValueType::Integer16,
            RuntimeValue::Integer32(_) => ValueType::Integer32,
            RuntimeValue::Integer64(_) => ValueType::Integer64,
            RuntimeValue::Integer128(_) => ValueType::Integer128,
            RuntimeValue::UnsignedInteger8(_) => ValueType::UnsignedInteger8,
            RuntimeValue::UnsignedInteger16(_) => ValueType::UnsignedInteger16,
            RuntimeValue::UnsignedInteger32(_) => ValueType::UnsignedInteger32,
            RuntimeValue::UnsignedInteger64(_) => ValueType::UnsignedInteger64,
            RuntimeValue::UnsignedInteger128(_) => ValueType::UnsignedInteger128,
            RuntimeValue::Float32(_) => ValueType::Float32,
            RuntimeValue::Float64(_) => ValueType::Float64,
            RuntimeValue::String(_) => ValueType::String,
            RuntimeValue::Character(_) => ValueType::Character,
            RuntimeValue::Boolean(_) => ValueType::Boolean,
            RuntimeValue::Null(_) => ValueType::Null,
        }
    }

    pub fn as_i8(self) -> Result<Integer8Value, String> {
        if !helpers::runtime_value_is_integer(&self) {
            return Err(format!("Cannot cast {:#?} into an i8", self));
        }

        match self.to_owned() {
            RuntimeValue::Integer8(runtime_val) => Ok(runtime_val),
            RuntimeValue::Integer16(runtime_val) => {
                helpers::cast_to_i8(runtime_val.value).map(Integer8Value::create)
            }
            RuntimeValue::Integer32(runtime_val) => {
                helpers::cast_to_i8(runtime_val.value).map(Integer8Value::create)
            }
            RuntimeValue::Integer64(runtime_val) => {
                helpers::cast_to_i8(runtime_val.value).map(Integer8Value::create)
            }
            RuntimeValue::Integer128(runtime_val) => {
                helpers::cast_to_i8(runtime_val.value).map(Integer8Value::create)
            }
            RuntimeValue::UnsignedInteger8(runtime_val) => {
                helpers::cast_to_i8(runtime_val.value).map(Integer8Value::create)
            }
            RuntimeValue::UnsignedInteger16(runtime_val) => {
                helpers::cast_to_i8(runtime_val.value).map(Integer8Value::create)
            }
            RuntimeValue::UnsignedInteger32(runtime_val) => {
                helpers::cast_to_i8(runtime_val.value).map(Integer8Value::create)
            }
            RuntimeValue::UnsignedInteger64(runtime_val) => {
                helpers::cast_to_i8(runtime_val.value).map(Integer8Value::create)
            }
            RuntimeValue::UnsignedInteger128(runtime_val) => {
                helpers::cast_u128_to_i8(runtime_val.value).map(Integer8Value::create)
            }
            _ => Err(format!("Cannot cast {:#?} into an i8", self)),
        }
    }

    pub fn as_i16(self) -> Result<Integer16Value, String> {
        if !helpers::runtime_value_is_integer(&self) {
            return Err(format!("Cannot cast {:#?} into an i16", self));
        }

        match self.to_owned() {
            RuntimeValue::Integer8(runtime_val) => {
                Ok(Integer16Value::create(runtime_val.value as i16))
            }
            RuntimeValue::Integer16(runtime_val) => Ok(runtime_val),
            RuntimeValue::Integer32(runtime_val) => {
                helpers::cast_to_i16(runtime_val.value).map(Integer16Value::create)
            }
            RuntimeValue::Integer64(runtime_val) => {
                helpers::cast_to_i16(runtime_val.value).map(Integer16Value::create)
            }
            RuntimeValue::Integer128(runtime_val) => {
                helpers::cast_to_i16(runtime_val.value).map(Integer16Value::create)
            }
            RuntimeValue::UnsignedInteger8(runtime_val) => {
                helpers::cast_to_i16(runtime_val.value).map(Integer16Value::create)
            }
            RuntimeValue::UnsignedInteger16(runtime_val) => {
                helpers::cast_to_i16(runtime_val.value).map(Integer16Value::create)
            }
            RuntimeValue::UnsignedInteger32(runtime_val) => {
                helpers::cast_to_i16(runtime_val.value).map(Integer16Value::create)
            }
            RuntimeValue::UnsignedInteger64(runtime_val) => {
                helpers::cast_to_i16(runtime_val.value).map(Integer16Value::create)
            }
            RuntimeValue::UnsignedInteger128(runtime_val) => {
                helpers::cast_u128_to_i16(runtime_val.value).map(Integer16Value::create)
            }
            _ => Err(format!("Cannot cast {:#?} into an i8", self)),
        }
    }

    pub fn as_i32(self) -> Result<Integer32Value, String> {
        if !helpers::runtime_value_is_integer(&self) {
            return Err(format!("Cannot cast {:#?} into an i32", self));
        }

        match self.to_owned() {
            RuntimeValue::Integer8(runtime_val) => {
                Ok(Integer32Value::create(runtime_val.value as i32))
            }
            RuntimeValue::Integer16(runtime_val) => {
                Ok(Integer32Value::create(runtime_val.value as i32))
            }
            RuntimeValue::Integer32(runtime_val) => Ok(runtime_val),
            RuntimeValue::Integer64(runtime_val) => {
                helpers::cast_to_i32(runtime_val.value).map(Integer32Value::create)
            }
            RuntimeValue::Integer128(runtime_val) => {
                helpers::cast_to_i32(runtime_val.value).map(Integer32Value::create)
            }
            RuntimeValue::UnsignedInteger8(runtime_val) => {
                helpers::cast_to_i32(runtime_val.value).map(Integer32Value::create)
            }
            RuntimeValue::UnsignedInteger16(runtime_val) => {
                helpers::cast_to_i32(runtime_val.value).map(Integer32Value::create)
            }
            RuntimeValue::UnsignedInteger32(runtime_val) => {
                helpers::cast_to_i32(runtime_val.value).map(Integer32Value::create)
            }
            RuntimeValue::UnsignedInteger64(runtime_val) => {
                helpers::cast_to_i32(runtime_val.value).map(Integer32Value::create)
            }
            RuntimeValue::UnsignedInteger128(runtime_val) => {
                helpers::cast_u128_to_i32(runtime_val.value).map(Integer32Value::create)
            }
            _ => Err(format!("Cannot cast {:#?} into an i32", self)),
        }
    }

    pub fn as_i64(self) -> Result<Integer64Value, String> {
        if !helpers::runtime_value_is_integer(&self) {
            return Err(format!("Cannot cast {:#?} into an i64", self));
        }

        match self.to_owned() {
            RuntimeValue::Integer8(runtime_val) => {
                Ok(Integer64Value::create(runtime_val.value as i64))
            }
            RuntimeValue::Integer16(runtime_val) => {
                Ok(Integer64Value::create(runtime_val.value as i64))
            }
            RuntimeValue::Integer32(runtime_val) => {
                Ok(Integer64Value::create(runtime_val.value as i64))
            }
            RuntimeValue::Integer64(runtime_val) => Ok(runtime_val),
            RuntimeValue::Integer128(runtime_val) => {
                helpers::cast_to_i64(runtime_val.value).map(Integer64Value::create)
            }
            RuntimeValue::UnsignedInteger8(runtime_val) => {
                helpers::cast_to_i64(runtime_val.value).map(Integer64Value::create)
            }
            RuntimeValue::UnsignedInteger16(runtime_val) => {
                helpers::cast_to_i64(runtime_val.value).map(Integer64Value::create)
            }
            RuntimeValue::UnsignedInteger32(runtime_val) => {
                helpers::cast_to_i64(runtime_val.value).map(Integer64Value::create)
            }
            RuntimeValue::UnsignedInteger64(runtime_val) => {
                helpers::cast_to_i64(runtime_val.value).map(Integer64Value::create)
            }
            RuntimeValue::UnsignedInteger128(runtime_val) => {
                helpers::cast_u128_to_i64(runtime_val.value).map(Integer64Value::create)
            }
            _ => Err(format!("Cannot cast {:#?} into an i64", self)),
        }
    }

    pub fn as_i128(self) -> Result<Integer128Value, String> {
        if !helpers::runtime_value_is_integer(&self) {
            return Err(format!("Cannot cast {:#?} into an i128", self));
        }

        match self.to_owned() {
            RuntimeValue::Integer8(runtime_val) => {
                Ok(Integer128Value::create(runtime_val.value as i128))
            }
            RuntimeValue::Integer16(runtime_val) => {
                Ok(Integer128Value::create(runtime_val.value as i128))
            }
            RuntimeValue::Integer32(runtime_val) => {
                Ok(Integer128Value::create(runtime_val.value as i128))
            }
            RuntimeValue::Integer64(runtime_val) => {
                Ok(Integer128Value::create(runtime_val.value as i128))
            }
            RuntimeValue::Integer128(runtime_val) => Ok(runtime_val),
            RuntimeValue::UnsignedInteger8(runtime_val) => {
                helpers::cast_to_i128(runtime_val.value).map(Integer128Value::create)
            }
            RuntimeValue::UnsignedInteger16(runtime_val) => {
                helpers::cast_to_i128(runtime_val.value).map(Integer128Value::create)
            }
            RuntimeValue::UnsignedInteger32(runtime_val) => {
                helpers::cast_to_i128(runtime_val.value).map(Integer128Value::create)
            }
            RuntimeValue::UnsignedInteger64(runtime_val) => {
                helpers::cast_to_i128(runtime_val.value).map(Integer128Value::create)
            }
            RuntimeValue::UnsignedInteger128(runtime_val) => {
                helpers::cast_u128_to_i128(runtime_val.value).map(Integer128Value::create)
            }
            _ => Err(format!("Cannot cast {:#?} into an i128", self)),
        }
    }

    pub fn as_u8(self) -> Result<UnsignedInteger8Value, String> {
        if !helpers::runtime_value_is_integer(&self) {
            return Err(format!("Cannot cast {:#?} into an u8", self));
        }

        match self.to_owned() {
            RuntimeValue::Integer8(runtime_val) => {
                helpers::cast_to_u8(runtime_val.value).map(UnsignedInteger8Value::create)
            }
            RuntimeValue::Integer16(runtime_val) => {
                helpers::cast_to_u8(runtime_val.value).map(UnsignedInteger8Value::create)
            }
            RuntimeValue::Integer32(runtime_val) => {
                helpers::cast_to_u8(runtime_val.value).map(UnsignedInteger8Value::create)
            }
            RuntimeValue::Integer64(runtime_val) => {
                helpers::cast_to_u8(runtime_val.value).map(UnsignedInteger8Value::create)
            }
            RuntimeValue::Integer128(runtime_val) => {
                helpers::cast_to_u8(runtime_val.value).map(UnsignedInteger8Value::create)
            }
            RuntimeValue::UnsignedInteger8(runtime_val) => {
                helpers::cast_to_u8(runtime_val.value).map(UnsignedInteger8Value::create)
            }
            RuntimeValue::UnsignedInteger16(runtime_val) => {
                helpers::cast_to_u8(runtime_val.value).map(UnsignedInteger8Value::create)
            }
            RuntimeValue::UnsignedInteger32(runtime_val) => {
                helpers::cast_to_u8(runtime_val.value).map(UnsignedInteger8Value::create)
            }
            RuntimeValue::UnsignedInteger64(runtime_val) => {
                helpers::cast_to_u8(runtime_val.value).map(UnsignedInteger8Value::create)
            }
            RuntimeValue::UnsignedInteger128(runtime_val) => {
                helpers::cast_u128_to_u8(runtime_val.value).map(UnsignedInteger8Value::create)
            }
            _ => Err(format!("Cannot cast {:#?} into an u8", self)),
        }
    }

    pub fn as_u16(self) -> Result<UnsignedInteger16Value, String> {
        if !helpers::runtime_value_is_integer(&self) {
            return Err(format!("Cannot cast {:#?} into an 168", self));
        }

        match self.to_owned() {
            RuntimeValue::Integer8(runtime_val) => {
                helpers::cast_to_u16(runtime_val.value).map(UnsignedInteger16Value::create)
            }
            RuntimeValue::Integer16(runtime_val) => {
                helpers::cast_to_u16(runtime_val.value).map(UnsignedInteger16Value::create)
            }
            RuntimeValue::Integer32(runtime_val) => {
                helpers::cast_to_u16(runtime_val.value).map(UnsignedInteger16Value::create)
            }
            RuntimeValue::Integer64(runtime_val) => {
                helpers::cast_to_u16(runtime_val.value).map(UnsignedInteger16Value::create)
            }
            RuntimeValue::Integer128(runtime_val) => {
                helpers::cast_to_u16(runtime_val.value).map(UnsignedInteger16Value::create)
            }
            RuntimeValue::UnsignedInteger8(runtime_val) => {
                Ok(UnsignedInteger16Value::create(runtime_val.value as u16))
            }
            RuntimeValue::UnsignedInteger16(runtime_val) => {
                helpers::cast_to_u16(runtime_val.value).map(UnsignedInteger16Value::create)
            }
            RuntimeValue::UnsignedInteger32(runtime_val) => {
                helpers::cast_to_u16(runtime_val.value).map(UnsignedInteger16Value::create)
            }
            RuntimeValue::UnsignedInteger64(runtime_val) => {
                helpers::cast_to_u16(runtime_val.value).map(UnsignedInteger16Value::create)
            }
            RuntimeValue::UnsignedInteger128(runtime_val) => {
                helpers::cast_u128_to_u16(runtime_val.value).map(UnsignedInteger16Value::create)
            }
            _ => Err(format!("Cannot cast {:#?} into an u16", self)),
        }
    }

    pub fn as_u32(self) -> Result<UnsignedInteger32Value, String> {
        if !helpers::runtime_value_is_integer(&self) {
            return Err(format!("Cannot cast {:#?} into an u32", self));
        }

        match self.to_owned() {
            RuntimeValue::Integer8(runtime_val) => {
                helpers::cast_to_u32(runtime_val.value).map(UnsignedInteger32Value::create)
            }
            RuntimeValue::Integer16(runtime_val) => {
                helpers::cast_to_u32(runtime_val.value).map(UnsignedInteger32Value::create)
            }
            RuntimeValue::Integer32(runtime_val) => {
                helpers::cast_to_u32(runtime_val.value).map(UnsignedInteger32Value::create)
            }
            RuntimeValue::Integer64(runtime_val) => {
                helpers::cast_to_u32(runtime_val.value).map(UnsignedInteger32Value::create)
            }
            RuntimeValue::Integer128(runtime_val) => {
                helpers::cast_to_u32(runtime_val.value).map(UnsignedInteger32Value::create)
            }
            RuntimeValue::UnsignedInteger8(runtime_val) => {
                Ok(UnsignedInteger32Value::create(runtime_val.value as u32))
            }
            RuntimeValue::UnsignedInteger16(runtime_val) => {
                Ok(UnsignedInteger32Value::create(runtime_val.value as u32))
            }
            RuntimeValue::UnsignedInteger32(runtime_val) => {
                helpers::cast_to_u32(runtime_val.value).map(UnsignedInteger32Value::create)
            }
            RuntimeValue::UnsignedInteger64(runtime_val) => {
                helpers::cast_to_u32(runtime_val.value).map(UnsignedInteger32Value::create)
            }
            RuntimeValue::UnsignedInteger128(runtime_val) => {
                helpers::cast_u128_to_u32(runtime_val.value).map(UnsignedInteger32Value::create)
            }
            _ => Err(format!("Cannot cast {:#?} into an u32", self)),
        }
    }

    pub fn as_u64(self) -> Result<UnsignedInteger64Value, String> {
        if !helpers::runtime_value_is_integer(&self) {
            return Err(format!("Cannot cast {:#?} into an u64", self));
        }

        match self.to_owned() {
            RuntimeValue::Integer8(runtime_val) => {
                helpers::cast_to_u64(runtime_val.value).map(UnsignedInteger64Value::create)
            }
            RuntimeValue::Integer16(runtime_val) => {
                helpers::cast_to_u64(runtime_val.value).map(UnsignedInteger64Value::create)
            }
            RuntimeValue::Integer32(runtime_val) => {
                helpers::cast_to_u64(runtime_val.value).map(UnsignedInteger64Value::create)
            }
            RuntimeValue::Integer64(runtime_val) => {
                helpers::cast_to_u64(runtime_val.value).map(UnsignedInteger64Value::create)
            }
            RuntimeValue::Integer128(runtime_val) => {
                helpers::cast_to_u64(runtime_val.value).map(UnsignedInteger64Value::create)
            }
            RuntimeValue::UnsignedInteger8(runtime_val) => {
                Ok(UnsignedInteger64Value::create(runtime_val.value as u64))
            }
            RuntimeValue::UnsignedInteger16(runtime_val) => {
                Ok(UnsignedInteger64Value::create(runtime_val.value as u64))
            }
            RuntimeValue::UnsignedInteger32(runtime_val) => {
                Ok(UnsignedInteger64Value::create(runtime_val.value as u64))
            }
            RuntimeValue::UnsignedInteger64(runtime_val) => Ok(runtime_val),
            RuntimeValue::UnsignedInteger128(runtime_val) => {
                helpers::cast_u128_to_u64(runtime_val.value).map(UnsignedInteger64Value::create)
            }
            _ => Err(format!("Cannot cast {:#?} into an u64", self)),
        }
    }

    pub fn as_u128(self) -> Result<UnsignedInteger128Value, String> {
        if !helpers::runtime_value_is_integer(&self) {
            return Err(format!("Cannot cast {:#?} into an u128", self));
        }

        match self.to_owned() {
            RuntimeValue::Integer8(runtime_val) => {
                helpers::cast_to_u128(runtime_val.value).map(UnsignedInteger128Value::create)
            }
            RuntimeValue::Integer16(runtime_val) => {
                helpers::cast_to_u128(runtime_val.value).map(UnsignedInteger128Value::create)
            }
            RuntimeValue::Integer32(runtime_val) => {
                helpers::cast_to_u128(runtime_val.value).map(UnsignedInteger128Value::create)
            }
            RuntimeValue::Integer64(runtime_val) => {
                helpers::cast_to_u128(runtime_val.value).map(UnsignedInteger128Value::create)
            }
            RuntimeValue::Integer128(runtime_val) => {
                helpers::cast_to_u128(runtime_val.value).map(UnsignedInteger128Value::create)
            }
            RuntimeValue::UnsignedInteger8(runtime_val) => {
                helpers::cast_to_u128(runtime_val.value).map(UnsignedInteger128Value::create)
            }
            RuntimeValue::UnsignedInteger16(runtime_val) => {
                helpers::cast_to_u128(runtime_val.value).map(UnsignedInteger128Value::create)
            }
            RuntimeValue::UnsignedInteger32(runtime_val) => {
                helpers::cast_to_u128(runtime_val.value).map(UnsignedInteger128Value::create)
            }
            RuntimeValue::UnsignedInteger64(runtime_val) => {
                helpers::cast_to_u128(runtime_val.value).map(UnsignedInteger128Value::create)
            }
            RuntimeValue::UnsignedInteger128(runtime_val) => Ok(runtime_val),
            _ => Err(format!("Cannot cast {:#?} into an u128", self)),
        }
    }

    pub fn as_f32(self) -> Result<Float32Value, String> {
        if !helpers::runtime_value_is_digit(&self) {
            return Err(format!("Cannot cast {:#?} into an f32", self));
        }

        match self {
            RuntimeValue::Integer8(runtime_val) => {
                helpers::cast_to_f32(runtime_val.value).map(Float32Value::create)
            }
            RuntimeValue::Integer16(runtime_val) => {
                helpers::cast_to_f32(runtime_val.value).map(Float32Value::create)
            }
            RuntimeValue::Integer32(runtime_val) => {
                helpers::cast_i32_to_f32(runtime_val.value).map(Float32Value::create)
            }
            RuntimeValue::Integer64(runtime_val) => {
                helpers::cast_i64_to_f32(runtime_val.value).map(Float32Value::create)
            }
            RuntimeValue::Integer128(runtime_val) => {
                helpers::cast_i128_to_f32(runtime_val.value).map(Float32Value::create)
            }
            RuntimeValue::UnsignedInteger8(runtime_val) => {
                helpers::cast_to_f32(runtime_val.value).map(Float32Value::create)
            }
            RuntimeValue::UnsignedInteger16(runtime_val) => {
                helpers::cast_to_f32(runtime_val.value).map(Float32Value::create)
            }
            RuntimeValue::UnsignedInteger32(runtime_val) => {
                helpers::cast_u32_to_f32(runtime_val.value).map(Float32Value::create)
            }
            RuntimeValue::UnsignedInteger64(runtime_val) => {
                helpers::cast_u64_to_f32(runtime_val.value).map(Float32Value::create)
            }
            RuntimeValue::UnsignedInteger128(runtime_val) => {
                helpers::cast_u128_to_f32(runtime_val.value).map(Float32Value::create)
            }
            RuntimeValue::Float32(runtime_val) => Ok(runtime_val),
            RuntimeValue::Float64(runtime_val) => {
                Ok(Float32Value::create(runtime_val.value as f32))
            }
            _ => Err(format!("Cannot cast {:#?} into an f64", self)),
        }
    }

    pub fn as_f64(self) -> Result<Float64Value, String> {
        if !helpers::runtime_value_is_digit(&self) {
            return Err(format!("Cannot cast {:#?} into an f64", self));
        }

        match self {
            RuntimeValue::Integer8(runtime_val) => {
                helpers::cast_to_f64(runtime_val.value).map(Float64Value::create)
            }
            RuntimeValue::Integer16(runtime_val) => {
                helpers::cast_to_f64(runtime_val.value).map(Float64Value::create)
            }
            RuntimeValue::Integer32(runtime_val) => {
                helpers::cast_to_f64(runtime_val.value).map(Float64Value::create)
            }
            RuntimeValue::Integer64(runtime_val) => {
                helpers::cast_i64_to_f64(runtime_val.value).map(Float64Value::create)
            }
            RuntimeValue::Integer128(runtime_val) => {
                helpers::cast_i128_to_f64(runtime_val.value).map(Float64Value::create)
            }
            RuntimeValue::UnsignedInteger8(runtime_val) => {
                helpers::cast_to_f64(runtime_val.value).map(Float64Value::create)
            }
            RuntimeValue::UnsignedInteger16(runtime_val) => {
                helpers::cast_to_f64(runtime_val.value).map(Float64Value::create)
            }
            RuntimeValue::UnsignedInteger32(runtime_val) => {
                helpers::cast_to_f64(runtime_val.value).map(Float64Value::create)
            }
            RuntimeValue::UnsignedInteger64(runtime_val) => {
                helpers::cast_u64_to_f64(runtime_val.value).map(Float64Value::create)
            }
            RuntimeValue::UnsignedInteger128(runtime_val) => {
                helpers::cast_u128_to_f64(runtime_val.value).map(Float64Value::create)
            }
            RuntimeValue::Float32(runtime_val) => {
                Ok(Float64Value::create(runtime_val.value as f64))
            }
            RuntimeValue::Float64(runtime_val) => Ok(runtime_val),
            _ => Err(format!("Cannot cast {:#?} into an f64", self)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Integer8Value {
    pub value_type: ValueType,
    pub value: i8,
}

impl Integer8Value {
    pub fn create(value: i8) -> Self {
        Integer8Value {
            value_type: ValueType::Integer8,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::Integer8(self)
    }
}

#[derive(Debug, Clone)]
pub struct Integer16Value {
    pub value_type: ValueType,
    pub value: i16,
}

impl Integer16Value {
    pub fn create(value: i16) -> Self {
        Integer16Value {
            value_type: ValueType::Integer16,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::Integer16(self)
    }
}

#[derive(Debug, Clone)]
pub struct Integer32Value {
    pub value_type: ValueType,
    pub value: i32,
}

impl Integer32Value {
    pub fn create(value: i32) -> Self {
        Integer32Value {
            value_type: ValueType::Integer32,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::Integer32(self)
    }
}

#[derive(Debug, Clone)]
pub struct Integer64Value {
    pub value_type: ValueType,
    pub value: i64,
}

impl Integer64Value {
    pub fn create(value: i64) -> Self {
        Integer64Value {
            value_type: ValueType::Integer64,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::Integer64(self)
    }
}

#[derive(Debug, Clone)]
pub struct Integer128Value {
    pub value_type: ValueType,
    pub value: i128,
}

impl Integer128Value {
    pub fn create(value: i128) -> Self {
        Integer128Value {
            value_type: ValueType::Integer128,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::Integer128(self)
    }
}

#[derive(Debug, Clone)]
pub struct UnsignedInteger8Value {
    pub value_type: ValueType,
    pub value: u8,
}

impl UnsignedInteger8Value {
    pub fn create(value: u8) -> Self {
        UnsignedInteger8Value {
            value_type: ValueType::UnsignedInteger8,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::UnsignedInteger8(self)
    }
}

#[derive(Debug, Clone)]
pub struct UnsignedInteger16Value {
    pub value_type: ValueType,
    pub value: u16,
}

impl UnsignedInteger16Value {
    pub fn create(value: u16) -> Self {
        UnsignedInteger16Value {
            value_type: ValueType::UnsignedInteger16,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::UnsignedInteger16(self)
    }
}

#[derive(Debug, Clone)]
pub struct UnsignedInteger32Value {
    pub value_type: ValueType,
    pub value: u32,
}

impl UnsignedInteger32Value {
    pub fn create(value: u32) -> Self {
        UnsignedInteger32Value {
            value_type: ValueType::UnsignedInteger32,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::UnsignedInteger32(self)
    }
}

#[derive(Debug, Clone)]
pub struct UnsignedInteger64Value {
    pub value_type: ValueType,
    pub value: u64,
}

impl UnsignedInteger64Value {
    pub fn create(value: u64) -> Self {
        UnsignedInteger64Value {
            value_type: ValueType::UnsignedInteger64,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::UnsignedInteger64(self)
    }
}

#[derive(Debug, Clone)]
pub struct UnsignedInteger128Value {
    pub value_type: ValueType,
    pub value: u128,
}

impl UnsignedInteger128Value {
    pub fn create(value: u128) -> Self {
        UnsignedInteger128Value {
            value_type: ValueType::UnsignedInteger128,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::UnsignedInteger128(self)
    }
}

#[derive(Debug, Clone)]
pub struct Float32Value {
    pub value_type: ValueType,
    pub value: f32,
}

impl Float32Value {
    pub fn create(value: f32) -> Self {
        Float32Value {
            value_type: ValueType::Float32,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::Float32(self)
    }
}

#[derive(Debug, Clone)]
pub struct Float64Value {
    pub value_type: ValueType,
    pub value: f64,
}

impl Float64Value {
    pub fn create(value: f64) -> Self {
        Float64Value {
            value_type: ValueType::Float64,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::Float64(self)
    }
}

#[derive(Debug, Clone)]
pub struct StringValue {
    pub value_type: ValueType,
    pub value: String,
}

impl StringValue {
    pub fn create(value: String) -> Self {
        StringValue {
            value_type: ValueType::String,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::String(self)
    }
}

#[derive(Debug, Clone)]
pub struct CharacterValue {
    pub value_type: ValueType,
    pub value: char,
}

impl CharacterValue {
    pub fn create(value: char) -> Self {
        CharacterValue {
            value_type: ValueType::Character,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::Character(self)
    }
}

#[derive(Debug, Clone)]
pub struct BooleanValue {
    pub value_type: ValueType,
    pub value: bool,
}

impl BooleanValue {
    pub fn create(value: bool) -> Self {
        BooleanValue {
            value_type: ValueType::Boolean,
            value,
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::Boolean(self)
    }
}

#[derive(Debug, Clone)]
pub struct NullValue {
    pub value_type: ValueType,
    pub value: String,
}

impl NullValue {
    pub fn create() -> Self {
        NullValue {
            value_type: ValueType::Null,
            value: String::from("null"),
        }
    }

    pub fn as_raw(self) -> RuntimeValue {
        RuntimeValue::Null(self)
    }
}
