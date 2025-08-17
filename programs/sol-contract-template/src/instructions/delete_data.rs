use anchor_lang::prelude::*;
use crate::state::DataAccount;

/// 删除数据账户
/// 
/// 这个函数删除用户的数据账户并将租金返还给用户
/// 使用 Anchor 的 close 约束自动处理账户关闭
/// 
/// # 参数
/// * `ctx` - 指令执行上下文，包含所需的账户
/// 
/// # 返回
/// * `Result<()>` - 成功时返回 Ok(())，失败时返回错误
pub fn delete_data(ctx: Context<DeleteData>) -> Result<()> {
    let data_account = &ctx.accounts.data_account;
    
    // 验证所有权 - 只有数据所有者可以删除
    require!(
        data_account.is_owner(&ctx.accounts.user.key()),
        crate::error::ErrorCode::Unauthorized
    );
    
    let data_length = data_account.data_length();
    let created_at = data_account.created_at;
    
    msg!("数据删除成功!");
    msg!("所有者: {}", data_account.owner);
    msg!("删除的数据长度: {} 字符", data_length);
    msg!("原创建时间: {}", created_at);
    msg!("删除时间: {}", Clock::get()?.unix_timestamp);
    
    // 账户会被自动关闭并将租金返还给用户
    // 这是通过 #[account(close = user)] 约束自动处理的
    
    Ok(())
}

/// 删除数据指令的账户验证结构
/// 
/// 使用 close 约束自动关闭账户并返还租金
#[derive(Accounts)]
pub struct DeleteData<'info> {
    /// 用户账户 - 接收返还的租金
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// 数据账户 - 将被关闭并返还租金给用户
    #[account(
        mut,
        close = user,                           // 关闭账户并将租金返还给 user
        seeds = [b"data", user.key().as_ref()], // 验证 PDA
        bump                                    // 验证 bump seed
    )]
    pub data_account: Account<'info, DataAccount>,
}