'use client';

import { FC } from 'react';
import Image from 'next/image';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

export const AppBar: FC = () => {
  return (
    <div className="flex items-center justify-between p-4 bg-gray-800 w-full">
      <Image src="/solanaLogo.png" height={30} width={200} alt="Solana Logo" />
      <span>Wallet-Adapter Example</span>
      <WalletMultiButton />
    </div>
  );
};