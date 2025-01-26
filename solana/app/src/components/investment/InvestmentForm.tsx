import * as React from 'react';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { Project } from '../../types';
import { investInProject } from '../../utils/project';
import { InvestmentCalculator } from './InvestmentCalculator';
import { RiskAssessment } from './RiskAssessment';
import { PaymentService } from '../../services/PaymentService';

interface InvestmentFormProps {
  project: Project;
  onInvest: (amount: number) => Promise<void>;
}

export const InvestmentForm: React.FC<InvestmentFormProps> = ({ project, onInvest }) => {
  const { connection } = useConnection();
  const { publicKey, signTransaction } = useWallet();
  const [amount, setAmount] = React.useState<number>(0);
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);
  const [method, setMethod] = React.useState('SOL');
  const [processing, setProcessing] = React.useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!publicKey || !signTransaction) return;

    try {
      setLoading(true);
      setError(null);
      const investmentAmount = parseFloat(amount.toString());
      
      const transaction = await investInProject(
        connection,
        project.publicKey,
        investmentAmount
      );

      await signTransaction(transaction);
      await connection.sendRawTransaction(transaction.serialize());

      await onInvest(investmentAmount);

      setAmount(0);
    } catch (err) {
      console.error('Investment failed:', err);
      setError('Failed to process investment. Please try again.');
    } finally {
      setLoading(false);
    }
  }

  const handlePaymentSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setProcessing(true);

    try {
      await PaymentService.processPayment({
        amount: parseFloat(amount.toString()),
        method,
        projectId: project.publicKey
      });
    } catch (error) {
      console.error('Investment failed:', error);
    } finally {
      setProcessing(false);
    }
  };

  return (
    <div className="investment-form">
      <form onSubmit={handlePaymentSubmit}>
        <div className="form-group">
          <label htmlFor="amount">Investment Amount (SOL)</label>
          <input
            id="amount"
            type="number"
            value={amount}
            onChange={(e) => setAmount(parseFloat(e.target.value))}
            min="0"
            step="0.1"
            required
            disabled={processing}
          />
        </div>

        <InvestmentCalculator 
          amount={amount}
          project={project}
        />

        <RiskAssessment 
          amount={amount}
          project={project}
        />

        {error && <div className="error-message">{error}</div>}

        <button type="submit" disabled={processing || !publicKey}>
          {processing ? 'Processing...' : 'Invest Now'}
        </button>
      </form>
    </div>
  );
}; 