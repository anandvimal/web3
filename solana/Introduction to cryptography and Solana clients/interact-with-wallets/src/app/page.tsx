'use client';

import { FC } from 'react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { AppBar } from '@/components/AppBar';
import { BalanceDisplay } from '@/components/BalanceDisplay';
import { SendSolForm } from '@/components/SendSolForm';
import { PingButton } from '@/components/PingButton';
import styles from './Home.module.css';

const Home: FC = () => {
  return (
    <div className={styles.container}>
      <AppBar />
      <div className={styles.body}>
        <BalanceDisplay />
        <SendSolForm />
        <PingButton />
      </div>
    </div>
  );
};

export default Home;