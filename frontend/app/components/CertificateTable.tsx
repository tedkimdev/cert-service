'use client';

import useSWR from 'swr';
import { CertificateListResponse } from '../types/certificate';
import { certificatesFetcher } from '../lib/api';

interface CertificateTableProps {
  initialData: CertificateListResponse;
  cursor?: string;
}

export default function CertificateTable({ initialData, cursor }: CertificateTableProps) {
  const params = new URLSearchParams();
  if (cursor) params.append('cursor', cursor);
  params.append('limit', '10');

  const { data, isLoading } = useSWR<CertificateListResponse>(
    `/api/certificates?${params}`,
    certificatesFetcher,
    { fallbackData: initialData }
  );

  if (!data) return <p>Loading...</p>;

  return (
    <table className="w-full border-collapse">
      <thead>
        <tr className="bg-gray-100 text-gray-900">
          <th className="p-3 text-left">Subject</th>
          <th className="p-3 text-left">Issuer</th>
          <th className="p-3 text-left">Expiration</th>
          <th className="p-3 text-left">SANs</th>
        </tr>
      </thead>
      <tbody>
        {data?.data.map(cert => (
          <tr
            key={cert.id}
            className="border-b hover:bg-gray-50 cursor-pointer hover:text-gray-900 relative"
          >
            <td className="p-3">
              <a href={`/inventory/${cert.id}`}
                className="absolute inset-0"
              />\
              {cert.subject}
            </td>
            <td className="p-3">{cert.issuer}</td>
            <td className="p-3">{new Date(cert.expiration).toLocaleDateString()}</td>
            <td className="p-3">{cert.san_entries.join(', ')}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}