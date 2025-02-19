use {
    super::check_if_rpc_is_responding_correctly_for_supported_chain,
    crate::context::ServerContext,
    test_context::test_context,
};

#[test_context(ServerContext)]
#[tokio::test]
#[ignore]
async fn infura_provider(ctx: &mut ServerContext) {
    // Ethereum mainnet
    check_if_rpc_is_responding_correctly_for_supported_chain(ctx, "eip155:1", "0x1").await;

    // Ethereum Goerli
    check_if_rpc_is_responding_correctly_for_supported_chain(ctx, "eip155:5", "0x5").await;

    // Ethereum Sepolia
    check_if_rpc_is_responding_correctly_for_supported_chain(ctx, "eip155:11155111", "0xaa36a7")
        .await;

    // Polgyon mainnet
    check_if_rpc_is_responding_correctly_for_supported_chain(ctx, "eip155:137", "0x89").await;

    // Polygon mumbai
    check_if_rpc_is_responding_correctly_for_supported_chain(ctx, "eip155:80001", "0x13881").await;

    // Optimism mainnet
    check_if_rpc_is_responding_correctly_for_supported_chain(ctx, "eip155:10", "0xa").await;

    // Optimism goerli
    check_if_rpc_is_responding_correctly_for_supported_chain(ctx, "eip155:420", "0x1A4").await;

    // Arbitrum mainnet
    check_if_rpc_is_responding_correctly_for_supported_chain(ctx, "eip155:42161", "0xa4b1").await;

    // Arbitrum goerli
    check_if_rpc_is_responding_correctly_for_supported_chain(ctx, "eip155:421613", "0x66eed").await;

    // Aurora mainnet
    check_if_rpc_is_responding_correctly_for_supported_chain(
        ctx,
        "eip155:1313161554",
        "0x4e454152",
    )
    .await;

    // Aurora testnet
    check_if_rpc_is_responding_correctly_for_supported_chain(
        ctx,
        "eip155:1313161555",
        "0x4e454153",
    )
    .await;

    // Base Goerli
    check_if_rpc_is_responding_correctly_for_supported_chain(ctx, "eip155:84531", "0x14a33").await
}
