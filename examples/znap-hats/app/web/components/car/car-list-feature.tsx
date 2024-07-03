'use client';

import { Car } from '@/utils/cars';
import { CarListItem } from './car-ui';

export default function CarListFeature({ cars }: { cars: Car[] }) {
  return (
    <ul className="flex justify-center gap-4 p-4">
      {cars.map((car) => (
        <li key={car.id}>
          <CarListItem car={car} />
        </li>
      ))}
    </ul>
  );
}
