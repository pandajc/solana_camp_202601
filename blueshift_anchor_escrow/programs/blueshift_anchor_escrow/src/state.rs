use anchor_lang::prelude::*;

// InitSpace不需要手动计算这个结构的租金，InitSpace 派生宏会自动计算所需空间
// 总空间 = INIT_SPACE（所有字段的总大小） + DISCRIMINATOR.len()
#[derive(InitSpace)]
// old version: Anchor assigns a unique 8 byte discriminator to each instruction and account type in a program. These discriminators serve as identifiers to distinguish between different instructions or account types.
// The discriminator is generated using the first 8 bytes of the Sha256 hash of a prefix combined with the instruction or account name
// from 0.31.0: Due to the transaction size limits enforced by the Solana runtime (1232 bytes), 8 bytes can be too high for some use cases
// The Discriminator trait had a fixed size type field ([u8; 8])
#[account(discriminator = 1)]
pub struct Escrow {
    pub seed: u64,
    // 创建托管账户的钱包；需要用于退款和接收付款
    pub maker: Pubkey,
    // 交换中“给出”和“获取”两侧的 SPL 铸币地址
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    // 创建者希望获得的代币 B 的数量
    pub receive: u64,
    // 缓存的 bump 字节；动态派生它会消耗计算资源，因此我们将其保存一次
    pub bump: u8,

}
