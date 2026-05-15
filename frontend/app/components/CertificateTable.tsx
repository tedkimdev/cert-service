import Link from 'next/link';
import { Certificate } from '../types/certificate';

interface CertificateTableProps {
  certificates: Certificate[];
}

export default function CertificateTable({ certificates }: CertificateTableProps) {
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
        {certificates.map(cert => (
          <tr key={cert.id} className="border-b hover:bg-gray-50">
            <td className="p-3">
              <Link href={`/inventory/${cert.id}`} className="block w-full">
                {cert.subject}
              </Link>
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