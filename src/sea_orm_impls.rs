// Copyright (c) The camino Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! SeaORM implementations for `Utf8PathBuf`.

use sea_orm::{
    entity::prelude::*,
    sea_query::{ArrayType, Nullable, ValueType},
    TryGetable,
};

use crate::Utf8PathBuf;

impl From<Utf8PathBuf> for Value {
    fn from(p: Utf8PathBuf) -> Self {
        Value::String(Some(Box::new(p.into_string())))
    }
}

impl TryGetable for Utf8PathBuf {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        String::try_get_by(res, index).map(Utf8PathBuf::from)
    }
}

impl ValueType for Utf8PathBuf {
    fn try_from(v: Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        <String as ValueType>::try_from(v).map(Utf8PathBuf::from)
    }

    fn type_name() -> String {
        "Utf8PathBuf".to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::None)
    }
}

impl Nullable for Utf8PathBuf {
    fn null() -> Value {
        Value::String(None)
    }
}
