import { getCertificates } from '../lib/api';
import DashboardCard from '../components/DashboardCard';
import CertificateTable from '../components/CertificateTable';

interface PageProps {
  searchParams: Promise<{ cursor?: string; }>;
}

export default async function InventoryPage({ searchParams }: PageProps) {
  const { cursor } = await searchParams;
//   const { data: certificates, total, next_cursor, has_more, expiring_soon_count } = await getCertificates(cursor);
  const response = await getCertificates(cursor);
  const { total, next_cursor, has_more, expiring_soon_count } = response;

  return (
    <main className="p-8">
      <h1 className="text-2xl font-bold mb-6">Certificate Inventory</h1>

      {/* dashboard */}
      <div className="grid grid-cols-2 gap-4 mb-8">
        <DashboardCard
          title="Total Certificates"
          value={total}
          color="blue"
        />
        <DashboardCard
          title="Expiring Soon (30 days)"
          value={expiring_soon_count}
          color="yellow"
        />
      </div>

      {/* certificate list */}
      <CertificateTable 
        initialData={response}
        cursor={cursor}
      />

      {/* Pagination */}
      <div className="mt-4 flex justify-end gap-2">
        {cursor && (
          <a
            href="/inventory"
            className="px-4 py-2 bg-gray-600 rounded hover:bg-gray-200 hover:text-gray-900"
          >
            First Page
          </a>
        )}
        {has_more && next_cursor && (
          <a
            href={`/inventory?cursor=${next_cursor}`}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            Next Page
          </a>
        )}
      </div>

    </main>
  );
}