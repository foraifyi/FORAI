declare module 'chart.js' {
  export * from '@types/chart.js';
}

declare module 'react-chartjs-2' {
  import { ChartData, ChartOptions } from 'chart.js';
  import * as React from 'react';

  export interface ChartComponentProps {
    data: ChartData;
    options?: ChartOptions;
    type?: string;
    height?: number;
    width?: number;
    redraw?: boolean;
    datasetKeyProvider?: (any: any) => any;
  }

  export class Line extends React.Component<ChartComponentProps> {}
  export class Bar extends React.Component<ChartComponentProps> {}
  export class Pie extends React.Component<ChartComponentProps> {}
  export class Doughnut extends React.Component<ChartComponentProps> {}
  export class Chart extends React.Component<ChartComponentProps> {}
} 