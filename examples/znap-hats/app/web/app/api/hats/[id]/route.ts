import hats from '../../data.json';
import { NextResponse } from 'next/server';

export async function GET(_req: Request, context: any) {
  const hat = hats.find((hat) => hat.id === context.params.id);

  if (!hat) {
    return NextResponse.json({ error: 'Hat not found.' }, { status: 404 });
  }

  return NextResponse.json(hat, { status: 200 });
}
