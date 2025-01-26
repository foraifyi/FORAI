import * as React from 'react';
import { useConnection } from '@solana/wallet-adapter-react';
import { User, UserRole, UserStatus } from '../../types';
import { fetchUsers, updateUserRole, updateUserStatus } from '../../utils/admin';
import { UserList } from './UserList';
import { UserFilters } from './UserFilters';
import { UserActions } from './UserActions';

interface UserManagementProps {
  onUserUpdate?: (user: User) => Promise<void>;
  onUpdateRole?: (userId: string, role: UserRole) => Promise<void>;
  onUpdateStatus?: (userId: string, status: UserStatus) => Promise<void>;
}

export const UserManagement: React.FC<UserManagementProps> = ({ onUserUpdate, onUpdateRole, onUpdateStatus }) => {
  const { connection } = useConnection();
  const [users, setUsers] = React.useState<User[]>([]);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);
  const [filters, setFilters] = React.useState({
    role: '',
    status: '',
    search: ''
  });

  React.useEffect(() => {
    loadUsers();
  }, [connection]);

  async function loadUsers() {
    try {
      setLoading(true);
      const userList = await fetchUsers(connection);
      setUsers(userList);
    } catch (err) {
      console.error('Failed to load users:', err);
      setError('Failed to load users. Please try again.');
    } finally {
      setLoading(false);
    }
  }

  async function handleRoleUpdate(userId: string, role: UserRole) {
    try {
      setError(null);
      await updateUserRole(connection, userId, role);
      if (onUpdateRole) {
        await onUpdateRole(userId, role);
      }
      await loadUsers();
    } catch (err) {
      console.error('Failed to update user role:', err);
      setError('Failed to update user role. Please try again.');
    }
  }

  async function handleStatusUpdate(userId: string, status: UserStatus) {
    try {
      setError(null);
      await updateUserStatus(connection, userId, status);
      if (onUpdateStatus) {
        await onUpdateStatus(userId, status);
      }
      await loadUsers();
    } catch (err) {
      console.error('Failed to update user status:', err);
      setError('Failed to update user status. Please try again.');
    }
  }

  const filteredUsers = users.filter(user => {
    return (
      (!filters.role || user.role === filters.role) &&
      (!filters.status || user.status === filters.status) &&
      (!filters.search || 
        user.id.includes(filters.search) ||
        user.profile?.name?.toLowerCase().includes(filters.search.toLowerCase()))
    );
  });

  return (
    <div className="user-management">
      <h2>User Management</h2>

      <UserFilters
        filters={filters}
        onFilterChange={setFilters}
      />

      {loading ? (
        <div>Loading users...</div>
      ) : (
        <UserList
          users={filteredUsers}
          onRoleUpdate={handleRoleUpdate}
          onStatusUpdate={handleStatusUpdate}
        />
      )}

      <UserActions users={filteredUsers} />

      {error && <div className="error-message">{error}</div>}
    </div>
  );
}; 