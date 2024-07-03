'use client';

import { Car } from '@/utils/cars';
import { useTransferUsdc } from './car-data-access';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

export function CarDetailInfo({ car }: { car: Car }) {
  return (
    <div className="w-[300px] border-base-100 bg-base-300 shadow-xl rounded-md overflow-hidden">
      <div className="p-4 text-neutral-content h-full flex flex-col">
        <div className="grow">
          <h2>{car.title}</h2>

          <p>
            $<span className="text-4xl">{car.price.toLocaleString()}</span>
          </p>
        </div>

        <div>
          <CarDetailBuyButton price={car.price} />
        </div>
      </div>
    </div>
  );
}

export function CarDetailBuyButton({ price }: { price: number }) {
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

export function CarListItem({ car }: { car: Car }) {
  return (
    <a href={car.detailsUrl}>
      <div className="border-2 border-base-100 bg-base-300 shadow-xl rounded-md overflow-hidden hover:bg-base-200 hover:shadow-2xl hover:rotate-1 hover:border-base-100">
        <img src={car.imageUrl} className="w-[300px] h-[200px] object-fill" />

        <div className="p-4 text-neutral-content h-24">
          <h2>{car.title}</h2>

          <p>
            $<span className="text-4xl">{car.price.toLocaleString()}</span>
          </p>
        </div>
      </div>
    </a>
  );
}
