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
