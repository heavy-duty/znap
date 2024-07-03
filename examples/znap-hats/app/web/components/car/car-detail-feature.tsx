'use client';

import { Car } from '@/utils/cars';
import { CarDetailInfo } from './car-ui';

export default async function CarDetailFeature({ car }: { car: Car }) {
  return (
    <div className="mt-4 flex gap-4">
      <img src={car.imageUrl} className="w-[600px] h-[400px] object-fill" />

      <CarDetailInfo car={car} />
    </div>
  );
}
