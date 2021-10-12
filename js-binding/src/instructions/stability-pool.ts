import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  PublicKey,
  SYSVAR_CLOCK_PUBKEY,
  TransactionInstruction,
} from "@solana/web3.js";
import BN from "bn.js";
import { deserializeUnchecked, Schema, serialize } from "borsh";
import { Numberu64 } from "../utils";

export class initializeInstruction {
  tag: number;
  nonce: number;
  static schema: Schema = new Map([
    [
      initializeInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["nonce", "u8"],
        ],
      },
    ],
  ]);

  constructor(obj: {
    nonce: number;
  }) {
    this.tag = 0;
    this.nonce = obj.nonce;
  }

  serialize(): Uint8Array {
    return serialize(initializeInstruction.schema, this);
  }

  getInstruction(
    programId: PublicKey,
    stabilityPoolId: PublicKey,
    authority: PublicKey,
    solusdPoolTokenAccount: PublicKey,
    community_issuance_account: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys = [
      {
        pubkey: stabilityPoolId,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: authority,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: solusdPoolTokenAccount,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: community_issuance_account,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: true,
      },
    ];

    return new TransactionInstruction({
      keys,
      programId: programId,
      data,
    });
  }
}

export type StabilityPoolInstruction =
  | initializeInstruction;

export function parseInstructionData(buffer: Buffer): StabilityPoolInstruction {
  let types = [
    initializeInstruction,
  ];
  let t = types[buffer[0]];
  return deserializeUnchecked(t.schema, t, buffer);
}
