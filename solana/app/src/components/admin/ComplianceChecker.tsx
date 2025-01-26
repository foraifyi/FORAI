import * as React from 'react';
import { useConnection } from '@solana/wallet-adapter-react';
import { ComplianceReport } from '../../types';
import { runComplianceCheck } from '../../utils/admin';

export const ComplianceChecker: React.FC = () => {
  const { connection } = useConnection();
  const [report, setReport] = React.useState<ComplianceReport | null>(null);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);

  React.useEffect(() => {
    checkCompliance();
  }, [connection]);

  // ... rest of component
}; 