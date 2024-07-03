import cars from '../../data.json';
import { NextResponse } from 'next/server';

export async function GET(_req: Request, context: any) {
  let car = cars.find((car) => car.id === context.params.id);

  if (!car) {
    return NextResponse.json({ error: 'Car not found.' }, { status: 404 });
  }

  return NextResponse.json(car, { status: 200 });
}
