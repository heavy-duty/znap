import { Car } from '@/utils/cars';
import CarDetailFeature from '@/components/car/car-detail-feature';

export const getCar = async (id: string) => {
  const res = await fetch(`http://localhost:3000/api/cars/${id}`, {
    headers: {
      Accept: 'application/json',
      method: 'GET',
    },
  });
  const cars = (await res.json()) as Car;

  return cars;
};

export default async function Page({ params }: { params: { id: string } }) {
  const car = await getCar(params.id);

  return <CarDetailFeature car={car} />;
}
