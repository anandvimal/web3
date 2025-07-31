import { Connection, LAMPORTS_PER_SOL, PublicKey} from "@solana/web3.js";

const publicKey = new PublicKey("DZiqa7he7jxc647PrgVxawA1L2GeJ74b8gHavaswhfGp");

const connection = new Connection("https://api.devnet.solana.com", "confirmed");

const balanceInLamports = await connection.getBalance(publicKey);

const balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL;

console.log(
  `💰 Finished! The balance for the wallet at address ${publicKey} is ${balanceInSOL}!`
);