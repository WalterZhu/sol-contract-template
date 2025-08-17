use anchor_lang::prelude::*;

/// 工具函数模块
/// 
/// 这个模块包含了智能合约中常用的辅助函数
/// 这些函数可以在多个指令中重复使用

/// 常量定义
pub const MAX_DATA_LENGTH: usize = 1000;
pub const DATA_SEED: &[u8] = b"data";

/// 验证数据长度是否有效
/// 
/// # 参数
/// * `data` - 要验证的数据字符串
/// 
/// # 返回
/// * `Result<()>` - 验证通过返回 Ok(())，否则返回错误
pub fn validate_data_length(data: &str) -> Result<()> {
    require!(!data.is_empty(), crate::error::ErrorCode::DataEmpty);
    require!(
        data.len() <= MAX_DATA_LENGTH,
        crate::error::ErrorCode::DataTooLong
    );
    Ok(())
}

/// 获取当前时间戳
/// 
/// # 返回
/// * `Result<i64>` - 当前的 Unix 时间戳
pub fn get_current_timestamp() -> Result<i64> {
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp;
    
    require!(timestamp > 0, crate::error::ErrorCode::InvalidTimestamp);
    
    Ok(timestamp)
}

/// 计算数据账户的 PDA 地址
/// 
/// # 参数
/// * `user_key` - 用户的公钥
/// * `program_id` - 程序 ID
/// 
/// # 返回
/// * `(Pubkey, u8)` - PDA 地址和 bump seed
pub fn find_data_account_pda(user_key: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[DATA_SEED, user_key.as_ref()],
        program_id
    )
}

/// 验证 PDA 地址是否正确
/// 
/// # 参数
/// * `expected_pda` - 期望的 PDA 地址
/// * `user_key` - 用户公钥
/// * `program_id` - 程序 ID
/// 
/// # 返回
/// * `Result<()>` - 验证通过返回 Ok(())，否则返回错误
pub fn validate_pda(
    expected_pda: &Pubkey,
    user_key: &Pubkey,
    program_id: &Pubkey
) -> Result<()> {
    let (actual_pda, _) = find_data_account_pda(user_key, program_id);
    require!(
        *expected_pda == actual_pda,
        crate::error::ErrorCode::Unauthorized
    );
    Ok(())
}

/// 格式化日志消息
/// 
/// # 参数
/// * `operation` - 操作类型
/// * `user_key` - 用户公钥
/// * `data_length` - 数据长度
/// * `timestamp` - 时间戳
pub fn log_operation(
    operation: &str,
    user_key: &Pubkey,
    data_length: usize,
    timestamp: i64
) {
    msg!("=== {} ===", operation);
    msg!("用户: {}", user_key);
    msg!("数据长度: {} 字符", data_length);
    msg!("时间戳: {}", timestamp);
    msg!("================");
}

/// 安全的字符串截断
/// 
/// # 参数
/// * `data` - 原始数据
/// * `max_length` - 最大长度
/// 
/// # 返回
/// * `String` - 截断后的字符串
pub fn safe_truncate(data: &str, max_length: usize) -> String {
    if data.len() <= max_length {
        data.to_string()
    } else {
        format!("{}...", &data[..max_length.saturating_sub(3)])
    }
}

/// 数据统计结构
pub struct DataStats {
    pub length: usize,
    pub word_count: usize,
    pub line_count: usize,
}

/// 计算数据统计信息
/// 
/// # 参数
/// * `data` - 要分析的数据
/// 
/// # 返回
/// * `DataStats` - 数据统计信息
pub fn calculate_data_stats(data: &str) -> DataStats {
    let length = data.len();
    let word_count = data.split_whitespace().count();
    let line_count = data.lines().count();
    
    DataStats {
        length,
        word_count,
        line_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_data_length() {
        // 测试有效数据
        assert!(validate_data_length("valid data").is_ok());
        
        // 测试空数据
        assert!(validate_data_length("").is_err());
        
        // 测试过长数据
        let long_data = "x".repeat(MAX_DATA_LENGTH + 1);
        assert!(validate_data_length(&long_data).is_err());
    }

    #[test]
    fn test_safe_truncate() {
        assert_eq!(safe_truncate("hello", 10), "hello");
        assert_eq!(safe_truncate("hello world", 5), "he...");
    }

    #[test]
    fn test_calculate_data_stats() {
        let stats = calculate_data_stats("hello world\nthis is test");
        assert_eq!(stats.length, 22);
        assert_eq!(stats.word_count, 4);
        assert_eq!(stats.line_count, 2);
    }
}