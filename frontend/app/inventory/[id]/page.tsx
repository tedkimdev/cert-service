import { getCertificate } from '../../lib/api';

interface PageProps {
  params: Promise<{ id: string }>;
}

export default async function CertificateDetailPage({ params }: PageProps) {
  const { id } = await params;
  const cert = await getCertificate(id);

  return (
    <main className="p-8">
      <a href="/inventory" className="text-blue-500 hover:underline mb-6 block">
        Back to Inventory
      </a>

      <h1 className="text-2xl font-bold mb-6">Certificate Detail</h1>

      <div className="bg-white border rounded-lg p-6 space-y-4">
        <div>
          <p className="text-sm text-gray-900">ID</p>
          <p className="font-mono text-gray-900">{cert.id}</p>
        </div>
        <div>
          <p className="text-sm text-gray-900">Subject</p>
          <p className="text-gray-900">{cert.subject}</p>
        </div>
        <div>
          <p className="text-sm text-gray-900">Issuer</p>
          <p className="text-gray-900">{cert.issuer}</p>
        </div>
        <div>
          <p className="text-sm text-gray-900">Expiration</p>
          <p className="text-gray-900">{new Date(cert.expiration).toLocaleString()}</p>
        </div>
        <div>
          <p className="text-sm text-gray-900">Created At</p>
          <p className="text-gray-900">{new Date(cert.created_at).toLocaleString()}</p>
        </div>
        <div>
          <p className="text-sm text-gray-900">SAN Entries</p>
          <ul className="list-disc list-inside">
            {cert.san_entries.map((san, i) => (
              <li key={i} className="text-gray-900">{san}</li>
            ))}
          </ul>
        </div>
      </div>
    </main>
  );
}