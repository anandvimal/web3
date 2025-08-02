'use client';

import { FC } from 'react';
import { AppBar } from '@/components/AppBar';
import { BalanceDisplay } from '@/components/BalanceDisplay';
import { SendSolForm } from '@/components/SendSolForm';
import { PingButton } from '@/components/PingButton';

const Home: FC = () => {
  return (
    <div className="flex flex-col items-center min-h-screen bg-gray-900 text-white">
      <AppBar />
      <div className="flex flex-col items-center justify-center flex-grow space-y-4">
        <BalanceDisplay />
        <SendSolForm />
        <PingButton />
      </div>
    </div>
  );
};

export default Home;