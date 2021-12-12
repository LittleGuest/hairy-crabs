//! 任务调度通用常量

pub const TASK_CLASS_NAME: &str = "TASK_CLASS_NAME";
/// 执行目标key
pub const TASK_PROPERTIES: &str = "TASK_PROPERTIES";
/// 默认
pub const MISFIRE_DEFAULT: &str = "0";
/// 立即触发执行
pub const MISFIRE_IGNORE_MISFIRES: &str = "1";
/// 触发一次执行
pub const MISFIRE_FIRE_AND_PROCEED: &str = "2";
/// 不触发立即执行
pub const MISFIRE_DO_NOTHING: &str = "3";
/// 状态
pub enum Status {
    /**
     * 正常
     */
    NORMAL = 0,
    /**
     * 暂停
     */
    PAUSE = 1,
}
