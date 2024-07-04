import HatListFeature from '@/components/hat/hat-list-feature';
import { Hat } from '@/utils/hat';

const getHats = async () => {
  const res = await fetch('http://localhost:5020/api/hats', {
    headers: {
      Accept: 'application/json',
      method: 'GET',
    },
  });
  const hats = (await res.json()) as Hat[];

  return hats;
};

export const metadata = {
  title: 'Check out our caps',
}

export default async function Page() {
  const hats = await getHats();

  return <HatListFeature hats={hats} />;
}
