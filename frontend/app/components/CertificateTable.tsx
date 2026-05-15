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
          <Link key={cert.id} href={`/inventory/${cert.id}`} legacyBehavior>
            <tr className="border-b hover:bg-gray-50 cursor-pointer">
              <td className="p-3">{cert.subject}</td>
              <td className="p-3">{cert.issuer}</td>
              <td className="p-3">{new Date(cert.expiration).toLocaleDateString()}</td>
              <td className="p-3">{cert.san_entries.join(', ')}</td>
            </tr>
          </Link>
        ))}
      </tbody>
    </table>
  );
}