//!

use darling::FromField;
use syn::{
    parse::{Parse, ParseStream},
    LitInt, LitStr, Token,
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

/// TODO
pub struct ExcelColumn {
    /// 导出时在excel中排序，默认为u8的最大值
    pub sort: u8,
    /// 导出到Excel中的名字
    pub name: String,
    /// 日期格式
    pub date_format: String,
    /// 如果是字典类型，请设置字典的type值（如：sys_user_sex）
    pub dict_type: String,
    /// 读取内容转表达式（如：0=男,1=女,2=未知）
    pub read_converter_expr: String,
    /// 分隔符，读取字符串组内容
    pub separator: String,
    /// BigDecimal 精度 默认：-1（默认不开启BigDecimal格式化）
    pub scale: i32,
    /// BigDecimal 舍入规则 默认：BigDecimal.ROUND_HALF_EVEN
    pub round_mode: u8,
    /// 导出类型（0数字，1字符串（默认），2图片）
    pub cell_type: u8,
    /// 导出时在excel中每个列的高度，单位为字符，默认14
    pub height: u8,
    /// 导出时在excel中每个列的宽度，单位为字符，默认16
    pub width: u8,
    /// 文字后缀，如% 90 变成90%
    pub suffix: String,
    /// 值为空时，字段的默认值
    pub default_value: String,
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

/// 定义 excel_col 中的关键字
mod kw {
    use syn::custom_keyword;

    custom_keyword!(name);
    custom_keyword!(order);
    custom_keyword!(width);
    custom_keyword!(target_type);
    custom_keyword!(date_format);
}

/// 定义 excel_col 的属性
#[derive(Default, FromField)]
pub struct ExcelCol {
    /// 列名
    #[darling(default)]
    pub name: Option<LitStr>,
    /// 列的顺序，数字越大，越靠右
    /// 默认为0,则按照定义的字段从上到下依次排列
    #[darling(default)]
    pub order: Option<LitInt>,
    /// 列的宽度
    #[darling(default)]
    pub width: Option<LitInt>,
    /// 导出的类型
    #[darling(default)]
    pub target_type: Option<LitStr>,
    /// 日期格式
    /// 当 type 为 Date 时生效
    #[darling(default)]
    pub date_format: Option<LitStr>,
}

impl Parse for ExcelCol {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        resolve(input)
    }
}

/// 解析 excel_col 关键字
fn resolve(attrs: ParseStream) -> syn::Result<ExcelCol> {
    let mut excel_col = ExcelCol::default();
    loop {
        // let lookahead = attrs.lookahead1();
        // if lookahead.peek(kw::name) {
        //     attrs.parse::<kw::name>()?;
        //     attrs.parse::<Token![=]>()?;
        //     name = Some(attrs.parse::<LitStr>()?);
        // } else if lookahead.peek(kw::order) {
        //     attrs.parse::<kw::order>()?;
        //     attrs.parse::<Token![=]>()?;
        //     order = Some(attrs.parse::<LitInt>()?);
        // } else {
        //     return Err(attrs.error("invalid argument"));
        // }
        // if let Err(_) = attrs.parse::<Token![,]>() {
        //     break;
        // }

        if attrs.peek(kw::name) {
            attrs.parse::<kw::name>()?;
            attrs.parse::<Token![=]>()?;
            excel_col.name = Some(attrs.parse::<LitStr>()?);
        } else if attrs.peek(kw::order) {
            attrs.parse::<kw::order>()?;
            attrs.parse::<Token![=]>()?;
            excel_col.order = Some(attrs.parse::<LitInt>()?);
        } else if attrs.peek(kw::width) {
            attrs.parse::<kw::width>()?;
            attrs.parse::<Token![=]>()?;
            excel_col.width = Some(attrs.parse::<LitInt>()?);
        } else if attrs.peek(kw::target_type) {
            attrs.parse::<kw::target_type>()?;
            attrs.parse::<Token![=]>()?;
            excel_col.target_type = Some(attrs.parse::<LitStr>()?);
        } else if attrs.peek(kw::date_format) {
            attrs.parse::<kw::date_format>()?;
            attrs.parse::<Token![=]>()?;
            excel_col.date_format = Some(attrs.parse::<LitStr>()?);
        } else {
            return Err(attrs.error("无效的参数"));
        }

        if let Err(_) = attrs.parse::<Token![,]>() {
            break;
        }
    }

    Ok(excel_col)
}
