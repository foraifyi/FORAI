import * as React from 'react';
import { useParams } from 'react-router-dom';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { PublicKey } from '@solana/web3.js';
import { Project } from '../types';
import { deserializeProject } from '../utils/serialization';
import { MilestoneProgress } from './milestone/MilestoneProgress';
import { InvestmentForm } from './investment/InvestmentForm';
import { ProjectStats } from './ProjectStats';
import { ProjectActions } from './ProjectActions';
import { ProjectService } from '../services/ProjectService';

interface ProjectDetailProps {
  onUpdate?: (project: Project) => Promise<void>;
}

export const ProjectDetail: React.FC<ProjectDetailProps> = ({ onUpdate }) => {
  const { projectId } = useParams<{ projectId: string }>();
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [project, setProject] = React.useState<Project | null>(null);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);

  React.useEffect(() => {
    if (projectId) {
      loadProject();
    }
  }, [projectId, connection]);

  async function loadProject() {
    try {
      setLoading(true);
      const projectPubkey = new PublicKey(projectId);
      const projectData = await deserializeProject(connection, projectPubkey);
      setProject(projectData);
    } catch (err) {
      console.error('Failed to load project:', err);
      setError('Failed to load project details. Please try again.');
    } finally {
      setLoading(false);
    }
  }

  if (loading) return <div>Loading project details...</div>;
  if (error) return <div className="error-message">{error}</div>;
  if (!project) return <div>Project not found</div>;

  return (
    <div className="project-detail">
      <header className="project-header">
        <h1>{project.name}</h1>
        <div className="project-meta">
          <span>Created by: {project.owner.toBase58()}</span>
          <span>Status: {project.status}</span>
        </div>
      </header>

      <section className="project-content">
        <div className="project-info">
          <p>{project.description}</p>
          <ProjectStats project={project} />
        </div>

        <MilestoneProgress 
          milestones={project.milestones}
          currentMilestone={project.currentMilestone}
        />

        {publicKey && (
          <InvestmentForm 
            project={project}
            onInvest={async (amount) => {
              if (onUpdate) {
                await onUpdate({
                  ...project,
                  totalInvestment: project.totalInvestment + amount
                });
              }
            }}
          />
        )}

        {publicKey?.equals(project.owner) && (
          <ProjectActions 
            project={project}
            onUpdate={onUpdate}
          />
        )}
      </section>
    </div>
  );
}; 