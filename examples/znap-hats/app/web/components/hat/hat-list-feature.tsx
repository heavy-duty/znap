'use client';

import { Hat } from '@/utils/hat';
import { HatListItem } from './hat-ui';

export default function HatListFeature({ hats }: { hats: Hat[] }) {
  return (
    <ul className="flex justify-center gap-4 p-4 flex-wrap">
      {hats.map((hat) => (
        <li key={hat.id}>
          <HatListItem hat={hat} />
        </li>
      ))}
    </ul>
  );
}
