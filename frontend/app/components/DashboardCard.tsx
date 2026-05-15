interface DashboardCardProps {
  title: string;
  value: number;
  color: 'blue' | 'yellow' | 'red';
}

export default function DashboardCard({ title, value, color }: DashboardCardProps) {
  const colorMap = {
    blue: 'bg-blue-100 text-gray-900',
    yellow: 'bg-yellow-100 text-gray-900',
    red: 'bg-red-100 text-gray-900',
  };

  return (
    <div className={`${colorMap[color]} p-4 rounded-lg`}>
      <p className="text-sm text-gray-500">{title}</p>
      <p className="text-3xl font-bold">{value}</p>
    </div>
  );
}
