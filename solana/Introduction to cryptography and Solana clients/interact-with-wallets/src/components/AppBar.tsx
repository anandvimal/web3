'use client';

import { FC } from 'react';
import Image from 'next/image';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import styles from './AppBar.module.css';

export const AppBar: FC = () => {
  return (
    <div className={styles.header}>
      <Image src="/solanaLogo.png" height={30} width={200} alt="Solana Logo" />
      <span>Wallet-Adapter Example</span>
      <WalletMultiButton />
    </div>
  );
};