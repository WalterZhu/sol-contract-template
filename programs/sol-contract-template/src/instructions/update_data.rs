use anchor_lang::prelude::*;
use crate::state::DataAccount;

/// 更新已存储的数据
/// 
/// 这个函数允许数据所有者更新他们已存储的数据
/// 包含所有权验证确保只有所有者才能更新数据
/// 
/// # 参数
/// * `ctx` - 指令执行上下文，包含所需的账户
/// * `new_data` - 新的数据字符串
/// 
/// # 返回
/// * `Result<()>` - 成功时返回 Ok(())，失败时返回错误
pub fn update_data(ctx: Context<UpdateData>, new_data: String) -> Result<()> {
    // 验证数据长度
    require!(new_data.len() <= 1000, crate::error::ErrorCode::DataTooLong);
    require!(!new_data.is_empty(), crate::error::ErrorCode::DataEmpty);
    
    let data_account = &mut ctx.accounts.data_account;
    
    // 验证所有权 - 只有数据所有者可以更新
    require!(
        data_account.is_owner(&ctx.accounts.user.key()),
        crate::error::ErrorCode::Unauthorized
    );
    
    let current_time = Clock::get()?.unix_timestamp;
    let old_data_length = data_account.data_length();
    
    // 更新数据
    data_account.data = new_data.clone();
    data_account.updated_at = current_time;
    
    msg!("数据更新成功!");
    msg!("所有者: {}", data_account.owner);
    msg!("旧数据长度: {} 字符", old_data_length);
    msg!("新数据长度: {} 字符", new_data.len());
    msg!("更新时间: {}", current_time);
    
    Ok(())
}

/// 更新数据指令的账户验证结构
/// 
/// 这个结构定义了更新数据时需要的账户
/// 注意这里不需要 init 约束，因为账户已经存在
#[derive(Accounts)]
pub struct UpdateData<'info> {
    /// 用户账户 - 必须是数据的所有者
    pub user: Signer<'info>,
    
    /// 数据账户 - 必须是可变的才能更新
    #[account(mut)]
    pub data_account: Account<'info, DataAccount>,
}