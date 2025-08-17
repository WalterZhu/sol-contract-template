use anchor_lang::prelude::*;

/// 智能合约错误代码定义
/// 
/// 这个枚举定义了智能合约可能抛出的所有自定义错误
/// 每个错误都有描述性的消息，便于调试和用户理解
#[error_code]
pub enum ErrorCode {
    /// 用户无权访问此资源
    #[msg("无权访问此数据")]
    Unauthorized,
    
    /// 提供的数据为空
    #[msg("数据不能为空")]
    DataEmpty,
    
    /// 数据长度超过限制
    #[msg("数据长度超过1000字符限制")]
    DataTooLong,
    
    /// 账户已存在
    #[msg("数据账户已存在")]
    AccountAlreadyExists,
    
    /// 账户不存在
    #[msg("数据账户不存在")]
    AccountNotFound,
    
    /// 数学运算溢出
    #[msg("数学运算溢出")]
    MathOverflow,
    
    /// 无效的时间戳
    #[msg("无效的时间戳")]
    InvalidTimestamp,
}

impl ErrorCode {
    /// 获取错误代码的数值
    /// 
    /// # 返回
    /// * `u32` - 错误代码对应的数值
    pub fn error_code(&self) -> u32 {
        (*self).into()
    }
    
    /// 获取错误消息
    /// 
    /// # 返回
    /// * `&str` - 错误描述消息
    pub fn error_msg(&self) -> &str {
        match self {
            ErrorCode::Unauthorized => "无权访问此数据",
            ErrorCode::DataEmpty => "数据不能为空",
            ErrorCode::DataTooLong => "数据长度超过1000字符限制",
            ErrorCode::AccountAlreadyExists => "数据账户已存在",
            ErrorCode::AccountNotFound => "数据账户不存在",
            ErrorCode::MathOverflow => "数学运算溢出",
            ErrorCode::InvalidTimestamp => "无效的时间戳",
        }
    }
}
