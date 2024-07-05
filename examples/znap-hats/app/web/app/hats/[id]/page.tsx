import { Hat } from '@/utils/hat';
import HatDetailFeature from '@/components/hat/hat-detail-feature';

type Props = {
   params: { id: string } 
}

const getHat = async (id: string) => {
  const res = await fetch(`http://localhost:5020/api/hats/${id}`, {
    headers: {
      Accept: 'application/json',
      method: 'GET',
    },
  });
  const hats = (await res.json()) as Hat;

  return hats;
};


export async function generateMetadata( { params }: Props) {
  const hat = await getHat(params.id);

  return {
    title: hat.title,
    openGraph: {
      title: hat.title,
      description: 'Check out this awesome cap',
      url: `https://testing-store.heavyduty.builders/hats/${hat.id}`,
      siteName: 'Znap Store',
      images: [{
        url: hat.shareImageUrl, 
        width: 2250,
        height: 2250, 
      }],
    },
    twitter: {
      card: 'summary_large_image',
      title: hat.title,
      description: 'Check out this awesome cap',
      creator: '@HeavyDutyBuild',
      images: [hat.shareImageUrl],
    }
  }
}

export default async function Page({ params }: Props) {
  const hat = await getHat(params.id);

  return <HatDetailFeature hat={hat} />;
}
