//! 代码生成通用常量

/// 单表（增删改查）    
pub const TPL_CRUD: &str = "crud";
/// 单表行编辑（增删改查）
pub const TPL_CRUD_ROW_EDIT: &str = "crudrowedit";
/// 树表（增删改查）
pub const TPL_TREE: &str = "tree";
/// 主子表左右布局（增删改查）
pub const TPL_SUB: &str = "sub";
/// 主子表嵌套（增删改查）
pub const TPL_SUB_NESTED: &str = "subnested";
/// 树（增删改查）
pub const TPL_TREEGRID: &str = "treegrid";
/// 树编码字段
pub const TREE_CODE: &str = "treeCode";
/// 树父编码字段
pub const TREE_PARENT_CODE: &str = "treeParentCode";
/// 树名称字段
pub const TREE_NAME: &str = "treeName";
/// 上级菜单ID字段
pub const PARENT_MENU_ID: &str = "parentMenuId";
/// 上级菜单名称字段
pub const PARENT_MENU_NAME: &str = "parentMenuName";
/// 是否可上传附件配置(1：是；2：否)
pub const ATTACH_OPTION: &str = "attachOption";
/// 是否可上传附件配置(1：是；2：否)
pub const DISABLE_ENABLE_OPTION: &str = "disableEnableOption";
/// 是否保存并继续添加
pub const SAVE_AND_ADD_OPTION: &str = "saveAndAddOption";
/// 是否有复制功能
pub const COPY_RECORD_OPTION: &str = "copyRecordOption";
/// 默认上级菜单，系统工具
pub const DEFAULT_PARENT_MENU_ID: &str = "3";
/// 菜单ICON
pub const MENU_ICON: &str = "menuIcon";
/// 数据库字符串类型
pub const COLUMNTYPE_STR: [&str; 4] = ["char", "varchar", "nvarchar", "varchar2"];
/// 数据库文本类型
pub const COLUMNTYPE_TEXT: [&str; 4] = ["tinytext", "text", "mediumtext", "longtext"];
/// 数据库时间类型
pub const COLUMNTYPE_TIME: [&str; 4] = ["datetime", "time", "date", "timestamp"];
/// 数据库数字类型
pub const COLUMNTYPE_NUMBER: [&str; 11] = [
    "tinyint",
    "smallint",
    "mediumint",
    "int",
    "number",
    "integer",
    "bit",
    "bigint",
    "float",
    "double",
    "decimal",
];
/// 页面不需要编辑字段
pub const COLUMNNAME_NOT_EDIT: [&str; 10] = [
    "id",
    "create_by",
    "create_dept",
    "create_time",
    "update_by",
    "update_time",
    "update_ip",
    "remark",
    "version",
    "del_flag",
];
/// 表中必须包含的常用字段
pub const COLUMNNAME_REQUIRE_COLUMN: [&str; 10] = [
    "remark",
    "id",
    "create_by",
    "create_dept",
    "create_time",
    "update_by",
    "update_time",
    "update_ip",
    "version",
    "del_flag",
];
/// 页面常用字段
pub const COLUMNNAME_COMMON_COLUMN: [&str; 18] = [
    "tree_sort",
    "sort",
    "status",
    "remark",
    "id",
    "parent_id",
    "parent_ids",
    "tree_sorts",
    "tree_level",
    "tree_leaf",
    "create_by",
    "create_dept",
    "create_time",
    "update_by",
    "update_time",
    "update_ip",
    "version",
    "del_flag",
];
/// 页面不需要显示的列表字段
pub const COLUMNNAME_NOT_LIST: [&str; 15] = [
    "aprent_id",
    "parent_ids",
    "tree_sorts",
    "tree_level",
    "tree_leaf",
    "id",
    "create_by",
    "create_dept",
    "create_time",
    "update_by",
    "update_time",
    "update_ip",
    "remark",
    "version",
    "del_flag",
];
/// 页面不需要查询字段
pub const COLUMNNAME_NOT_QUERY: [&str; 13] = [
    "parentId",
    "parentIds",
    "treeSort",
    "treeSorts",
    "treeLevel",
    "treeLeaf",
    "id",
    "create_by",
    "create_time",
    "del_flag",
    "update_by",
    "update_time",
    "remark",
];
/// Entity基类字段
pub const BASE_ENTITY: [&str; 10] = [
    "id",
    "createBy",
    "createDept",
    "createTime",
    "updateBy",
    "updateTime",
    "updateIp",
    "remark",
    "version",
    "delFlag",
];
/// Tree基类字段
pub const TREE_ENTITY: [&str; 7] = [
    "parentId",
    "parentIds",
    "treeSort",
    "treeSorts",
    "treeLevel",
    "treeLeaf",
    "children",
];
/// 文本框
pub const HTML_INPUT: &str = "input";
/// 数字框
pub const HTML_NUMBER: &str = "number";
/// 文本域
pub const HTML_TEXTAREA: &str = "textarea";
/// 下拉框
pub const HTML_SELECT: &str = "select";
/// 下拉框
pub const HTML_SELECT_MULTIPLE: &str = "selectMultiple";
/// 开关
pub const HTML_SWITCH: &str = "switch";
/// 单选框
pub const HTML_RADIO: &str = "radio";
/// 单选框
pub const HTML_RADIO_BUTTON: &str = "radioButton";
/// 复选框
pub const HTML_CHECKBOX: &str = "checkbox";
/// 日期控件
pub const HTML_DATETIME: &str = "datetime";
/// 时间戳控件
pub const HTML_DATE: &str = "date";
/// 图片上传控件
pub const HTML_IMAGE_UPLOAD: &str = "imageUpload";
/// 文件上传控件
pub const HTML_FILE_UPLOAD: &str = "fileUpload";
/// 富文本控件
pub const HTML_EDITOR: &str = "editor";
/// 字符串类型
pub const TYPE_STRING: &str = "String";
/// 整型
pub const TYPE_INTEGER: &str = "Integer";
/// 长整型
pub const TYPE_LONG: &str = "Long";
/// 浮点型
pub const TYPE_DOUBLE: &str = "Double";
/// 高精度计算类型
pub const TYPE_BIGDECIMAL: &str = "BigDecimal";
/// 时间类型
pub const TYPE_DATE: &str = "Date";
/// 模糊查询
pub const QUERY_LIKE: &str = "LIKE";
/// 精确查询
pub const QUERY_EQ: &str = "EQ";
/// 区间查询
pub const QUERY_BETWEEN: &str = "BETWEEN";
/// 需要
pub const REQUIRE: &str = "1";
/// 居中
pub const ALIGH_CENTER: &str = "center";
/// 居右
pub const ALIGH_RIGHT: &str = "right";
/// 居左
pub const ALIGH_LEFT: &str = "left";
