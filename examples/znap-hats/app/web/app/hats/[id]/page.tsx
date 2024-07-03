import { Hat } from '@/utils/hat';
import HatDetailFeature from '@/components/hat/hat-detail-feature';

export const getHat = async (id: string) => {
  const res = await fetch(`http://localhost:3000/api/hats/${id}`, {
    headers: {
      Accept: 'application/json',
      method: 'GET',
    },
  });
  const hats = (await res.json()) as Hat;

  return hats;
};

export default async function Page({ params }: { params: { id: string } }) {
  const hat = await getHat(params.id);

  return <HatDetailFeature hat={hat} />;
}
