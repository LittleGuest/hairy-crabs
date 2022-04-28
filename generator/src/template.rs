pub const MODEL_TEMPLATE: &str = r#"
use rbatis::{crud_table, Timestamp};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// {{table.comment}}
#[crud_table]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct {{ struct_name }} { {% if has_columns %}{% for column in columns %}
    /// {{column.comment}}
    {%if column.field_type == "String"%}#[validate(length(max = {{column.max_length}}))]{%endif%}
    pub {{column.name}}: Option<{{column.field_type}}>,{% endfor %}{% endif %}
}
"#;

/// mod.rs 文件模板
pub const MOD_TEMPLATE: &str = r#"
{% for table_name, _ in table_names %}
mod {{table_name}};
pub use {{table_name}}::*;
{% endfor %}
"#;
