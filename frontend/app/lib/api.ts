import { Certificate, CertificateListResponse } from '../types/certificate';

// dev only - ignore self-signed cert
process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'https://localhost:3000';

export async function getCertificates(cursor?: string, limit = 10): Promise<CertificateListResponse> {
  const params = new URLSearchParams();
  if (cursor) params.append('cursor', cursor);
  params.append('limit', limit.toString());

  const res = await fetch(`${API_URL}/certificates?${params}`, {
    cache: 'no-store',
  });

  if (!res.ok) {
    throw new Error('Failed to fetch certificates');
  }

  return res.json();
}

export async function getCertificate(id: string): Promise<Certificate> {
  const res = await fetch(`${API_URL}/certificates/${id}`, {
    cache: 'no-store',
  });

  if (!res.ok) {
    throw new Error('Failed to fetch certificate');
  }

  return res.json();
}

export const certificatesFetcher = (url: string) =>
  fetch(url).then(res => res.json());
