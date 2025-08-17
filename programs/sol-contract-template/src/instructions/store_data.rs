use anchor_lang::prelude::*;
use crate::state::DataAccount;

/// 存储数据到链上
/// 
/// 这个函数为用户创建一个新的数据账户并存储数据
/// 使用 PDA (程序派生地址) 确保每个用户只能有一个数据账户
/// 
/// # 参数
/// * `ctx` - 指令执行上下文，包含所需的账户
/// * `data` - 要存储的数据字符串
/// 
/// # 返回
/// * `Result<()>` - 成功时返回 Ok(())，失败时返回错误
pub fn store_data(ctx: Context<StoreData>, data: String) -> Result<()> {
    // 验证数据长度
    require!(data.len() <= 1000, crate::error::ErrorCode::DataTooLong);
    require!(!data.is_empty(), crate::error::ErrorCode::DataEmpty);
    
    let data_account = &mut ctx.accounts.data_account;
    let current_time = Clock::get()?.unix_timestamp;
    
    // 初始化数据账户
    data_account.owner = ctx.accounts.user.key();
    data_account.data = data.clone();
    data_account.created_at = current_time;
    data_account.updated_at = 0; // 初始化时设为0
    
    msg!("数据存储成功!");
    msg!("所有者: {}", data_account.owner);
    msg!("数据长度: {} 字符", data.len());
    msg!("创建时间: {}", current_time);
    
    Ok(())
}

/// 存储数据指令的账户验证结构
/// 
/// 这个结构定义了存储数据时需要的所有账户和约束
#[derive(Accounts)]
pub struct StoreData<'info> {
    /// 用户账户 - 必须签名并支付账户租金
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// 数据账户 - 使用 PDA 确保唯一性
    #[account(
        init,                                    // 初始化新账户
        payer = user,                           // 用户支付账户租金
        space = 8 + DataAccount::INIT_SPACE,    // 账户空间大小
        seeds = [b"data", user.key().as_ref()], // PDA 种子
        bump                                    // PDA bump seed
    )]
    pub data_account: Account<'info, DataAccount>,
    
    /// 系统程序 - 用于创建新账户
    pub system_program: Program<'info, System>,
}