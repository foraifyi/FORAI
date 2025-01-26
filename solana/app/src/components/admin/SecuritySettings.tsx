import * as React from 'react';
import { useConnection } from '@solana/wallet-adapter-react';
import { SecuritySettings as Settings } from '../../types';
import { updateSecuritySettings } from '../../utils/admin';
import { SecurityService } from '../../services/SecurityService';

interface SecuritySettingsProps {
  settings: Settings;
  onUpdate: (settings: Settings) => Promise<void>;
}

export const SecuritySettings: React.FC<SecuritySettingsProps> = ({
  settings: initialSettings,
  onUpdate
}) => {
  const { connection } = useConnection();
  const [settings, setSettings] = React.useState<Settings>(initialSettings);
  const [loading, setLoading] = React.useState(true);
  const [saving, setSaving] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);

  React.useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    try {
      setLoading(true);
      setError(null);
      const loadedSettings = await SecurityService.getSettings();
      setSettings(loadedSettings);
    } catch (err) {
      console.error('Failed to load security settings:', err);
      setError('Failed to load security settings. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const saveSettings = async () => {
    try {
      setSaving(true);
      setError(null);
      await updateSecuritySettings(connection, settings);
      
      if (onUpdate) {
        await onUpdate(settings);
      }
    } catch (err) {
      console.error('Failed to save security settings:', err);
      setError('Failed to save security settings. Please try again.');
    } finally {
      setSaving(false);
    }
  };

  function handleInputChange(e: React.ChangeEvent<HTMLInputElement>) {
    const { name, value, type, checked } = e.target;
    setSettings(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value
    }));
  }

  if (!settings) return <div>Loading security settings...</div>;

  return (
    <div className="security-settings">
      <h2>Security Settings</h2>

      <form onSubmit={saveSettings}>
        <div className="form-group">
          <label>
            <input
              type="checkbox"
              name="multiFactorAuth"
              checked={settings.multiFactorAuth}
              onChange={handleInputChange}
            />
            Enable Multi-Factor Authentication
          </label>
        </div>

        <div className="form-group">
          <label>
            <input
              type="checkbox"
              name="ipWhitelisting"
              checked={settings.ipWhitelisting}
              onChange={handleInputChange}
            />
            Enable IP Whitelisting
          </label>
        </div>

        <div className="form-group">
          <label>Maximum Failed Login Attempts</label>
          <input
            type="number"
            name="maxLoginAttempts"
            value={settings.maxLoginAttempts}
            onChange={handleInputChange}
            min="1"
            max="10"
          />
        </div>

        <div className="form-group">
          <label>Session Timeout (minutes)</label>
          <input
            type="number"
            name="sessionTimeout"
            value={settings.sessionTimeout}
            onChange={handleInputChange}
            min="5"
            max="120"
          />
        </div>

        {error && <div className="error-message">{error}</div>}

        <button type="submit" disabled={loading || saving}>
          {loading ? 'Loading...' : saving ? 'Saving...' : 'Save Settings'}
        </button>
      </form>
    </div>
  );
}; 