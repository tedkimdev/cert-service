import { getCertificates } from '../lib/api';
import DashboardCard from '../components/DashboardCard';
import CertificateTable from '../components/CertificateTable';

export default async function InventoryPage() {
  const { data: certificates, total } = await getCertificates();

  const expiringSoon = certificates.filter(cert => {
    const daysLeft = Math.ceil(
      (new Date(cert.expiration).getTime() - Date.now()) / (1000 * 60 * 60 * 24)
    );
    return daysLeft <= 30 && daysLeft > 0;
  }).length;

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
          value={expiringSoon}
          color="yellow"
        />
      </div>

      {/* certificate list */}
      <CertificateTable certificates={certificates} />
    </main>
  );
}