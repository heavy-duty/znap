'use client';

import { Hat } from '@/utils/hat';
import { HatDetailInfo } from './hat-ui';

export default async function HatDetailFeature({ hat }: { hat: Hat }) {
  return (
    <div className="mt-4 flex gap-4">
      <img src={hat.imageUrl} className="w-[600px] h-[600px] object-fill" />

      <HatDetailInfo hat={hat} />
    </div>
  );
}
