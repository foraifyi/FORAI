import 'react';

declare module 'react' {
  // Add missing hooks types
  export function useState<T>(initialState: T | (() => T)): [T, Dispatch<SetStateAction<T>>];
  export function useEffect(effect: EffectCallback, deps?: DependencyList): void;
  export function useContext<T>(context: Context<T>): T;
  export function useReducer<R extends Reducer<any, any>>(
    reducer: R,
    initialState: ReducerState<R>,
    initializer?: (arg: ReducerState<R>) => ReducerState<R>,
  ): [ReducerState<R>, Dispatch<ReducerAction<R>>];
  export function useCallback<T extends (...args: any[]) => any>(
    callback: T,
    deps: DependencyList,
  ): T;
  export function useMemo<T>(factory: () => T, deps: DependencyList | undefined): T;
  export function useRef<T>(initialValue: T): MutableRefObject<T>;
  export function useRef<T>(initialValue: T | null): RefObject<T>;
  export function useLayoutEffect(effect: EffectCallback, deps?: DependencyList): void;
} 