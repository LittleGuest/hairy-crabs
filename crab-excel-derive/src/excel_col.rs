use darling::{ast, util, FromDeriveInput, FromField, FromMeta};
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitInt, LitStr, Token, Type,
};

/// 导出的类型
pub enum TargetType {
    /// 文本格式，默认值
    String,
    ///  数字格式
    Num,
    /// 函数格式
    Fn,
    /// 图片
    Img,
    /// 日期格式
    Date,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(excel_col))]
pub struct ExcelWriterMacro {
    pub ident: Ident,
    pub data: ast::Data<util::Ignored, ExcelCol>,
}

/*
pub struct ExcelColumn {
    /// 如果是字典类型，请设置字典的type值（如：sys_user_sex）
    pub dict_type: String,
    /// 读取内容转表达式（如：0=男,1=女,2=未知）
    pub read_converter_expr: String,
    /// BigDecimal 精度 默认：-1（默认不开启BigDecimal格式化）
    pub scale: i32,
    /// BigDecimal 舍入规则 默认：BigDecimal.ROUND_HALF_EVEN
    pub round_mode: u8,
    /// 提示信息
    pub prompt: String,
    /// 设置只能选择不能输入的列的内容
    pub combo: Vec<String>,
    /// 是否导出数据，应对需求：只需要导出一份模板，只需标题，但不需要内容，默认为true
    #[deprecated]
    pub is_export: bool,
    /// 另一个类中的属性名称，支持多级获取，以小数点隔开
    #[deprecated]
    pub target_attr: String,
    /// 是否自动统计数据，在最后追加一行统计数据总和，默认为false
    #[deprecated]
    pub is_statistics: bool,
    /// 导出字段对齐方式：0：默认，1靠左，2居中，3，靠右
    pub align: u8,
    /// 自定义数据处理器
    // pub handler: impl ExcelHandlerAdapter,
    /// 自定义数据处理器参数
    pub args: Vec<String>,
    /// 字段类型：0-导出导出，1-仅导出，2-仅导入
    pub r#type: u8,
}
*/
///
/// 定义 excel_col 的属性
#[derive(Debug, FromField)]
#[darling(attributes(excel_col))]
pub struct ExcelCol {
    pub ident: Option<Ident>,
    pub ty: Type,

    /// 导出到Excel中的名字
    #[darling(default)]
    pub name: Option<String>,

    /// 列的顺序，数字越大，越靠右
    /// 默认为0,则按照定义的字段从上到下依次排列
    /// 导出时在excel中排序，默认为u16的最大值
    #[darling(default)]
    pub sort: Option<u16>,

    /// 导出时在excel中每个列的宽度，单位为字符，默认16
    #[darling(default)]
    pub width: Option<u8>,

    /// 导出时在excel中每个列的高度，单位为字符，默认14
    #[darling(default)]
    pub height: Option<u8>,

    /// 导出类型（0数字，1字符串（默认），2图片）
    #[darling(default)]
    pub cell_type: Option<String>,

    /// 日期格式
    /// 当 type 为 Date 时生效
    #[darling(default)]
    pub date_format: Option<String>,

    /// 值为空时，字段的默认值
    #[darling(default)]
    pub default_value: Option<String>,

    /// 文字后缀，如% 90 变成90%
    #[darling(default)]
    pub suffix: Option<String>,

    /// 分隔符，读取字符串组内容
    #[darling(default)]
    pub separator: Option<String>,
}
