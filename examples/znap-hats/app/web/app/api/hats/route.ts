import hats from '../data.json';
import { NextResponse } from 'next/server';

export async function GET(_req: Request) {
  return NextResponse.json(hats);
}
