# Solana Smart Contract Template

一个模块化的 Solana 智能合约模板项目，使用 Anchor 框架构建。提供了完整的数据存储、更新和删除功能。

## 特性

- 🏗️ **模块化架构**：清晰的代码组织结构
- 💾 **数据管理**：完整的 CRUD 操作支持
- 🔐 **权限控制**：基于所有者的访问控制
- 🛡️ **错误处理**：完善的错误处理机制
- 🧪 **测试覆盖**：TypeScript 测试用例
- 📝 **代码注释**：详细的中文注释

## 目录结构

```
sol-contract-template/
├── Anchor.toml                 # Anchor 项目配置文件
├── Cargo.toml                  # Rust 工作空间配置
├── package.json                # Node.js 依赖配置
├── tsconfig.json               # TypeScript 配置
├── programs/                   # 智能合约源码目录
│   └── sol-contract-template/
│       ├── Cargo.toml          # 合约 Rust 依赖
│       ├── Xargo.toml          # Solana 构建配置
│       └── src/
│           ├── lib.rs          # 主程序入口
│           ├── error.rs        # 错误定义
│           ├── utils.rs        # 工具函数
│           ├── instructions/   # 指令实现
│           │   ├── mod.rs
│           │   ├── initialize.rs
│           │   ├── store_data.rs
│           │   ├── update_data.rs
│           │   └── delete_data.rs
│           └── state/          # 状态定义
│               ├── mod.rs
│               └── data_account.rs
├── tests/                      # 测试文件
│   └── sol-contract-template.ts
├── migrations/                 # 部署脚本
│   └── deploy.ts
└── target/                     # 编译输出目录
```

## 环境要求

确保你的开发环境已安装以下工具：

- **Rust** >= 1.89.0
- **Solana CLI** >= 2.2.0 (推荐使用 Agave)
- **Anchor CLI** >= 0.31.0
- **Node.js** >= 16.0.0
- **Yarn** (推荐) 或 npm

### 安装指南

1. 安装 Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 安装 Solana CLI (使用新的 Agave 安装器):
```bash
# 安装最新稳定版 (推荐)
curl -sSfL https://release.anza.xyz/stable/install | sh

# 或安装特定版本
# curl -sSfL https://release.anza.xyz/v2.2.21/install | sh
```

**注意**: Solana Labs 仓库已于 2025年1月归档，现由 Anza 维护 Agave 项目。

3. 安装 Anchor CLI:
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

4. 验证安装:
```bash
rustc --version    # 应显示 1.89.0 或更高版本
solana --version   # 应显示 2.2.x (Agave) 或更高版本
anchor --version   # 应显示 0.31.x 或更高版本
```

**示例输出**:
```
rustc 1.89.0 (29483883e 2025-08-04)
solana-cli 2.2.21 (src:23e01995; feat:3073396398, client:Agave)
anchor-cli 0.31.1
```

## 编译构建

### 方式一：使用 Anchor (推荐)

```bash
# 安装依赖
yarn install

# 构建智能合约
anchor build

# 运行测试
anchor test
```

### 方式二：使用 Cargo

```bash
# 编译合约
cargo build

# 发布模式编译
cargo build --release
```

**注意**: 如果遇到 `anchor build` 的 Cargo.lock 版本兼容问题，可以先使用 `cargo build` 进行开发和测试。

## 使用流程

### 1. 本地开发

```bash
# 克隆项目
git clone https://github.com/WalterZhu/sol-contract-template.git
cd sol-contract-template

# 安装依赖
yarn install

# 启动本地验证节点
solana-test-validator

# 构建和部署
anchor build
anchor deploy
```

### 2. 配置钱包

```bash
# 生成新钱包
solana-keygen new

# 或使用现有钱包
solana config set --keypair ~/.config/solana/id.json

# 获取测试代币
solana airdrop 2
```

### 3. 程序交互

智能合约提供以下功能：

- **initialize**: 初始化程序
- **store_data**: 存储数据到链上
- **update_data**: 更新已存储的数据
- **delete_data**: 删除数据并回收租金

### 4. 运行测试

```bash
# 运行所有测试
anchor test

# 或使用 yarn
yarn test

# 运行特定测试文件
npx ts-mocha -p ./tsconfig.json tests/sol-contract-template.ts
```

## 核心功能

### 数据存储 (store_data)
- 为用户创建数据账户
- 存储最大 1000 字符的数据
- 自动处理账户租金

### 数据更新 (update_data)
- 仅允许数据所有者更新
- 支持数据大小变更
- 自动调整账户大小

### 数据删除 (delete_data)
- 删除用户数据账户
- 回收账户租金给用户
- 权限验证保护

## 部署

### 本地网络部署

```bash
# 启动本地验证节点
solana-test-validator

# 部署到本地网络
anchor deploy
```

### 测试网部署

```bash
# 切换到测试网
solana config set --url devnet

# 获取测试代币
solana airdrop 2

# 部署到测试网
anchor deploy --provider.cluster devnet
```

### 主网部署

```bash
# 切换到主网
solana config set --url mainnet-beta

# 部署到主网 (确保有足够的 SOL)
anchor deploy --provider.cluster mainnet-beta
```

## 开发指南

### 代码风格

项目使用 Prettier 进行代码格式化：

```bash
# 检查代码格式
yarn lint

# 自动修复格式
yarn lint:fix
```

### 添加新功能

1. 在 `instructions/` 目录添加新指令
2. 在 `state/` 目录定义相关状态
3. 在 `lib.rs` 中注册新的程序方法
4. 在 `tests/` 目录添加测试用例

### 错误处理

所有自定义错误定义在 `error.rs` 文件中，包括：
- `DataEmpty`: 数据为空
- `DataTooLong`: 数据过长
- `Unauthorized`: 权限不足

## 故障排除

### Platform-tools 下载问题

如果遇到 `anchor build` 时 platform-tools 相关错误，可以手动下载：

**Apple Silicon (M1/M2/M3) Mac**:
```bash
curl -L -o platform-tools.tar.bz2 \
  "https://github.com/anza-xyz/platform-tools/releases/download/v1.48/platform-tools-osx-aarch64.tar.bz2"
```

**Intel Mac**:
```bash
curl -L -o platform-tools.tar.bz2 \
  "https://github.com/anza-xyz/platform-tools/releases/download/v1.48/platform-tools-osx-x86_64.tar.bz2"
```

然后解压到正确位置：
```bash
# 解压到 Solana 安装目录
tar -xjf platform-tools.tar.bz2 -C ~/.cache/solana/v1.48/platform-tools/
```

### 其他常见问题

- **Cargo.lock 版本错误**: 删除 `Cargo.lock` 后重新运行 `cargo update`
- **网络连接问题**: 检查防火墙设置，确保可以访问 GitHub 和 crates.io
- **磁盘空间不足**: platform-tools 约需要 300MB 空间

## 部署信息

### Testnet 部署

**程序地址**: `CSeMuL6j2DLdbiiQo7i4NU9yy99ohJ5wCmw8QabFJcAn`

**网络**: Solana Devnet

**浏览器链接**: 
- [程序详情](https://explorer.solana.com/address/CSeMuL6j2DLdbiiQo7i4NU9yy99ohJ5wCmw8QabFJcAn?cluster=devnet)
- [部署交易](https://explorer.solana.com/tx/35AncvKki4bmHBDAuUhjpYrbSRHshKjmEfBnV1pxMo2xNbiLwYRAJoQkNwMESiCpjLvKdCe9Q5pobADPUeESAJkD?cluster=devnet)

**程序功能测试**:
```bash
# 连接到 devnet
solana config set --url devnet

# 查看程序信息
solana program show CSeMuL6j2DLdbiiQo7i4NU9yy99ohJ5wCmw8QabFJcAn
```

## 许可证

本项目采用 ISC 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 联系方式

- GitHub: [WalterZhu](https://github.com/WalterZhu)
- 项目地址: https://github.com/WalterZhu/sol-contract-template