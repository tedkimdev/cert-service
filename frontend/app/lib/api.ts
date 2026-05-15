import { Certificate, CertificateListResponse } from '../types/certificate';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';

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
