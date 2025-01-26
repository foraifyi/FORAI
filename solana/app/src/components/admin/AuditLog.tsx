import * as React from 'react';
import { useConnection } from '@solana/wallet-adapter-react';
import { AuditEvent, AuditFilters } from '../../types';
import { fetchAuditLogs } from '../../utils/admin';
import { AuditLogFilters } from './AuditLogFilters';
import { AuditLogTable } from './AuditLogTable';

interface AuditLogProps {
  filters?: AuditFilters;
  onFilterChange?: (filters: AuditFilters) => void;
}

export const AuditLog: React.FC<AuditLogProps> = ({
  filters: initialFilters,
  onFilterChange
}: AuditLogProps): JSX.Element => {
  const { connection } = useConnection();
  const [logs, setLogs] = React.useState<AuditEvent[]>([]);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);
  const [filters, setFilters] = React.useState<AuditFilters>(initialFilters || {
    startDate: '',
    endDate: '',
    eventType: '',
    severity: ''
  });

  React.useEffect(() => {
    loadAuditLogs();
  }, [connection, filters]);

  async function loadAuditLogs() {
    try {
      setLoading(true);
      const auditLogs = await fetchAuditLogs(connection, filters);
      setLogs(auditLogs);
    } catch (err) {
      console.error('Failed to load audit logs:', err);
      setError('Failed to load audit logs. Please try again.');
    } finally {
      setLoading(false);
    }
  }

  function handleFilterChange(newFilters: AuditFilters) {
    setFilters(newFilters);
    if (onFilterChange) {
      onFilterChange(newFilters);
    }
  }

  return (
    <div className="audit-log">
      <h2>Audit Log</h2>

      <AuditLogFilters
        filters={filters}
        onChange={handleFilterChange}
      />

      {loading ? (
        <div>Loading audit logs...</div>
      ) : (
        <AuditLogTable
          logs={logs}
          onSort={(field, direction) => {
            // Implement sorting logic
          }}
        />
      )}

      {error && <div className="error-message">{error}</div>}

      <div className="audit-summary">
        <div className="summary-item">
          <h4>Total Events</h4>
          <div className="value">{logs.length}</div>
        </div>
        <div className="summary-item">
          <h4>Critical Events</h4>
          <div className="value error">
            {logs.filter(log => log.severity === 'CRITICAL').length}
          </div>
        </div>
        <div className="summary-item">
          <h4>Warning Events</h4>
          <div className="value warning">
            {logs.filter(log => log.severity === 'WARNING').length}
          </div>
        </div>
      </div>
    </div>
  );
}; 