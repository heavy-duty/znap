import HatListFeature from '@/components/hat/hat-list-feature';
import { Hat } from '@/utils/hat';

export const getHats = async () => {
  const res = await fetch('http://localhost:3000/api/hats', {
    headers: {
      Accept: 'application/json',
      method: 'GET',
    },
  });
  const hats = (await res.json()) as Hat[];

  return hats;
};

export default async function Page() {
  const hats = await getHats();

  return <HatListFeature hats={hats} />;
}
