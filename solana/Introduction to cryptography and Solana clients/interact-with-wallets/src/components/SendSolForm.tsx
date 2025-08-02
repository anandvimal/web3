'use client';

import { FC, FormEvent } from 'react';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { PublicKey, Transaction, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js';

export const SendSolForm: FC = () => {
  const { connection } = useConnection();
  const { publicKey, sendTransaction } = useWallet();

  const sendSol = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const recipient = (event.currentTarget.elements.namedItem('recipient') as HTMLInputElement).value;

    if (!publicKey) {
      console.error('Wallet not connected');
      return;
    }

    try {
      const recipientPubKey = new PublicKey(recipient);
      const transaction = new Transaction();
      const sendSolInstruction = SystemProgram.transfer({
        fromPubkey: publicKey,
        toPubkey: recipientPubKey,
        lamports: 0.1 * LAMPORTS_PER_SOL,
      });

      transaction.add(sendSolInstruction);

      const signature = await sendTransaction(transaction, connection);
      console.log(`Transaction signature: ${signature}`);
      window.open(`https://explorer.solana.com/tx/${signature}?cluster=devnet`, '_blank');
    } catch (error) {
      console.error('Transaction failed:', error);
    }
  };

  return (
    <form onSubmit={sendSol} className="flex flex-col items-center mt-4">
      <input
        type="text"
        name="recipient"
        placeholder="Recipient Public Key"
        className="px-4 py-2 m-2 border rounded bg-gray-800 text-white border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500 w-80"
      />
      <button type="submit" className="px-4 py-2 m-2 bg-blue-500 text-white rounded hover:bg-blue-600">
        Send 0.1 SOL
      </button>
    </form>
  );
};