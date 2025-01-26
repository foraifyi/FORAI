import * as React from 'react';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { Project, Milestone } from '../../types';
import { createMilestoneTransaction } from '../../utils/transaction';
import { MilestoneForm } from './MilestoneForm';
import { MilestoneList } from './MilestoneList';
import { updateMilestone } from '../../utils/project';

interface MilestoneManagerProps {
  project: Project;
  onUpdate: (milestones: Milestone[]) => Promise<void>;
}

export const MilestoneManager: React.FC<MilestoneManagerProps> = ({ project, onUpdate }) => {
  const { connection } = useConnection();
  const { publicKey, signTransaction } = useWallet();
  const [milestones, setMilestones] = React.useState<Milestone[]>(project.milestones);
  const [processing, setProcessing] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);

  const addMilestone = () => {
    setMilestones([
      ...milestones,
      {
        id: Date.now().toString(),
        title: '',
        description: '',
        deadline: new Date(),
        completed: false
      }
    ]);
  };

  const updateMilestone = async (index: number, completed: boolean) => {
    try {
      setProcessing(true);
      setError(null);

      const updatedMilestones = [...milestones];
      updatedMilestones[index].completed = completed;

      await onUpdate(updatedMilestones);
      setMilestones(updatedMilestones);
    } catch (err) {
      console.error('Failed to update milestone:', err);
      setError('Failed to update milestone. Please try again.');
    } finally {
      setProcessing(false);
    }
  };

  async function handleMilestoneComplete(milestoneId: string) {
    if (!publicKey || !signTransaction) return;

    try {
      setProcessing(true);
      setError(null);

      const transaction = await createMilestoneTransaction(
        connection,
        project.id,
        milestoneId,
        'complete'
      );

      await signTransaction(transaction);
      await connection.sendRawTransaction(transaction.serialize());

      const updatedMilestones = milestones.map((milestone) =>
        milestone.id === milestoneId ? { ...milestone, completed: true } : milestone
      );

      await onUpdate(updatedMilestones);
      setMilestones(updatedMilestones);
    } catch (err) {
      console.error('Failed to complete milestone:', err);
      setError('Failed to complete milestone. Please try again.');
    } finally {
      setProcessing(false);
    }
  }

  return (
    <div className="milestone-manager">
      <h2>Project Milestones</h2>

      <MilestoneList
        milestones={milestones}
        onComplete={handleMilestoneComplete}
        loading={processing}
      />

      {publicKey?.equals(project.id) && (
        <MilestoneForm
          project={project.id}
          onSubmit={(newMilestone) => {
            onUpdate([...milestones, newMilestone]);
          }}
        />
      )}

      <button 
        onClick={addMilestone}
        disabled={processing}
      >
        Add Milestone
      </button>

      {error && <div className="error-message">{error}</div>}
    </div>
  );
}; 