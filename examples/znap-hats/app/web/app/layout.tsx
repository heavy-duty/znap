import './global.css';
import { UiLayout } from '@/components/ui/ui-layout';
import { ClusterProvider } from '@/components/cluster/cluster-data-access';
import { SolanaProvider } from '@/components/solana/solana-provider';
import { ReactQueryProvider } from './react-query-provider';

export const metadata = {
  title: 'Welcome to the Znap Hat Store',
  description: 'An example made by Znap',
  applicationName: 'Znap Hat Store',
  authors: [{ name: 'Heavy Duty Builders', url: 'https://x.com/HeavyDutyBuild' }]
};

const links: { label: string; path: string }[] = [
  { label: 'Hats', path: '/hats' },
  { label: 'Account', path: '/account' },
  { label: 'Clusters', path: '/clusters' },
];

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <ReactQueryProvider>
          <ClusterProvider>
            <SolanaProvider>
              <UiLayout links={links}>{children}</UiLayout>
            </SolanaProvider>
          </ClusterProvider>
        </ReactQueryProvider>
      </body>
    </html>
  );
}
