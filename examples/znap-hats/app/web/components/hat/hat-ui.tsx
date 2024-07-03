'use client';

import { Hat } from '@/utils/hat';
import { useTransferUsdc } from './hat-data-access';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

export function HatDetailInfo({ hat }: { hat: Hat }) {
  return (
    <div className="w-[300px] border-base-100 bg-base-300 shadow-xl rounded-md overflow-hidden">
      <div className="p-4 text-neutral-content h-full flex flex-col">
        <div className="grow">
          <h2>{hat.title}</h2>

          <p>
            $<span className="text-4xl">{hat.price.toLocaleString()}</span>
          </p>
        </div>

        <div>
          <HatDetailBuyButton price={hat.price} />
        </div>
      </div>
    </div>
  );
}

export function HatDetailBuyButton({ price }: { price: number }) {
  const wallet = useWallet();
  const mutation = useTransferUsdc();

  if (!wallet.publicKey) {
    return <WalletMultiButton className="btn btn-lg btn-primary w-full" />;
  }

  return (
    <button
      className="btn btn-lg btn-primary w-full"
      onClick={() => mutation.mutateAsync({ amount: price })}
    >
      Buy now
    </button>
  );
}

export function HatListItem({ hat }: { hat: Hat }) {
  return (
    <a href={hat.detailsUrl}>
      <div className="border-2 border-base-100 bg-base-300 shadow-xl rounded-md overflow-hidden hover:bg-base-200 hover:shadow-2xl hover:rotate-1 hover:border-base-100">
        <img src={hat.imageUrl} className="w-[300px] h-[300px] object-fill" />

        <div className="w-[300px] p-4 text-neutral-content h-24">
          <h2 className="truncate">{hat.title}</h2>

          <p>
            $<span className="text-4xl">{hat.price.toLocaleString()}</span>
          </p>
        </div>
      </div>
    </a>
  );
}
