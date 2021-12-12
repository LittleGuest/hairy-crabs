/// struct 模板
pub(crate) const MODEL_TEMPLATE: &str = r#"
use rbatis::{crud_table, Timestamp};
use serde::{Deserialize, Serialize};

/// {{table.table_comment}}
#[crud_table]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct {{ struct_name }} { {% if has_columns %}{% for column in columns %}
    /// {{column.column_comment}}
    pub {{column.column_name}}: {% if column.is_nullable == "NO" %}{{column.field_type}},{% else %}Option<{{column.field_type}}>,{% endif %}{% endfor %}{% endif %}
}
"#;

/// mod.rs 文件模板
pub(crate) const MOD_TEMPLATE: &str = r#"
{% for table_name, _ in table_names %}
mod {{table_name}};
pub use {{table_name}}::*;
{% endfor %}
"#;
