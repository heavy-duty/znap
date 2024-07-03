import CarListFeature from '@/components/car/car-list-feature';
import { Car } from '@/utils/cars';

export const getCars = async () => {
  const res = await fetch('http://localhost:3000/api/cars', {
    headers: {
      Accept: 'application/json',
      method: 'GET',
    },
  });
  const cars = (await res.json()) as Car[];

  return cars;
};

export default async function Page() {
  const cars = await getCars();

  return <CarListFeature cars={cars} />;
}
