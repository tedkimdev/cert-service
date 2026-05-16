export interface Certificate {
  id: string;
  subject: string;
  issuer: string;
  expiration: string;
  san_entries: string[];
  created_at: string;
}

export interface CertificateListResponse {
  data: Certificate[];
  next_cursor: string | null;
  has_more: boolean;
  total: number;
  expiring_soon_count: number;
}

