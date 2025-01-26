export interface RiskMetrics {
  overallScore: number;
  factors: RiskFactor[];
  trends: RiskTrendData;
  recommendations: RiskRecommendation[];
}

export interface RiskFactor {
  name: string;
  score: number;
  description: string;
  weight: number;
}

export interface RiskTrendData {
  labels: string[];
  datasets: {
    label: string;
    data: number[];
    borderColor: string;
    fill: boolean;
  }[];
}

export interface RiskRecommendation {
  id: string;
  title: string;
  description: string;
  priority: RiskPriority;
  impact: number;
}

export enum RiskPriority {
  Low = 'LOW',
  Medium = 'MEDIUM',
  High = 'HIGH',
  Critical = 'CRITICAL'
} 