solana-keygen new --force
solana balance --url devnet
solana-keygen pubkey
solana airdrop 2 7bTKsQ1jHrstRLwC3e41hbWU34b1rv9Fpw521tAKhgHj --url devnet

cargo install spl-token-cli
spl-token create-token --url devnet
// token: ZxHdUX47KuxfZ8PkASzv7D3WFbzkHvMmLyhXhhhgbFo

spl-token create-account ZxHdUX47KuxfZ8PkASzv7D3WFbzkHvMmLyhXhhhgbFo --url devnet
// account: B1KojSnRo8hYFcoyszYU5rpJYn353Fn6UicvjhViSWX6

spl-token balance ZxHdUX47KuxfZ8PkASzv7D3WFbzkHvMmLyhXhhhgbFo --url devnet

spl-token mint ZxHdUX47KuxfZ8PkASzv7D3WFbzkHvMmLyhXhhhgbFo 666 --url devnet

spl-token supply ZxHdUX47KuxfZ8PkASzv7D3WFbzkHvMmLyhXhhhgbFo --url devnet
spl-token authorize ZxHdUX47KuxfZ8PkASzv7D3WFbzkHvMmLyhXhhhgbFo mint --disable --url devnet

spl-token burn B1KojSnRo8hYFcoyszYU5rpJYn353Fn6UicvjhViSWX6 1 --url devnet

// tranfer token to wallet
spl-token transfer ZxHdUX47KuxfZ8PkASzv7D3WFbzkHvMmLyhXhhhgbFo 150 7iCLPVu2CRUT7Mjyh5W1gkKTsHSJv1ei1tW52A453HLt --url devnet