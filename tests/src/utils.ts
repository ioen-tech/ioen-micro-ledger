import { Config, InstallAgentsHapps } from '@holochain/tryorama';
import path from 'path'
import { fileURLToPath } from "url";
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const ioenMicroLedgerDna = path.join(__dirname, "../../dna/workdir/ioen_micro_ledger.dna");


export const config = Config.gen();

export const installation: InstallAgentsHapps = [
  // one agent
  [
    [
      ioenMicroLedgerDna, // contains this dna
    ]
  ]
];

export const sleep = (ms: number) => new Promise(resolve => setTimeout(() => resolve(null), ms));
