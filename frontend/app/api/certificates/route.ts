import { NextRequest, NextResponse } from 'next/server';

const API_URL = process.env.API_URL || 'https://localhost:3000';

process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

export async function GET(request: NextRequest) {
  const searchParams = request.nextUrl.searchParams;
  const cursor = searchParams.get('cursor');
  const limit = searchParams.get('limit') || '10';

  const params = new URLSearchParams();
  if (cursor) params.append('cursor', cursor);
  params.append('limit', limit);

  const res = await fetch(`${API_URL}/certificates?${params}`);
  const data = await res.json();

  return NextResponse.json(data);
}