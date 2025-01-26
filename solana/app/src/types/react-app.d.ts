/// <reference types="react" />
/// <reference types="react-dom" />

declare namespace JSX {
  interface IntrinsicElements {
    [elemName: string]: any;
  }
}

declare module 'react' {
  export = React;
  export as namespace React;

  namespace React {
    type FC<P = {}> = FunctionComponent<P>;
    
    interface FunctionComponent<P = {}> {
      (props: P, context?: any): ReactElement<any, any> | null;
    }

    export function useState<T>(initialState: T | (() => T)): [T, (newState: T | ((prevState: T) => T)) => void];
    export function useEffect(effect: () => void | (() => void | undefined), deps?: ReadonlyArray<any>): void;
    export function useCallback<T extends (...args: any[]) => any>(callback: T, deps: ReadonlyArray<any>): T;
    export function useMemo<T>(factory: () => T, deps: ReadonlyArray<any> | undefined): T;
    export function useRef<T>(initialValue: T): { current: T };
  }
}

declare module '*.svg' {
  const content: any;
  export default content;
}

declare module '*.png' {
  const content: any;
  export default content;
}

declare module '*.jpg' {
  const content: any;
  export default content;
} 