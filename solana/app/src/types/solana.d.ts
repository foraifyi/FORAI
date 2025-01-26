declare module '@solana/web3.js' {
  export class Connection {
    constructor(endpoint: string, commitment?: string);
    getAccountInfo(publicKey: PublicKey): Promise<AccountInfo<Buffer> | null>;
    // ... other methods
  }

  export class PublicKey {
    constructor(value: string);
    equals(other: PublicKey): boolean;
    toBase58(): string;
    // ... other methods
  }

  export class Transaction {
    constructor(options?: {
      feePayer?: PublicKey;
      recentBlockhash?: string;
      nonceInfo?: NonceInformation;
    });
    add(...instructions: TransactionInstruction[]): Transaction;
    sign(...signers: Account[]): void;
    serialize(): Buffer;
  }

  export class Account {
    constructor();
    publicKey: PublicKey;
    secretKey: Uint8Array;
    sign(transaction: Transaction): void;
  }

  export interface AccountInfo<T> {
    executable: boolean;
    owner: PublicKey;
    lamports: number;
    data: T;
    rentEpoch?: number;
  }

  export type TransactionInstruction = {
    keys: Array<{
      pubkey: PublicKey;
      isSigner: boolean;
      isWritable: boolean;
    }>;
    programId: PublicKey;
    data: Buffer;
  };

  export interface NonceInformation {
    nonce: string;
    nonceInstruction: TransactionInstruction;
  }
}

declare module '@solana/wallet-adapter-react' {
  export function useConnection(): { connection: Connection };
  export function useWallet(): {
    publicKey: PublicKey | null;
    connected: boolean;
    connecting: boolean;
    disconnect(): Promise<void>;
    connect(): Promise<void>;
    // ... other properties and methods
  };
} 