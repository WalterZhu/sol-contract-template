use anchor_lang::prelude::*;

// 声明程序 ID
declare_id!("3jRnZM4jzBjoAdU4HCdJHcdsAYshMzXi3Fgo6REJw6AW");

// 导入模块
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

// 重新导出常用类型，方便外部使用
pub use error::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;

/// Solana 智能合约模板主程序
/// 
/// 这是一个模块化的智能合约示例，展示了如何：
/// - 组织代码结构
/// - 实现数据存储功能
/// - 处理权限验证
/// - 管理错误处理
/// 
/// 功能包括：
/// - 初始化程序
/// - 存储数据到链上
/// - 更新已存储的数据
/// - 删除数据并回收租金
#[program]
pub mod sol_contract_template {
    use super::*;

    /// 初始化程序
    /// 
    /// 这是程序的入口点，通常在第一次部署时调用
    /// 
    /// # 参数
    /// * `ctx` - 指令执行上下文
    /// 
    /// # 返回
    /// * `Result<()>` - 成功时返回 Ok(())
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    /// 存储数据到链上
    /// 
    /// 为用户创建一个新的数据账户并存储提供的数据
    /// 每个用户只能拥有一个数据账户
    /// 
    /// # 参数
    /// * `ctx` - 指令执行上下文，包含所需的账户
    /// * `data` - 要存储的数据字符串（最大1000字符）
    /// 
    /// # 返回
    /// * `Result<()>` - 成功时返回 Ok(())，失败时返回相应错误
    /// 
    /// # 错误
    /// * `ErrorCode::DataEmpty` - 如果提供的数据为空
    /// * `ErrorCode::DataTooLong` - 如果数据长度超过1000字符
    pub fn store_data(ctx: Context<StoreData>, data: String) -> Result<()> {
        instructions::store_data(ctx, data)
    }

    /// 更新已存储的数据
    /// 
    /// 允许数据所有者更新他们已存储的数据
    /// 只有数据的所有者才能执行此操作
    /// 
    /// # 参数
    /// * `ctx` - 指令执行上下文，包含数据账户
    /// * `new_data` - 新的数据字符串（最大1000字符）
    /// 
    /// # 返回
    /// * `Result<()>` - 成功时返回 Ok(())，失败时返回相应错误
    /// 
    /// # 错误
    /// * `ErrorCode::Unauthorized` - 如果调用者不是数据所有者
    /// * `ErrorCode::DataEmpty` - 如果提供的数据为空
    /// * `ErrorCode::DataTooLong` - 如果数据长度超过1000字符
    pub fn update_data(ctx: Context<UpdateData>, new_data: String) -> Result<()> {
        instructions::update_data(ctx, new_data)
    }

    /// 删除数据账户
    /// 
    /// 删除用户的数据账户并将账户租金返还给用户
    /// 只有数据的所有者才能执行此操作
    /// 
    /// # 参数
    /// * `ctx` - 指令执行上下文，包含要删除的数据账户
    /// 
    /// # 返回
    /// * `Result<()>` - 成功时返回 Ok(())，失败时返回相应错误
    /// 
    /// # 错误
    /// * `ErrorCode::Unauthorized` - 如果调用者不是数据所有者
    pub fn delete_data(ctx: Context<DeleteData>) -> Result<()> {
        instructions::delete_data(ctx)
    }
}
