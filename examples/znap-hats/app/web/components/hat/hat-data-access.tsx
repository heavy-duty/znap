'use client';

import { useTransactionToast } from '@/components/ui/ui-layout';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import {
  Connection,
  PublicKey,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
} from '@solana/web3.js';
import { useQueryClient, useMutation } from '@tanstack/react-query';
import toast from 'react-hot-toast';
import {
  createTransferCheckedInstruction,
  getAssociatedTokenAddress,
} from '@solana/spl-token';

const DESTINATION_PUBLIC_KEY = new PublicKey(
  'Fpb6uVk3tWrQ93og9WZm581s9Wge5BJPFAkbjS6nLzNJ'
);
const MINT_PUBLIC_KEY = new PublicKey(
  '5R5kzomKtVjciTSHEaSZ6RcgEGCzjZeQ7NnstVModK6Q'
);
const MINT_DECIMALS = 6;

export function useTransferUsdc() {
  const { connection } = useConnection();
  const transactionToast = useTransactionToast();
  const wallet = useWallet();
  const client = useQueryClient();

  return useMutation({
    mutationFn: async (input: { amount: number }) => {
      const publicKey = wallet.publicKey;

      if (!publicKey) {
        throw new Error('Wallet Not Connected');
      }

      // get all addresses and shit.
      const senderPublicKey = await getAssociatedTokenAddress(
        MINT_PUBLIC_KEY,
        publicKey
      );
      const receiverPublicKey = await getAssociatedTokenAddress(
        MINT_PUBLIC_KEY,
        DESTINATION_PUBLIC_KEY
      );
      const fullAmount = input.amount * Math.pow(10, MINT_DECIMALS);

      const transferInstruction = createTransferCheckedInstruction(
        senderPublicKey,
        MINT_PUBLIC_KEY,
        receiverPublicKey,
        publicKey,
        fullAmount,
        MINT_DECIMALS
      );

      const { transaction, latestBlockhash } = await createTransaction({
        payerPublicKey: publicKey,
        connection,
        instructions: [transferInstruction],
      });

      // Send transaction and await for signature
      const signature = await wallet.sendTransaction(transaction, connection);

      // Send transaction and await for signature
      await connection.confirmTransaction(
        { signature, ...latestBlockhash },
        'confirmed'
      );

      console.log(signature);
      return signature;
    },
    onSuccess: (signature) => {
      if (signature) {
        transactionToast(signature);
      }
    },
    onError: (error) => {
      toast.error(`Transaction failed! ${error}`);
    },
  });
}

export async function createTransaction({
  payerPublicKey,
  instructions,
  connection,
}: {
  payerPublicKey: PublicKey;
  connection: Connection;
  instructions: TransactionInstruction[];
}): Promise<{
  transaction: VersionedTransaction;
  latestBlockhash: { blockhash: string; lastValidBlockHeight: number };
}> {
  // Get the latest blockhash to use in our transaction
  const latestBlockhash = await connection.getLatestBlockhash();

  // Create a new TransactionMessage with version and compile it to legacy
  const messageLegacy = new TransactionMessage({
    payerKey: payerPublicKey,
    recentBlockhash: latestBlockhash.blockhash,
    instructions,
  }).compileToLegacyMessage();

  // Create a new VersionedTransaction which supports legacy and v0
  const transaction = new VersionedTransaction(messageLegacy);

  return {
    transaction,
    latestBlockhash,
  };
}
