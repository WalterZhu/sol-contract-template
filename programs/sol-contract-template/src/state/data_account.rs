use anchor_lang::prelude::*;

/// 链上数据存储账户
/// 
/// 这个结构体定义了存储在区块链上的数据格式
/// 每个用户都有一个唯一的数据账户
#[account]
#[derive(InitSpace)]
pub struct DataAccount {
    /// 数据所有者的公钥 (32 字节)
    pub owner: Pubkey,
    
    /// 存储的数据内容
    /// 使用 max_len 约束确保账户大小可预测
    #[max_len(1000)]
    pub data: String,
    
    /// 数据创建时间戳 (Unix 时间戳)
    pub created_at: i64,
    
    /// 数据最后更新时间戳 (Unix 时间戳)
    /// 初始值为 0，表示从未更新
    pub updated_at: i64,
}

impl DataAccount {
    /// 验证数据所有者
    /// 
    /// # 参数
    /// * `user_key` - 要验证的用户公钥
    /// 
    /// # 返回
    /// * `bool` - 如果用户是所有者返回 true
    pub fn is_owner(&self, user_key: &Pubkey) -> bool {
        self.owner == *user_key
    }
    
    /// 获取数据长度
    /// 
    /// # 返回
    /// * `usize` - 数据字符串的长度
    pub fn data_length(&self) -> usize {
        self.data.len()
    }
    
    /// 检查数据是否为空
    /// 
    /// # 返回
    /// * `bool` - 如果数据为空返回 true
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}