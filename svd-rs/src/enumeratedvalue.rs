use super::{BuildError, Description, EmptyToNone, Name, SvdError, ValidateLevel};

/// Describes a single entry in the enumeration.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct EnumeratedValue {
    /// String describing the semantics of the value. Can be displayed instead of the value
    pub name: String,

    /// Extended string describing the value
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<String>,

    /// Defines the constant for the bit-field as decimal, hexadecimal or binary number
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub value: Option<u64>,

    /// Defines the name and description for all other values that are not listed explicitly
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub is_default: Option<bool>,
}

/// Errors for [`EnumeratedValue::validate`]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// No value was specified
    #[error("EnumeratedValue has no `value` or `is_default`")]
    AbsentValue,
    /// Passed both value and isDefault, only checked in strict mode
    #[error("EnumeratedValue with `value` (passed {0:?}) should not have `is_default(True)`")]
    ValueAndDefault(Option<u64>),
    /// The value is not in range.
    #[error("Value {0} out of range [{} - {}]", .1.start, .1.end - 1)]
    OutOfRange(u64, core::ops::Range<u64>),
}

/// Builder for [`EnumeratedValue`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EnumeratedValueBuilder {
    name: Option<String>,
    description: Option<String>,
    value: Option<u64>,
    is_default: Option<bool>,
}

impl From<EnumeratedValue> for EnumeratedValueBuilder {
    fn from(e: EnumeratedValue) -> Self {
        Self {
            name: Some(e.name),
            description: e.description,
            value: e.value,
            is_default: e.is_default,
        }
    }
}

impl EnumeratedValueBuilder {
    /// Set the name of the enumerated value.
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// Set the description of the enumerated value.
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Set the value of the enumerated value.
    pub fn value(mut self, value: Option<u64>) -> Self {
        self.value = value;
        self
    }
    #[allow(clippy::wrong_self_convention)]
    /// Set if the enumerated value is defaulted for non-explicit values.
    pub fn is_default(mut self, value: Option<bool>) -> Self {
        self.is_default = value;
        self
    }
    /// Validate and build a [`EnumeratedValue`].
    pub fn build(self, lvl: ValidateLevel) -> Result<EnumeratedValue, SvdError> {
        let ev = EnumeratedValue {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            description: self.description.empty_to_none(),
            value: self.value,
            is_default: self.is_default,
        };
        ev.validate(lvl)?;
        Ok(ev)
    }
}

impl EnumeratedValue {
    /// Enumerated value is defaulted for non-explicit values
    pub fn is_default(&self) -> bool {
        matches!(self.is_default, Some(true))
    }
    /// Make a builder for [`EnumeratedValue`]
    pub fn builder() -> EnumeratedValueBuilder {
        EnumeratedValueBuilder::default()
    }
    /// Modify an existing [`EnumeratedValue`] based on a [builder](EnumeratedValueBuilder).
    pub fn modify_from(
        &mut self,
        builder: EnumeratedValueBuilder,
        lvl: ValidateLevel,
    ) -> Result<(), SvdError> {
        if let Some(name) = builder.name {
            self.name = name;
        }
        if builder.description.is_some() {
            self.description = builder.description.empty_to_none();
        }
        if builder.value.is_some() {
            self.value = builder.value;
        }
        if builder.is_default.is_some() {
            self.is_default = builder.is_default;
        }
        self.validate(lvl)
    }
    /// Validate the [`EnumeratedValue`].
    pub fn validate(&self, lvl: ValidateLevel) -> Result<(), SvdError> {
        if !lvl.is_disabled() {
            if lvl.is_strict() {
                super::check_name(&self.name, "name")?;
            }
            match (self.value.is_some(), self.is_default()) {
                (false, false) => Err(Error::AbsentValue.into()),
                (true, true) if lvl.is_strict() => Err(Error::ValueAndDefault(self.value).into()),
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }
    pub(crate) fn check_range(&self, range: &core::ops::Range<u64>) -> Result<(), SvdError> {
        Ok(())
    }
}

impl Name for EnumeratedValue {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Description for EnumeratedValue {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
