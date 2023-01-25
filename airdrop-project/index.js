const {
  Connection,
  PublicKey,
  clusterApiUrl,
  Keypair,
  LAMPORTS_PER_SOL,
} = require("@solana/web3.js");

const getWalletBalance = async (_publicKey) => {
  try {
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    const balance = await connection.getBalance(_publicKey);
    console.log("Balance: ", balance);
  } catch (err) {
    console.log(err);
  }
};

const airDropSol = async (_publicKey) => {
  try {
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    const fromAirDropSignature = await connection.requestAirdrop(
      _publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction({ signature: fromAirDropSignature });
  } catch (err) {
    console.log(err);
  }
};

const main = async () => {
  const wallet = new Keypair();

  const publicKey = new PublicKey(wallet._keypair.publicKey);
  const secretKey = wallet._keypair.secretKey;

  await getWalletBalance(publicKey);
  await airDropSol(publicKey);
  await getWalletBalance(publicKey);
};

main();
