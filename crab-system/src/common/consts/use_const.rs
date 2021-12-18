/// 平台内系统用户的唯一标志    
pub const SYS_USER: &str = "SYS_USER";
/// 正常状态
pub const NORMAL: &str = "0";
/// 异常状态
pub const EXCEPTION: &str = "1";
/// 用户封禁状态
pub const USER_DISABLE: &str = "1";
/// 角色封禁状态
pub const ROLE_DISABLE: &str = "1";
/// 部门正常状态
pub const DEPT_NORMAL: &str = "0";
/// 部门停用状态
pub const DEPT_DISABLE: &str = "1";
/// 字典正常状态
pub const DICT_NORMAL: &str = "0";
/// 是否为系统默认（是）
pub const YES: &str = "Y";
/// 是否菜单外链（是）
pub const YES_FRAME: &str = "0";
/// 是否菜单外链（否）
pub const NO_FRAME: &str = "1";
/// 菜单类型（目录）
pub const TYPE_DIR: &str = "M";
/// 菜单类型（菜单）
pub const TYPE_MENU: &str = "C";
/// 菜单类型（按钮）
pub const TYPE_BUTTON: &str = "F";
/// Layout组件标识
pub const LAYOUT: &str = "Layout";
/// ParentView组件标识
pub const PARENT_VIEW: &str = "ParentView";
/// 校验返回结果码
pub const UNIQUE: &str = "0";
pub const NOT_UNIQUE: &str = "1";
/// 用户名长度限制
pub const USERNAME_MIN_LENGTH: u32 = 2;
pub const USERNAME_MAX_LENGTH: u32 = 20;
/// 密码长度限制
pub const PASSWORD_MIN_LENGTH: u32 = 5;
pub const PASSWORD_MAX_LENGTH: u32 = 20;
