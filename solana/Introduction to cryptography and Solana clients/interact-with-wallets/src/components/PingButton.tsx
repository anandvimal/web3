'use client';

import { FC } from 'react';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';
import styles from './PingButton.module.css';

export const PingButton: FC = () => {
  const { connection } = useConnection();
  const { publicKey, sendTransaction } = useWallet();

  const onClick = async () => {
    if (!connection || !publicKey) {
      console.error('Wallet not connected or connection unavailable');
      return;
    }

    try {
      const programId = new PublicKey('ChT1B39WKLS8qUrkLvFDXMhEJ4F1XZzwUNHUt4AU9aVa');
      const programDataAccount = new PublicKey('Ah9K7dQ8EHaZqcAsgBW8w37yN2eAy3koFmUn4x3CJtod');
      const transaction = new Transaction();

      const instruction = new TransactionInstruction({
        keys: [
          {
            pubkey: programDataAccount,
            isSigner: false,
            isWritable: true,
          },
        ],
        programId,
      });

      transaction.add(instruction);

      const signature = await sendTransaction(transaction, connection);
      console.log('Transaction Signature:', signature);
      window.open(`https://explorer.solana.com/tx/${signature}?cluster=devnet`, '_blank');
    } catch (error) {
      console.error('Transaction failed:', error);
    }
  };

  return (
    <div className={styles.buttonContainer}>
      <button className={styles.button} onClick={onClick}>
        Ping!
      </button>
    </div>
  );
};