import { writable } from "svelte/store";
import type { WalletConnection } from "near-api-js";

export const walletConnection = writable<WalletConnection>();
