use anchor_lang::prelude::*;

/// 初始化程序指令
/// 
/// 这是一个简单的初始化函数，通常用于程序的第一次部署
/// 在实际应用中，可能会初始化全局状态或执行其他设置操作
pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
    msg!("Solana 智能合约模板已成功初始化!");
    msg!("程序 ID: {}", &_ctx.program_id);
    msg!("当前时间: {}", Clock::get()?.unix_timestamp);
    
    Ok(())
}

/// 初始化指令的账户验证结构
/// 
/// 由于这是一个简单的初始化函数，不需要特殊的账户
/// 在更复杂的场景中，可能需要验证管理员账户或初始化全局状态账户
#[derive(Accounts)]
pub struct Initialize {}
